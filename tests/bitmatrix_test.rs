extern crate binary_nn;

mod bitmatrix_tests {
  use binary_nn::backend::bitmatrix::*;
  use binary_nn::backend::bitpack::Bitpack32;

  #[test]
  fn bitmatrix_from_falses() {
    let x: BitMatrix = BitMatrix::falses((3, 40));
  }
}
