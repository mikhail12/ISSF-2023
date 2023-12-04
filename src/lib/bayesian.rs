
use std::sync::Arc;

use nalgebra::{DVector, Dyn, DMatrix, Cholesky};
use special::Error;

use super::sirmodel::SIRModel;

// Define a Gaussian Process struct
pub struct GaussianProcess {
    // Mean function
    mean: Box<dyn Fn(f64) -> f64>,

    // Covariance function (kernel)
    kernel: Box<dyn Fn(f64, f64) -> f64>,

    // Training data
    x_train: Vec<f64>,
    y_train: Vec<f64>,

    // Cholesky decomposition of the covariance matrix
    chol_decomp: Option<Cholesky<f64, Dyn>>,
}

impl GaussianProcess {
    // Constructor function to create a new Gaussian Process
    pub fn new(mean: Box<dyn Fn(f64) -> f64>, kernel: Box<dyn Fn(f64, f64) -> f64>) -> Self {
        GaussianProcess {
            mean,
            kernel,
            x_train: Vec::new(),
            y_train: Vec::new(),
            chol_decomp: None,
        }
    }

    // Add training data to the GP
    pub fn add_data(&mut self, x: f64, y: f64) {
        let mut actualX = x;
        let mut actualY = y;
        if actualX < 0.0 {
            actualX = -actualX;
        }

        //if actualY < 0.0 {
        //    actualY = -actualY;
        //}
        self.x_train.push(actualX);
        self.y_train.push(actualY);
        println!("Added data x: {:?}",self.x_train);
        println!("Added data y: {:?}",self.y_train);
        
    }

    // Update the GP with training data
    pub fn fit(&mut self) {
        // Check if there is no training data
        if self.x_train.is_empty() || self.y_train.is_empty() {
            panic!("Cannot fit without training data.");
        }
        println!("This runs");

        // Calculate the covariance matrix of the training data
        let cov_matrix = DMatrix::from_fn(self.x_train.len(), self.x_train.len(), |i, j| {
            (self.kernel)(self.x_train[i], self.x_train[j])
        });

        // Add a small diagonal jitter to improve numerical stability during Cholesky decomposition
        let jitter = 1e-6;
        let cov_matrix_jittered = cov_matrix + DMatrix::identity(self.x_train.len(), self.x_train.len()) * jitter;

        // Perform Cholesky decomposition
        match Cholesky::new(cov_matrix_jittered) {
            Some(chol_decomp) => {
                self.chol_decomp = Some(chol_decomp);
            }
            None => {
                panic!("Cholesky decomposition failed. The covariance matrix may not be positive definite.");
            }
        }
    }

    pub fn checkEmpty(&mut self) -> bool {
        if self.x_train.is_empty() || self.y_train.is_empty() {
            return true;
        }
        false
    }

    // Predict mean and variance at a given point
    pub fn predict(&mut self, x: f64) -> (f64, f64) {


        // Check if there is no training data
        if self.x_train.is_empty() || self.y_train.is_empty() {
            panic!("Cannot predict without training data.");
        }
        println!(";bdaskfjnbfn");
        println!("{:?}",self.x_train);
        println!("{:?}", self.y_train);

        // Fit the GP if it hasn't been fitted yet
        if self.x_train.len() != self.y_train.len() {
            println!("fitting");
            self.fit();
        }
        self.fit();

        // Calculate the mean prediction
        let mean = (self.mean)(x);

        // Calculate the covariance matrix between the training data and the test point
        let cov_matrix = DMatrix::from_fn(self.x_train.len(), 1, |i, _| {
            (self.kernel)(self.x_train[i], x)
        });

        // Calculate the covariance matrix for the test point
        let k_xx =  (self.kernel)(x, x);


        // Calculate the predictive variance
        let variance_matrix = (cov_matrix.transpose() * self.inverse_covariance_matrix() * cov_matrix).map(|value| k_xx - value);
        let variance = variance_matrix.sum();
        (mean, variance)
    }

    // Helper function to compute the inverse of the covariance matrix
    pub fn inverse_covariance_matrix(&self) -> DMatrix<f64> {
        // Check if Cholesky decomposition has been computed
        let chol_decomp = match &self.chol_decomp {
            Some(chol) => chol,
            None => panic!("Cholesky decomposition not available. Fit the GP first."),
        };

        // Solve the linear system using the Cholesky decomposition
        let identity_matrix = DMatrix::identity(self.x_train.len(), self.x_train.len());
        let inverse_cov_matrix = chol_decomp.solve(&identity_matrix);

        inverse_cov_matrix
    }
}

// Define a Bayesian Optimization struct
pub struct BayesianOptimization {
    gp: GaussianProcess,
    kappa: f64, // Exploration-exploitation trade-off parameter
    baseModel: Arc<std::sync::Mutex<SIRModel>>,
    expPopSize: usize,
    simPopSize: usize,
}

impl BayesianOptimization {
    // Create a new Bayesian Optimization instance
    pub fn new(gp: GaussianProcess, kappa:f64, baseModel: Arc<std::sync::Mutex<SIRModel>>,expPopSize: usize, simPopSize: usize) -> Self {
        BayesianOptimization { gp, kappa, baseModel, expPopSize, simPopSize}
    }

