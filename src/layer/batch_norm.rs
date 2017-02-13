use loader;
use backend::bitvec::*;
use backend::bitmatrix_trait::*;

pub struct BatchNormLayer {
  // avg_mean: Vec<f32>,
  // avg_var: Vec<f32>,
  // beta: Vec<f32>,
  // gamma: Vec<f32>,
  pub threshold: Vec<i32>,
}

impl BatchNormLayer {
  pub fn new(avg_mean: Vec<f32>, avg_var: Vec<f32>, beta: Vec<f32>, gamma: Vec<f32>) -> BatchNormLayer {
    let mut threshold = Vec::new();
    let e = 0.00001;

    for i in 0..avg_mean.len() {
      // avg_mean - ((beta * math.sqrt(avg_var + 0.0001)) / gamma)
      let x = avg_mean[i] - ((beta[i] * (avg_var[i] + e).sqrt()) / gamma[i]);
      threshold.push(x as i32);
    }

    return BatchNormLayer {
      // avg_mean: avg_mean,
      // avg_var: avg_var,
      // beta: beta,
      // gamma: gamma,
      threshold: threshold,
    };
  }

  pub fn load(path: String, nrow: usize) -> BatchNormLayer {
    let mut avg_mean = loader::load_f32(path);
    let mut avg_var = avg_mean.split_off(nrow);
    let mut beta = avg_var.split_off(nrow);
    let gamma = beta.split_off(nrow);
    return BatchNormLayer::new(avg_mean, avg_var, beta, gamma);
  }

  pub fn len(&self) -> usize {
    return self.threshold.len();
  }

  pub fn forward(&self, x_vec: &Vec<i32>) -> BitVec {
    let length = self.len();
    let mut result = BitVec::falses(length as u32);
    for i in 0..length {
      let z = x_vec[i] - self.threshold[i];
      if z >= 0 {
        result.set_true(i as u32);
      } else {
        result.set_false(i as u32);
      }
    }
    return result;
  }
}
