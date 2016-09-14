extern crate binary_nn;

mod sandbox_test {
  use binary_nn::sandbox::*;

  #[test]
  fn add_test() {
    assert!(add(3, 5) == 8);
  }
}
