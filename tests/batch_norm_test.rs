extern crate binary_nn;

mod batch_norm_layer_tests {
  use binary_nn::layer::linear::*;
  use binary_nn::layer::batch_norm::*;
  use binary_nn::loader;
  use binary_nn::backend::bitmatrix_trait::*;

  #[test]
  fn load_test() {
    let bn = BatchNormLayer::load("tests/data01/binary_net.b1.dat".to_string(), 1000);
    assert_eq!(bn.len(), 1000);
  }

  #[test]
  fn forward_test() {
    let l = BinaryLinearLayer::load("tests/data01/binary_net.l1.W.dat".to_string(), 1000, 784);
    let bn = BatchNormLayer::load("tests/data01/binary_net.b1.dat".to_string(), 1000);
    let x = vec![128u8; 784];
    let y = l.forward_u8(&x);
    let z = bn.forward(&y);

    let expected = loader::load_text_as_f32("tests/data01/output_bn.txt".to_string());
    for i in 0..expected.len() {
      let b = expected[i] >= 0.0;
      assert_eq!(z.get(i as u32), b);
    }
  }
}
