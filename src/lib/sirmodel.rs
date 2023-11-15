


use tokio::task::block_in_place;

use crate::lib::intervention::InterventionType;

use super::{matrix::Matrix, person::{Person, Personstate}, wgpuInit::WgpuInit, intervention::Intervention};
use rand::{thread_rng,Rng};



pub struct SIRModel {
    population: Vec<Vec<Person>>,
    populationposvel: Vec<[Vec<f32>;4]>,
    populationinf: Vec<Vec<u32>>,
    popsize: usize,
    spreadMinMax: Matrix,
    spawnLoc: Matrix,
    infRad: f32,
    infectiousPeriod: usize,
    daysRun: usize,
    wgpuinit: WgpuInit,
    simulated: bool,
    spreadRan: f64,
    interventions: Vec<Intervention>
}

//spreadran is actually the spread rate

impl SIRModel {

    pub fn setSpread(&mut self, spreadRate: f64) {
        self.spreadMinMax = Matrix::from(vec![vec![spreadRate-(self.spreadRan/2.0),spreadRate+(self.spreadRan/2.0)]]);
        
    }

    pub fn clearOut(&mut self) {
        let mut res = SIRModel::emptyTZero(self.popsize, self.daysRun, self.wgpuinit);
        for i in 0..(self.popsize-self.getNumInfected(0)) {
            res.population[0].push(Person::random(Personstate::Sus,self.spreadMinMax.clone(), self.spawnLoc.clone(), self.velocityMinMax.clone()));
        }
        for i in 0..self.getNumInfected(0) {
            res.population[0].push(Person::random(Personstate::Inf,self.spreadMinMax.clone(), self.spawnLoc.clone(), self.velocityMinMax.clone()));
    
        }

        for i in 0..self.popsize {
            res.populationposvel[0][0].push(res.population[0][i].getPosVel()[0]);
            res.populationposvel[0][1].push(res.population[0][i].getPosVel()[1]);
            res.populationposvel[0][2].push(res.population[0][i].getPosVel()[2]);
            res.populationposvel[0][3].push(res.population[0][i].getPosVel()[3]);
            res.populationinf[0].push(boolToU32(res.population[0][i].infectCheck()))
        }

        res.popsize = self.popsize;
        res.spreadMinMax = self.spreadMinMax;
        res.spreadRan = self.spreadRan;
        println!("x: {:?}, y: {:?}",self.spawnLoc.clone().data[0][0],self.spawnLoc.clone().data[0][1]);
        res.spawnLoc = self.spawnLoc;
        res.infRad = self.infRad;
        res.infectiousPeriod = self.infectiousPeriod;
        res.interventions = self.interventions;
        self = &mut res;
    }

    pub fn emptyTZero(popsize: usize, days: usize, wgpuinit: WgpuInit)-> SIRModel {
        SIRModel {
            population: vec![Vec::new();days],
            populationposvel: vec![[Vec::new(),Vec::new(),Vec::new(),Vec::new()];days],
            populationinf: vec![Vec::new();days],
            popsize: 1,
            spreadMinMax: Matrix::zeros(1, 2),
            spawnLoc: Matrix::zeros(1, 2),
            infRad:0.0,
            infectiousPeriod:7,
            daysRun: days,
            wgpuinit: wgpuinit,
            simulated: false,
            spreadRan: 0.0,
            interventions: Vec::new()
        }
    }

