use backend::bitmatrix::BitMatrix2;
use backend::bitmatrix_trait::*;
use backend::bitvec::BitVec;
use loader;

pub struct BinaryLinearLayer {
  pub weight: BitMatrix2,
}

impl BinaryLinearLayer {
  pub fn load(path: String, nrow: u32, ncol: u32) -> BinaryLinearLayer {
    let weight = loader::load_f32_as_bitmatrix(path, nrow, ncol);
    return BinaryLinearLayer { weight: weight };
  }

  pub fn forward_u8(&self, x: &Vec<u8>) -> Vec<i32> {
    let mut z = Vec::new();
    let (nrow, ncol) = self.weight.len();
    for irow in 0..nrow {
      let mut val = 0i32;
      for icol in 0..ncol {
        if self.weight.get((irow, icol)) {
          val += x[icol as usize] as i32;
        } else {
          val -= x[icol as usize] as i32;
        }
      }
      // println!("{}, {}", irow, val);
      z.push(val);
    }
    return z;
  }

  pub fn forward(&self, x: &BitVec) -> Vec<i32> {
    return self.weight.dot_vec(x);
  }
}
