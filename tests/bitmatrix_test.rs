extern crate binary_nn;

mod bitmatrix_tests {
  use binary_nn::backend::bitmatrix::*;
  use binary_nn::backend::bitpack::Bitpack32;

  #[test]
  fn bitmatrix_set_true() {
    let mut x = BitMatrix2::falses((3, 40));
    assert_eq!(x.get((1, 33)), false);
    x.set_true((1, 33));
    assert_eq!(x.get((1, 33)), true);
  }
}