    pub fn new(popsize: usize, infRad:f32, infectiousPeriod:usize, startInfNum:usize, spreadRate: f64, spreadRan: f64, spawn_x: f64, spawn_y: f64, minVelocity: f64, maxVelocity: f64, daysRun: usize, wgpuinit: WgpuInit, interventions: Vec<Intervention>) -> SIRModel {
        let mut spawnLoc = Matrix::from(vec![vec![spawn_x*0.5,spawn_y*0.5]]);
        let mut spreadMinMax = Matrix::from(vec![vec![spreadRate-(spreadRan/2.0),spreadRate+(spreadRan/2.0)]]);
        let mut velocityMinMax = Matrix::from(vec![vec![minVelocity,maxVelocity]]);
        let mut res = SIRModel::emptyTZero(popsize, daysRun, wgpuinit);
        for i in 0..(popsize-startInfNum) {
            res.population[0].push(Person::random(Personstate::Sus,spreadMinMax.clone(), spawnLoc.clone(), velocityMinMax.clone()));
        }
        for i in 0..startInfNum {
            res.population[0].push(Person::random(Personstate::Inf,spreadMinMax.clone(), spawnLoc.clone(), velocityMinMax.clone()));
    
        }

        for i in 0..popsize {
            res.populationposvel[0][0].push(res.population[0][i].getPosVel()[0]);
            res.populationposvel[0][1].push(res.population[0][i].getPosVel()[1]);
            res.populationposvel[0][2].push(res.population[0][i].getPosVel()[2]);
            res.populationposvel[0][3].push(res.population[0][i].getPosVel()[3]);
            res.populationinf[0].push(boolToU32(res.population[0][i].infectCheck()))
        }

        res.popsize = popsize;
        res.spreadMinMax = spreadMinMax;
        res.spreadRan = spreadRate;
        println!("x: {:?}, y: {:?}",spawnLoc.clone().data[0][0],spawnLoc.clone().data[0][1]);
        res.spawnLoc = spawnLoc;
        res.infRad = infRad;
        res.infectiousPeriod = infectiousPeriod;
        res.interventions = interventions;
        res
    }

    pub async fn runSim(&mut self) {
        println!("This code actually executed");
        for i in 0..self.daysRun {
            self.timestep(i).await
        }
        println!("This finished");
        self.simulated = true;
    }

    pub fn numInfected(&mut self) -> Vec<usize> {
        let mut c = vec![0];
        for i in 0..self.daysRun {
            for j in 0..self.populationinf[i].len() {
                if self.populationinf[i][j] > 0 {
                    c[i] = c[i] + 1;
                }
            }
        }
        c
    }

    
    pub fn print_matrix(&mut self,days: usize) {
        for day in 0..days {
            println!("Day: {}, Infected: {}, Sus: {}, Rem: {}",day,self.getNumInfected(day),self.getNumSusceptible(day),self.getNumRemoved(day));
        }
    }

    pub fn print_matrix_all_days(&mut self) {
        for day in 0..self.daysRun {
            println!("Day: {}, Infected: {}, Sus: {}, Rem: {}",day,self.getNumInfected(day),self.getNumSusceptible(day),self.getNumRemoved(day));
        }
    }

    pub fn exportMatrixAllDays(&mut self) -> Vec<Vec<usize>> {
        let mut res = vec![vec![]];
        for day in 0..self.daysRun {
            let mut tempDay =vec![self.getNumInfected(day),self.getNumSusceptible(day),self.getNumRemoved(day)];
            res.push(tempDay);
        }
        res
    }

    pub fn getPopSize(&mut self) -> usize {
        self.popsize
    }

    pub fn getNumSusceptible(&mut self, day: usize) -> usize {
        let mut res = 0;
        for mut per in self.population[day].clone() {
            if per.getState() == Personstate::Sus {
                res +=1;
            }
        }
        res
    }

    pub fn getNumRemoved(&mut self, day: usize) -> usize {
        let mut res = 0;
        for mut per in self.population[day].clone() {
            if per.getState() == Personstate::Rem {
                res +=1;
            }
        }
        res
    }


    pub fn getNumInfected(&mut self, day: usize ) -> usize {
        let mut res = 0;
        for mut per in self.population[day].clone() {
            if per.getState() == Personstate::Inf {
                res +=1;
            }
        }
        res
    }


    

