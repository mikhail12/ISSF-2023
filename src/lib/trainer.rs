
pub enum TrainModel {
    Simple,
    Bayesian
}

pub struct Trainer {
    baseModel: SIRModel,
    numModels: usize,
    numEpochs: usize,
    startingData: Vec<usize>,
    models: Vec<Vec<SIRModel>>,
    errors: Vec<f64>,
    guesses: Vec<f64>,
    controlledvalues: Vec<{}>,
    trainingmodel: TrainModel,
    learningrate: f64
}

impl Trainer {

    pub async fn train(&mut self) ->  f64 {
        let mut rng = rand::thread_rng();
        for ep in 0..self.numEpochs {
            let guess = 0.0;
            if ep > 0 {
                guess = guesses[epoch - 1];
            } else {
                guess = rng.gen();
            }
            let errorsSum = 0.0;
            for i in 0..self.numModels {
                let current = self.baseModel;
                current.setSpread(guess);
                current.clearOut();
                current.runSim().await;
                let infected = current.numInfected();
                let currentError = error(self.startingData[0],infected);
                errorsSum = errorsSum + currentError;
            }   
            let errs = errorsSum / self.numModels;
            
            if self.trainingmodel == TrainModel::Simple {
                let newGuess = guess + self.learningrate * errs;
                guesses[ep] = newGuess;
            }
        }
        
        guesses.last()
    }

    fn error(expected: Vec<usize>, simulated: Vec<usize>) -> f64 {
        let errsum = 0.0;
        for n in 0..expected.len() {
            errsum = errsum + expected - simulated;
        }
        errsum / (expected.len())
    }
}