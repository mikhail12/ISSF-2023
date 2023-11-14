use super::matrix::Matrix;
use rand::{thread_rng,Rng};

#[derive(Clone)]
pub struct Person {
    state: Personstate,
    spreadChance: f64,
    posx: f32,
    posy: f32,
    velx: f32,
    vely: f32,
    daysInfected:usize,
    is_from: Locations,
    chance_to_city: f64,
    in_city: bool
}

impl Person {
    pub fn random(state: Personstate, spreadMinMax: Matrix, mut spawnLoc: Matrix, mut velocityMinMax: Matrix) -> Person {
        let mut spreadChance = Matrix::random(2,1).multiply(&spreadMinMax).data[0][0];
        if spreadChance > 1.0 {
            spreadChance = 1.0;
        } else if spreadChance < 0.0 {
            spreadChance = 0.0;
        }

        let mut rand = thread_rng();
        let mut loc = Locations::City;
        let randNum = rand.gen::<f64>();
        let num = 0.4;
        let num2 = 0.6;
        if randNum > 1.0 - num {
            loc = Locations::City;
        } else if 0.0 < randNum && randNum < num * 0.25 {
            loc = Locations::NE;
        } else if num * 0.25 < randNum && randNum < num * 0.5 {
             loc = Locations::SE;
        } else if num * 0.5 < randNum && randNum < num * 0.75 {
            loc = Locations::SE;
        } else if num * 0.75 < randNum && randNum < num {
            loc = Locations::SE;
        }


        Person {state, spreadChance, posx: (rand.gen::<f32>() * (spawnLoc.get(0,0) as f32)), posy: (rand.gen::<f32>() * (spawnLoc.get(0,1) as f32)), velx: (rand.gen::<f32>() * (velocityMinMax.get(0,0) as f32)), vely: (rand.gen::<f32>() * (velocityMinMax.get(0,1) as f32)), daysInfected: 0,is_from: loc, chance_to_city: randNum * num2 ,in_city: true}
    }

    pub fn changeState(&mut self, day:usize, state: Personstate) {
        self.state = state;
    }

    pub fn infectCheck(&mut self) -> bool {
        self.state == Personstate::Inf
    }

    pub fn susCheck(&mut self) -> bool {
        self.state == Personstate::Sus
    }

    pub fn getSpreadChance(&mut self) -> f64 {
        self.spreadChance
    }


    pub fn getState(&mut self) -> Personstate {
        self.state
    }

    pub fn getInCity(&mut self) -> bool {
        self.in_city
    }

    pub fn getDaysInfected(&mut self) -> usize {
        self.daysInfected
    }

    pub fn addToDaysInfected(&mut self) {
        self.daysInfected += 1;
    }

    pub fn infect(&mut self) -> bool {
        let mut rand = thread_rng();
        if rand.gen::<f64>() *2.0 - 1.0 < self.spreadChance{
            false
        } else {
            true 
        }
    }

    pub fn getPosVel(&mut self) -> [f32;4] {
        [self.posx,self.posy, self.velx,self.vely]
    }

    //deprecated movement system

}

#[derive(Clone,Copy,PartialEq)]
pub enum Personstate {
    Sus,
    Inf,
    Rem
}

#[derive(Clone,Copy,PartialEq)]
pub enum Locations {
    City,
    NE,
    SE,
    SW,
    NW
}
