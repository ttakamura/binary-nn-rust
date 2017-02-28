extern crate binary_nn;

mod stack_tests {
  use binary_nn::layer::linear::*;
  use binary_nn::layer::batch_norm::*;
  use binary_nn::loader;
  use binary_nn::backend::bitmatrix_trait::*;
  use binary_nn::backend::bitvec::*;

  fn argmax(vec: Vec<f32>) -> usize {
    let mut max_idx = 0;
    let mut max = 0.0;
    for i in 0..vec.len() {
      if max < vec[i] {
        max = vec[i];
        max_idx = i;
      }
    }
    return max_idx;
  }

  fn predict(prefix: String, data_path: String) -> usize {
    let l1 = BinaryLinearLayer::load(prefix.clone() + "binary_net.l1.W.dat", 1000, 784);
    let bn1 = BatchNormLayer::load(prefix.clone() + "binary_net.b1.dat", 1000);
    let l2 = BinaryLinearLayer::load(prefix.clone() + "binary_net.l2.W.dat", 1000, 1000);
    let bn2 = BatchNormLayer::load(prefix.clone() + "binary_net.b2.dat", 1000);
    let l3 = BinaryLinearLayer::load(prefix.clone() + "binary_net.l3.W.dat", 10, 1000);
    let bn3 = BatchNormLayer::load(prefix.clone() + "binary_net.b3.dat", 10);

    let a = loader::load_f32(data_path);
    let x = a.into_iter().map(|b| (b * 256.0) as u8).collect();

    let y1 = l1.forward_u8(&x);
    let z1 = bn1.forward(&y1);
    let y2 = l2.forward(&z1);
    let z2 = bn2.forward(&y2);
    let y3 = l3.forward(&z2);
    let z3 = bn3.forward_f32(&y3);
    println!("{:?}", z3);
    return argmax(z3);
  }

  // create mode 100644 tests/data02/binary_net.x.1206.5.dat
  // create mode 100644 tests/data02/binary_net.x.2001.8.dat
  // create mode 100644 tests/data02/binary_net.x.3000.9.dat
  // create mode 100644 tests/data02/binary_net.x.4000.7.dat
  // create mode 100644 tests/data02/binary_net.x.8000.0.dat

  #[test]
  fn predict_test() {
    let y = predict("tests/data02/".to_string(),
                    "tests/data02/binary_net.x.1206.5.dat".to_string());
    assert_eq!(y, 5);

    let y = predict("tests/data02/".to_string(),
                    "tests/data02/binary_net.x.2001.8.dat".to_string());
    assert_eq!(y, 8);

    let y = predict("tests/data02/".to_string(),
                    "tests/data02/binary_net.x.3000.9.dat".to_string());
    assert_eq!(y, 9);

    let y = predict("tests/data02/".to_string(),
                    "tests/data02/binary_net.x.4000.7.dat".to_string());
    assert_eq!(y, 7);
  }

  #[test]
  fn forward_stack_test() {
    let l1 = BinaryLinearLayer::load("tests/data01/binary_net.l1.W.dat".to_string(), 1000, 784);
    let bn1 = BatchNormLayer::load("tests/data01/binary_net.b1.dat".to_string(), 1000);
    let l2 = BinaryLinearLayer::load("tests/data01/binary_net.l2.W.dat".to_string(), 1000, 1000);
    let bn2 = BatchNormLayer::load("tests/data01/binary_net.b2.dat".to_string(), 1000);
    let l3 = BinaryLinearLayer::load("tests/data01/binary_net.l3.W.dat".to_string(), 10, 1000);
    let bn3 = BatchNormLayer::load("tests/data01/binary_net.b3.dat".to_string(), 10);

    let x = vec![128u8; 784];
    let y1 = l1.forward_u8(&x);
    let z1 = bn1.forward(&y1);
    let y2 = l2.forward(&z1);
    let z2 = bn2.forward(&y2);
    let y3 = l3.forward(&z2);
    let z3 = bn3.forward_f32(&y3);

    let expected = loader::load_text_as_f32("tests/data01/output_bn2.txt".to_string());
    for i in 0..expected.len() {
      let b = expected[i] >= 0.0;
      assert_eq!(z2.get(i as u32), b);
    }

    let expected = loader::load_text_as_f32("tests/data01/output_bn3.txt".to_string());
    for i in 0..expected.len() {
      let diff = (expected[i] - z3[i]).abs();
      if diff == 0.0 {
        assert!(true);
      } else {
        println!("bn3 - i {}, z {}, expected {}", i, z3[i], expected[i]);
        assert!(false, "diff {:?} should be small than the threshold", diff);
      }
    }
  }
}
