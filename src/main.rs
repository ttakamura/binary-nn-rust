extern crate binary_nn;

fn main() {
  let x: i32 = 1;
  let y: i32 = 4;
  println!("{:?}", x | y);

  println!("3 + 5 = {}", binary_nn::sandbox::add(3, 5));
  println!("Hello, world!");
}
