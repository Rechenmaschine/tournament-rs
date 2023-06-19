use anyhow::{anyhow, Error};
use ndarray::{Array, Array1, Array2, Axis};

fn log_likelihood(win_matrix: &Array2<f64>, params: &Array1<f64>) -> f64 {
    let mut ll = 0.0;
    for ((i, j), win) in win_matrix.indexed_iter() {
        ll += win * (params[i].ln() - (params[i] + params[j]).ln());
    }
    -ll
}

fn ll_gradient(win_matrix: &Array2<f64>, params: &Array1<f64>) -> Result<Array1<f64>, Error> {
    let w = win_matrix.sum_axis(Axis(1));
    let mut gradient = Array1::zeros(params.len());

    for i in 0..params.len() {
        let mut sum = 0.0;
        for j in 0..params.len() {
            if i != j {
                sum += (win_matrix[[i, j]] + win_matrix[[j, i]]) / (params[i] + params[j]);
            }
        }

        if sum != 0.0 {
            gradient[i] = w[i] / sum;
        } else {
            return Err(anyhow!("gradient calculation: division by zero"));
        }
    }

    Ok(gradient)
}

fn normalize(v: Array1<f64>) -> Array1<f64> {
    let sum: f64 = v.sum();
    v.map(|x| x / sum)
}

struct BradleyTerry {
    win_matrix: Array2<f64>,
    params: Option<Array1<f64>>,
    is_computed: bool,
    eps: f64,
}

impl BradleyTerry {
    /// Creates a new Bradley-Terry model with the given win matrix.
    /// `eps` is the convergence threshold, and is set to 1e-7 by default.
    fn new(win_matrix: Array2<f64>) -> Self {
        Self {
            win_matrix,
            params: None,
            is_computed: false,
            eps: 1e-7,
        }
    }

    /// Sets the convergence threshold.
    fn with_eps(self, eps: f64) -> Self {
        Self { eps, ..self }
    }

    /// Computes the parameters.
    fn compute_params(&mut self) {
        let mut params = Array::from_vec(vec![1.0; self.win_matrix.len_of(Axis(0))]);

        const MAX_ITER: usize = 200;

        for i in 0..MAX_ITER {
            let grad = ll_gradient(&self.win_matrix, &params).unwrap();
            let new_params = normalize(&params + &grad);

            if i % 10 == 0 {
                // convergence check
                let difference_per_element_squared: f64 =
                    (&new_params - &params).map(|x| x * x).sum() / params.len() as f64;
                if difference_per_element_squared <= self.eps {
                    // converged
                    break;
                }
            }
            params = new_params;
        }

        self.is_computed = true;
        self.params = Some(params);
    }

    /// Returns the estimated parameters.
    /// If the parameters have not been computed yet, they will be computed.
    fn params(&mut self) -> Array1<f64> {
        if !self.is_computed {
            self.compute_params();
        }
        self.params.clone().unwrap()
    }

    /// Updates the win matrix.
    fn update(&mut self, win_matrix: Array2<f64>) {
        self.win_matrix = win_matrix;
        self.is_computed = false;
    }
}

/*
fn main() {
    let start = Instant::now();
    let wins = Array2::from_shape_vec(
        (4, 4),
        vec![
            0., 2., 0., 1.,
            3., 0., 5., 0.,
            0., 3., 0., 1.,
            4., 0., 3., 0.,
        ]).unwrap();

    let mut bt = BradleyTerry::new(wins).with_eps(1e-7);
    let params = bt.params();

    let duration = start.elapsed();
    println!("{}", params);
    println!("Time elapsed: {:?}", duration);
}
 */
