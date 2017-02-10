use backend::bitmatrix::BitMatrix2;
use loader;

pub struct BinaryLinearLayer {
  pub weight: BitMatrix2,
}

impl BinaryLinearLayer {
  pub fn load(path: String, nrow: u32, ncol: u32) -> BinaryLinearLayer {
    let weight = loader::load_f32_as_bitmatrix(path, nrow, ncol);
    return BinaryLinearLayer { weight: weight };
  }

  pub fn forward_u8(&self, x: &Vec<u8>) -> Vec<u8> {
    return Vec::new();
  }
}