    pub async fn timestep(&mut self, time: usize) {
        let mut rng = rand::thread_rng();
        if time > 0 {
            for mut int in self.interventions.clone() {
                if time == int.getStart() {
                    match int.getType() {
                        InterventionType::Kkkkkzone => {
                            let mut resula = Vec::new();
                            let mut resulb = Vec::new();
                            for p in self.populationposvel[time-1][2].clone() {
                                resula.push(p*0.05);
                            }
                            for p in self.populationposvel[time-1][3].clone() {
                                resulb.push(p*0.05);
                            }
                            self.populationposvel[time-1][2] = resula;
                            self.populationposvel[time-1][3] = resulb;
                        },
                        InterventionType::Mask => {
                            self.spreadRan = self.spreadRan * 0.4;
                        }
                    }
                    
                } else if time == int.getEnd() {
                    match int.getType() {
                        InterventionType::Kkkkkzone => {
                            let mut resula = Vec::new();
                            let mut resulb = Vec::new();
                            for p in self.populationposvel[time-1][2].clone() {
                                resula.push(p*20.0);
                            }
                            for p in self.populationposvel[time-1][3].clone() {
                                resulb.push(p*20.0);
                            }
                            self.populationposvel[time-1][2] = resula;
                            self.populationposvel[time-1][3] = resulb;
                        },
                        InterventionType::Mask => {
                            self.spreadRan = self.spreadRan / 0.4;
                        }
                    }
                }
            }
            self.populationposvel[time] = self.wgpuinit.moveCol(self.populationposvel[time-1][0].clone(), self.populationposvel[time-1][1].clone(), self.populationposvel[time-1][2].clone(), self.populationposvel[time-1][3].clone(), [self.spawnLoc.get(0, 0) as f32,self.spawnLoc.get(0, 1)as f32] ).await;
            self.populationinf[time] = self.wgpuinit.checkInf(self.populationposvel[time][0].clone(), self.populationposvel[time][1].clone(), self.populationinf[time-1].clone(), self.infRad*self.infRad).await;
            for p in 0..self.populationinf.len() {
                if self.populationinf[time-1][p] == 0 {
                    if rng.gen::<f64>() > self.spreadRan {
                        self.populationinf[time][p] = 0;
                    }
                }
            }
        }
        println!("timestep: {:?}", time)
    }

    pub fn newFrame(&mut self, time: usize) {
        if self.simulated {
            self.wgpuinit.newFrame(self.populationposvel[time][0].clone(), self.populationposvel[time][1].clone(), self.populationinf[time].clone());
        }
    }

    //pub fn oldtimestep(&mut self, time: usize) {
        
            //for i in 0..self.popsize {
            //    if self.population[time][i].infectCheck() {
            //        print!("check; ");
            //        for j in 0..(self.popsize-1) {
            //            if self.infRad > self.population[time][i].getPos().data[0][1] - self.population[time][j].getPos().data[0][1] {
            //                if 0.0 - self.infRad < self.population[time][i].getPos().data[0][1] - self.population[time][j].getPos().data[0][1] {
            //                    
            //                    if self.infRad > self.population[time][i].getPos().data[0][0] - self.population[time][j].getPos().data[0][0] {
            //                        
            //                        if 0.0 - self.infRad < self.population[time][i].getPos().data[0][1] - self.population[time][j].getPos().data[0][1] {
            //                            
            //                            if Self::findDistance(self.population[time][i].getPos().clone(), &self.population[time][j].getPos()) < self.infRad { 
                                            //print!("check a; ");
            //                                if self.population[time][j].infect() {
            //                                    self.population[time][j].changeState(time, Personstate::Inf);
            //                                }
            //                            }
            //                        }
            //                    }
            //                }
            //            }
            //        }
            //    }
            //    
            //}

            //for i in 0..self.popsize {
            //    if self.population[time][i].getInCity() {
            //        if self.population[time][i].infectCheck() {
            //            for j in 0..(self.popsize-1) {
            //                if self.population[time][j].getInCity() {
            //                    if self.population[time][j].susCheck() {
            //                        if self.infRad > self.population[time][i].getPos().data[0][1] - self.population[time][j].getPos().data[0][1] || 0.0 - self.infRad < self.population[time][i].getPos().data[0][1] - self.population[time][j].getPos().data[0][1] || self.infRad > self.population[time][i].getPos().data[0][0] - self.population[time][j].getPos().data[0][0] || 0.0 - self.infRad < self.population[time][i].getPos().data[0][0] - self.population[time][j].getPos().data[0][0]{
            //                            if Self::findDistance(self.population[time][i].getPos().clone(), &self.population[time][j].getPos()) < self.infRad { 
            //                                //print!("check a; ");
            //                                if self.population[time][j].infect() {
            //                                    self.population[time][j].changeState(time, Personstate::Inf);
            //                                }
            //                            }
             //                       }
            //                    }
            //                }
            //            }
            //        }
            //    }
            //    
    //        }
    //    }
//    }

    
    

    fn findDistance(pos1: Matrix, pos2: &Matrix) -> f64 {
        let mut res: f64 = f64::powi(pos1.data[0][0]-pos2.data[0][0],2) + f64::powi(pos1.data[0][1]-pos2.data[0][1],2);
        res.sqrt()
    }
}


fn boolToU32(data: bool) -> u32 {
    let mut res: u32 = 0;
    if data {
        res = 1;
    } else {
        res = 0;
    }
    res
}