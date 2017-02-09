use backend::bitmatrix::BitMatrix2;
use backend::bitmatrix_trait::*;

pub struct BinaryLinearLayer {
  weight: BitMatrix2,
}

impl BinaryLinearLayer {
  pub fn forward_u8(&self, x: &Vec<u8>) -> Vec<u8> {
    return Vec::new();
  }
}
