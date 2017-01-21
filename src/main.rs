extern crate binary_nn;
mod loader;

fn main() {
  loader::load("data/binary_net.l1.b.dat".to_string());
}
