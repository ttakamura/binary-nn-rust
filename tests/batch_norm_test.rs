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

  #[test]
  fn forward_stack_test() {
    let x = vec![128u8; 784];

    let l1 = BinaryLinearLayer::load("tests/data01/binary_net.l1.W.dat".to_string(), 1000, 784);
    let bn1 = BatchNormLayer::load("tests/data01/binary_net.b1.dat".to_string(), 1000);
    let l2 = BinaryLinearLayer::load("tests/data01/binary_net.l2.W.dat".to_string(), 1000, 1000);
    let bn2 = BatchNormLayer::load("tests/data01/binary_net.b2.dat".to_string(), 1000);
    let l3 = BinaryLinearLayer::load("tests/data01/binary_net.l3.W.dat".to_string(), 10, 1000);
    let bn3 = BatchNormLayer::load("tests/data01/binary_net.b3.dat".to_string(), 10);

    let y1 = l1.forward_u8(&x);
    let z1 = bn1.forward(&y1);

    let y2 = l2.forward(&z1);
    let z2 = bn2.forward(&y2);

    let expected = loader::load_text_as_f32("tests/data01/output_bn2.txt".to_string());
    for i in 0..expected.len() {
      if expected[i].abs() > 0.008 {
        let b = expected[i] >= 0.0;
        // println!("i {}, y {}, z {}, expected {}",
        //          i,
        //          y2[i],
        //          z2.get(i as u32),
        //          expected[i]);
        assert_eq!(z2.get(i as u32), b);
      }
    }

    let y3 = l3.forward(&z2);
    let z3 = bn3.forward_f32(&y3);

    let expected = loader::load_text_as_f32("tests/data01/output_bn3.txt".to_string());
    for i in 0..expected.len() {
      let diff = (expected[i] - z3[i]).abs();
      println!("i {}, y {}, z {}, expected {}",
               i,
               y3[i],
               z3[i],
               expected[i]);
      assert_eq!(diff, 0.0);
    }
  }
}
