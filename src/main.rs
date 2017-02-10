extern crate binary_nn;
use binary_nn::backend::bitmatrix_trait::*;
use binary_nn::layer::batch_norm::*;
use binary_nn::layer::linear::*;

fn main() {
  let l1 = BinaryLinearLayer::load("data/binary_net.l1.W.dat".to_string(), 1000, 784);
  println!("l1[0, 0]   {}", l1.weight.get((0, 0)));
  println!("l1[999, 783] {}", l1.weight.get((999, 783)));

  let z = l1.forward_u8(&vec![128u8; 784]);
  println!("z[0] {}", z[0]);
  println!("z[500] {}", z[500]);
  println!("z[999] {}", z[999]);

  let bn = BatchNormLayer::load("data/binary_net.b1.dat".to_string(), 1000);
  println!("bn.len {}", bn.len());
}
