extern crate binary_nn;
use binary_nn::backend::bitmatrix_trait::*;

mod loader;

fn main() {
  let result = loader::load_f32_as_bitmatrix("data/binary_net.l1.W.dat".to_string(), 1000, 784);
  println!("result[0, 0]   {}", result.get((0, 0)));
  for i in 0..100 {
    println!("result[{}, 783] {}", i, result.get((i, 783)));
  }
  println!("result[999, 783] {}", result.get((999, 783)));
}
