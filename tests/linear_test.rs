extern crate binary_nn;

mod linear_layer_tests {
  use binary_nn::backend::bitmatrix_trait::*;
  use binary_nn::layer::linear::*;
  use binary_nn::loader;

  #[test]
  fn load_test() {
    let l = BinaryLinearLayer::load("tests/data01/binary_net.l1.W.dat".to_string(), 1000, 784);
    let (nrow, ncol) = l.weight.len();
    assert_eq!(nrow, 1000);
    assert_eq!(ncol, 784);
  }

  #[test]
  fn forward_u8_test() {
    let l = BinaryLinearLayer::load("tests/data01/binary_net.l1.W.dat".to_string(), 1000, 784);
    let x = vec![128u8; 784];
    let actual_y = l.forward_u8(&x);
    let expected_y = loader::load_text_as_i32("tests/data01/output_y.txt".to_string());
    for (a, b) in actual_y.iter().zip(expected_y.iter()) {
      assert_eq!(a, b);
    }
  }
}
