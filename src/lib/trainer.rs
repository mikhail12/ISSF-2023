use std::sync::Arc;

use super::sirmodel::SIRModel;

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
    learningrate: f64
}

impl Trainer {

    pub fn new(baseModel: Arc<std::sync::Mutex<SIRModel>>,
        numModels: usize,
        numEpochs: usize,
        startingData: Vec<Vec<usize>>,
        guesses: Vec<f64>,
        trainingmodel: TrainModel,
        learningrate: f64) -> Trainer {
            Trainer {baseModel, numModels, numEpochs, startingData, guesses, trainingmodel, learningrate}
    }

    pub async fn train(&mut self) ->  f64 {
        let mut rng = rand::thread_rng();
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
                baseSimul.clearOut();
                baseSimul.setSpread(guess);
                baseSimul.setDays(self.startingData.len());
                baseSimul.runSim().await;
                let infected = baseSimul.numInfected();
                let currentError = Self::error(self.startingData[0].clone(),infected);
                errorsSum = errorsSum + currentError;
            }   
            let errs = errorsSum / (self.numModels as f64);
            
            if self.trainingmodel == TrainModel::Simple {
                let newGuess = guess + self.learningrate * errs;
                self.guesses.push(newGuess);
            } else if self.trainingmodel == TrainModel::Bayesian {

            }
        }
        
        *self.guesses.last().unwrap()
    }

    fn error(expected: Vec<usize>, simulated: Vec<usize>) -> f64 {
        let mut errsum = 0.0;
        for n in 0..expected.len() {
            errsum = errsum as f64 + expected[n] as f64 - simulated[n] as f64;
        }
        errsum / (expected.len() as f64)
    }

    
}