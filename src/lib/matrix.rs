use rand::{thread_rng,Rng};

#[derive(Clone)]
pub struct Matrix {
    pub rows: usize,
    pub cols: usize,
    pub data: Vec<Vec<f64>>
}

impl Matrix {
    pub fn zeros(rows:usize, cols: usize) -> Matrix {
        Matrix { rows,cols, data: vec![vec![0.0; cols]; rows]}
    }

    pub fn get(&mut self, row: usize, col: usize) -> f64 {
        self.data[row][col]
    }

    pub fn random(rows:usize, cols: usize) -> Matrix {
        let mut rng = thread_rng();
        let mut res = Matrix::zeros(rows, cols);
        for i in 0..rows {
            for j in 0..cols {
                res.data[i][j] = rng.gen::<f64>() * 2.0 - 1.0;
            }
        }
        res
    }

    pub fn randomPeep(rows:usize, cols: usize) -> Matrix {
        let mut rng = thread_rng();
        let mut res = Matrix::zeros(rows, cols);
        for i in 0..rows {
            for j in 0..cols {
                res.data[i][j] = rng.gen::<f64>();
            }
        }
        res
    }

    //pub fn randomTensor(rows: usize, cols: usize) -> Tensor<f64> {
    //    let mut rng = thread_rng();
    //    let mut res: Tensor = vec![vec![0.0,cols],rows];
    //    for i in 0..rows {
    //       for j in 0..cols {
    //            res.data[i][j] = rng.gen::<f64>() * 2.0 - 1.0;
    //        }
    //    }
    //    [res]
    //}

    pub fn from(data: Vec<Vec<f64>>) -> Matrix {
        Matrix { rows: data.len(), cols: data[0].len(), data }
    }

    pub fn multiply(&mut self, other:&Matrix) -> Matrix{
        if self.cols != other.rows {
            panic!("Attempted to multiply by matrix of incorrect dimensions")
        }
        let mut res = Matrix::zeros(self.rows, other.cols);

        for i in 0..self.rows {
            for j in 0..other.cols {
                let mut sum = 0.0;
                for k in 0..self.cols {
                    sum += self.data[i][k] * other.data[k][j];
                }
                res.data[i][j] = sum;
            }
        }
        res
    }

    pub fn add(&mut self, other: &Matrix) -> Matrix {
        if self.rows != other.rows || self.cols != other.cols {
            panic!("Attempted to add matrices of incorrect dimensions")
        }
        let mut res = Matrix::zeros(self.rows, other.cols);

        for i in 0..self.rows {
            for j in 0..self.cols {
                res.data[i][j] = self.data[i][j] + other.data[i][j];
            }
        }
        res
    }

    pub fn point_multiply(&mut self, other: &Matrix) -> Matrix {
        if self.rows != other.rows || self.cols != other.cols {
            panic!("Attempted to point multiply matrices of incorrect dimensions")
        }
        let mut res = Matrix::zeros(self.rows, other.cols);

        for i in 0..self.rows {
            for j in 0..self.cols {
                res.data[i][j] = self.data[i][j] * other.data[i][j];
            }
        }
        res
    }

    pub fn subtract(&mut self, other: &Matrix) -> Matrix {
        if self.rows != other.rows || self.cols != other.cols {
            panic!("Attempted to subtract matrices of incorrect dimensions")
        }
        let mut res = Matrix::zeros(self.rows, other.cols);

        for i in 0..self.rows {
            for j in 0..self.cols {
                res.data[i][j] = self.data[i][j] - other.data[i][j];
            }
        }
        res
    }

    pub fn map(&mut self, function: &dyn Fn(f64)-> f64) -> Matrix {
        Matrix::from((self.data)
        .clone()
        .into_iter()
        .map(|row| row.into_iter().map(|value| function(value)).collect()).collect())
    }

    pub fn transpose (&mut self) -> Matrix {
        let mut res = Matrix::zeros(self.cols, self.rows);

        for i in 0..self.rows {
            for j in 0..self.cols {
                res.data[j][i] = self.data[i][j];
            }
        }
        res
    }
}