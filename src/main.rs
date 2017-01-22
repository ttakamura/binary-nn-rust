extern crate binary_nn;
mod loader;

fn main() {
  let result = loader::load_f32("data/binary_net.l1.b.dat".to_string());
  println!("result length is {}", result.len());
}
