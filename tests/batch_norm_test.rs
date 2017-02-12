extern crate binary_nn;

mod batch_norm_layer_tests {
  use binary_nn::layer::linear::*;
  use binary_nn::layer::batch_norm::*;
  use binary_nn::loader;

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
    for (a, b) in z.iter().zip(expected.iter()) {
      // println!("{}, {}", a, b);
      let i = a > &0i32;
      let j = b > &0.0f32;
      assert_eq!(i, j);
    }
  }
}
