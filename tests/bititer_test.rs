extern crate binary_nn;

mod bititer_tests {
  use binary_nn::backend::bititer::*;
  use binary_nn::backend::bitpack::*;

  #[test]
  fn bitop_xnor() {
    let mut x = Bitpack32::falses();
    let mut y = Bitpack32::falses();
    x.set_true(10);
    x.set_true(11);
    y.set_true(9);
    y.set_true(10);

    let op = BitOpXnor {};
    let z = op.process(x, y);
    assert_eq!(z.get(9), false);
    assert_eq!(z.get(10), true);
    assert_eq!(z.get(11), false);
    assert_eq!(z.get(12), true);
  }
}
