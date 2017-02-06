extern crate binary_nn;
use binary_nn::backend::bitmatrix_trait::*;

mod loader;

fn main() {
  let result = loader::load_f32_as_bitmatrix("data/binary_net.l1.W.dat".to_string(), 1000, 784);
  println!("result[0, 0]   {}", result.get((0, 0)));
  println!("result[999, 783] {}", result.get((999, 783)));

  let bn = loader::load_batch_norm_weight("data/binary_net.b1.dat".to_string(), 1000);
  println!("avg_mean[999] {}", bn.avg_mean.len());
  println!("avg_var[999] {}", bn.avg_var.len());
  println!("beta[999] {}", bn.beta.len());
  println!("gamma[999] {}", bn.gamma.len());
}
