use loader;

pub struct BatchNormLayer {
  avg_mean: Vec<f32>,
  avg_var: Vec<f32>,
  beta: Vec<f32>,
  gamma: Vec<f32>,
}

impl BatchNormLayer {
  pub fn load(path: String, nrow: usize) -> BatchNormLayer {
    let mut avg_mean = loader::load_f32(path);
    let mut avg_var = avg_mean.split_off(nrow);
    let mut beta = avg_var.split_off(nrow);
    let gamma = beta.split_off(nrow);
    return BatchNormLayer {
      avg_mean: avg_mean,
      avg_var: avg_var,
      beta: beta,
      gamma: gamma,
    };
  }

  pub fn len(&self) -> usize {
    return self.avg_mean.len();
  }
}
