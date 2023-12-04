use std::sync::Arc;

use super::sirmodel::SIRModel;

use super::bayesian::{BayesianOptimization,GaussianProcess};

use rand::{thread_rng,Rng};

#[derive(PartialEq)]
pub enum TrainModel {
    Simple,
    Bayesian
}

pub struct Trainer {
    baseModel: Arc<std::sync::Mutex<SIRModel>>,
    numModels: usize,
    numEpochs: usize,
    startingData: Vec<Vec<usize>>,
    guesses: Vec<f64>,
    trainingmodel: TrainModel,
    learningrate: f64,
    expPopSize: usize,
    simPopSize: usize
}

impl Trainer {

    pub fn new(baseModel: Arc<std::sync::Mutex<SIRModel>>,
        numModels: usize,
        numEpochs: usize,
        startingData: Vec<Vec<usize>>,
        guesses: Vec<f64>,
        trainingmodel: TrainModel,
        learningrate: f64,
        expPopSize: usize,
        simPopSize: usize) -> Trainer {
            Trainer {baseModel, numModels, numEpochs, startingData, guesses, trainingmodel, learningrate, expPopSize, simPopSize}
    }

    pub async fn train(&mut self,expectedPopSize: usize) ->  f64 {
        let mut rng = rand::thread_rng();
        
            
        if self.trainingmodel == TrainModel::Simple {
            for ep in 0..self.numEpochs {
                let mut guess = 0.0;
                if ep > 0 {
                    guess = self.guesses[ep - 1];
                } else {
                    guess = rng.gen::<f64>();
                }
                let mut errorsSum = 0.0;
                for i in 0..self.numModels {
                    let mut baseSimul = self.baseModel.lock().unwrap();
                    baseSimul.clearOut(self.startingData[0][0]);
                    baseSimul.setSpread(guess);
                    baseSimul.setDays(40);
                    baseSimul.runSim().await;
                    let infected = baseSimul.propInfected();
                    //println!("{:?}", infected);
                    let currentError = Self::error(self.startingData[0].clone(),infected, expectedPopSize);
                    errorsSum = errorsSum + currentError;
                }   
                let errs = errorsSum / (self.numModels as f64);
                
                let newGuess = guess + self.learningrate * errs;
                self.guesses.push(newGuess);
            }
            return *self.guesses.last().unwrap();
        } else if self.trainingmodel == TrainModel::Bayesian {
            // Define the mean function and kernel for the Gaussian Process
            let mean_function = Box::new(|x: f64| 0.0);
            let kernel_function = Box::new(|x1: f64, x2: f64| (-0.5 * (x1 - x2).powi(2)).exp());

            // Create a Gaussian Process for Bayesian Optimization
            let gp = GaussianProcess::new(mean_function, kernel_function);

            // Define the Bayesian Optimization instance with a kappa value (exploration-exploitation trade-off)
            let mut bo = BayesianOptimization::new(gp, 1.0,self.baseModel.clone(), self.expPopSize, self.simPopSize);

            // Perform Bayesian Optimization with 10 iterations
            let best_point = bo.optimize(self.numEpochs, self.startingData.clone()).await;

            println!("Best Point: {}", best_point);
            return best_point;
        } else {
            return 0.0;
        }
    }

    fn error(expected: Vec<usize>, simulated: Vec<f64>, expectedPopSize: usize) -> f64 {
        let mut errsum = 0.0;
        for n in 0..simulated.len() {
            errsum = errsum as f64 + (expected[n] as f64 )/(expectedPopSize as f64)- simulated[n] as f64;
        }
        errsum / (expected.len() as f64)
    }

    
}