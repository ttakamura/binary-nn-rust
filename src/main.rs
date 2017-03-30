extern crate binary_nn;
use binary_nn::backend::bitmatrix_trait::*;
use binary_nn::layer::base::*;
use binary_nn::layer::batch_norm::*;
use binary_nn::layer::linear::*;
use binary_nn::loader;

fn main() {
  let l1 = BinaryLinearLayer::load("data/binary_net.l1.W.dat".to_string(), 1000, 784);
  println!("l1[0, 0]   {}", l1.weight.get((0, 0)));
  println!("l1[999, 783] {}", l1.weight.get((999, 783)));

  let serialized_l1 = l1.encode();
  println!("serialized_l1 {}", serialized_l1.len());

  let l1_2 = BinaryLinearLayer::decode(serialized_l1);
  println!("l1_2[0, 0]   {}", l1_2.weight.get((0, 0)));
  println!("l1_2[999, 783] {}", l1_2.weight.get((999, 783)));

  let z = l1.forward_u8(&vec![128u8; 784]);
  println!("z[0] {}", z[0]);
  println!("z[500] {}", z[500]);
  println!("z[999] {}", z[999]);

  let bn = BatchNormLayer::load("data/binary_net.b1.dat".to_string(), 1000);
  println!("bn.len {}", bn.len());
  println!("bn.threshold[100] {}", bn.threshold[100]);
  println!("bn.threshold[200] {}", bn.threshold[200]);
  println!("bn.threshold[300] {}", bn.threshold[300]);

  let serialized_bn = bn.encode();
  println!("serialized_bn {}", serialized_bn.len());

  let bn2 = BatchNormLayer::decode(serialized_bn);
  println!("bn2.threshold[100] {}", bn2.threshold[100]);
  println!("bn2.threshold[200] {}", bn2.threshold[200]);
  println!("bn2.threshold[300] {}", bn2.threshold[300]);

  let x = loader::load_text_as_i32("data/output_y.txt".to_string());
  println!("x[100] {}", x[100]);

  let x = loader::load_text_as_f32("data/output_bn.txt".to_string());
  println!("x[100] {}", x[100]);
}