    // Optimize the objective function
    pub async fn optimize(&mut self, num_iterations: usize, startingData: Vec<Vec<usize>>) -> f64 {
        
        let mut points = Vec::new();
        for _ in 1..num_iterations {
            println!("Does this run?");
            
            if self.gp.checkEmpty() {
                let point = 0.01;
                
                let mut errorsSum = 0.0;
                for i in 0..20 {
                    let mut baseSimul = self.baseModel.lock().unwrap();
                    println!("checking stuff {:?}",startingData[0].len() - 1);
                    baseSimul.clearOut(startingData[0][0]);
                    baseSimul.setSpread(point);
                    baseSimul.setDays(40);
                    baseSimul.runSim().await;
                    let infected = baseSimul.propInfected();
                    println!("Infected {:?}", infected);
                    let currentError = Self::error(startingData[0].clone(),infected, self.expPopSize);
                    errorsSum = errorsSum + currentError;
                }   
                let mut objective_value = (errorsSum / (100 as f64));
                if objective_value < 0.0 {
                    objective_value = -objective_value;
                }

                //println!("{:?}", infected);
                //let objective_value = Self::error(startingData[0].clone(),infected);
                //println!("{:?}", objective_value);
                self.gp.add_data(point,objective_value);
                //println!("this rasnds");
            }
            
            // Select the next point to evaluate based on the acquisition function
            let next_point = self.select_next_point();

            let mut point = next_point;
            println!("Point chosen: {:?}", point);
            // Evaluate the objective function at the selected point
            if next_point < 0.0 {
                point = -point;
            }

            if next_point > 1.0 {
                point = 1.0 / next_point
            }
            println!("checking stuff {:?}",point);
            points.push(point);
                    
            let mut errorsSum = 0.0;
            for i in 0..10 {
                let mut baseSimul = self.baseModel.lock().unwrap();
                baseSimul.clearOut(startingData[0][0]);
                baseSimul.setSpread(point);
                baseSimul.setDays(40);
                baseSimul.runSim().await;
                let infected = baseSimul.propInfected();
                println!("Infected {:?}", infected);
                let currentError = Self::error(startingData[0].clone(),infected, self.expPopSize);
                errorsSum = errorsSum + currentError;
            }   
            let objective_value = (errorsSum / (10 as f64));


            /*
            let mut baseSimul = self.baseModel.lock().unwrap();
            baseSimul.clearOut(startingData[0][0]);
            baseSimul.setSpread(next_point);
            baseSimul.setDays(startingData[0].len());
            baseSimul.runSim().await;
            let infected = baseSimul.numInfected();
            let objective_value = Self::error(startingData[0].clone(),infected);
            println!("{:?}",objective_value);
            println!("Above isobjective value");
            // Update the Gaussian Process with the new observation

             */
            self.gp.add_data(next_point, objective_value);
            self.gp.fit();
        }

        println!("points: {:?}", points);
        // Return the point with the best observed value
        self.select_next_point()
    }

    // Expected Improvement (EI) acquisition function
    pub fn expected_improvement(&mut self, x: f64) -> f64 {
        
        
        // Get the mean and variance predictions from the Gaussian Process
        let (mean, variance) = self.gp.predict(x);

        // Calculate the standard deviation (avoiding negative variance)
        let std_dev = variance.max(0.0).sqrt();

        // Calculate the improvement over the current best observed value
        let mut f_max = 0.0;
        for y in self.gp.y_train.clone().into_iter() {
            if y > f_max {
                f_max = y;
            }
        }
        let mut f_min = f_max;
        for y in self.gp.y_train.clone().into_iter() {
            if y < f_min {
                f_min = y;
            }
        }

        //println!("{:?}",self.gp.y_train.clone());


        //let f_min = self.gp.y_train.min();
        let improvement = mean - f_min;

        // Calculate the Z-score
        let z_score = improvement / std_dev;

        // Calculate the Expected Improvement
        let ei = std_dev * (z_score * 0.5 * (1.0 + z_score.error())) + (-z_score * 0.5 * (1.0 - z_score.error()).exp());

        ei
    }

    // Select the next point to evaluate based on the acquisition function (EI)
    pub fn select_next_point(&mut self) -> f64 {
        // Define a search space (replace with your actual search space)
        let lower_bound = 0.0;
        let upper_bound = 1.0;

        // Use a numerical optimization method to find the point that maximizes EI
        let mut max_point = lower_bound;
        let mut max_ei = f64::NEG_INFINITY;

        // Choose a set of candidate points for optimization (e.g., using a grid search)
        let num_candidates = 100;

        let mut candidate_points = vec![lower_bound];
        let spacing = (upper_bound - lower_bound) / ((num_candidates - 1 )as f64);

        for i in 1..(num_candidates - 1) {
            candidate_points.push(lower_bound + spacing * (i as f64));
        }

        

        for candidate in candidate_points {
            let ei = self.expected_improvement(candidate);
            //println!("Expected Improvement: {:?}", ei);
            if ei > max_ei {
                max_ei = ei;
                max_point = candidate;
            }
        }
        //println!("Does this run?");

        if max_point < 0.0 {
            max_point = - max_point;
        }

        max_point
    }

    

    fn error(expected: Vec<usize>, simulated: Vec<f64>, expPopSize: usize) -> f64 {
        let mut errsum = 0.0;
        for n in 0..(simulated.len() - 1) {
            errsum = errsum as f64 + (expected[n] as f64)/(expPopSize as f64) - simulated[n] as f64;
        }
        errsum / (simulated.len() as f64)
    }
}


/*
fn async main() {
    // Define the mean function and kernel for the Gaussian Process
    let mean_function = Box::new(|x: f64| 0.0);
    let kernel_function = Box::new(|x1: f64, x2: f64| (-0.5 * (x1 - x2).powi(2)).exp());

    // Create a Gaussian Process for Bayesian Optimization
    let gp = GaussianProcess::new(mean_function, kernel_function);

    // Define the Bayesian Optimization instance with a kappa value (exploration-exploitation trade-off)
    let mut bo = BayesianOptimization { gp, kappa: 1.0 };

    // Perform Bayesian Optimization with 10 iterations
    let best_point = bo.optimize(10).await;

    println!("Best Point: {}", best_point);
}
 */