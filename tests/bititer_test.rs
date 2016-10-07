extern crate binary_nn;

mod bititer_tests {
  use binary_nn::backend::bititer::*;
  use binary_nn::backend::bitpack::*;
  use binary_nn::backend::bitvec::*;
  use binary_nn::backend::bitmatrix_trait::*;

  fn prepare_vector() -> (BitVec, BitVec) {
    let mut x = BitVec::falses(34);
    x.set_true(29);
    x.set_true(30);
    let mut y = BitVec::falses(34);
    y.set_true(30);
    y.set_true(33);
    return (x, y);
  }

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

  #[test]
  fn bitzip_xnor() {
    //   let (x, y) = prepare_vector();
    //   let op = BitOpXnor {};
    //   let zipper = BitIterZip {
    //     op: op,
    //     left: x.iter(),
    //     right: y.iter(),
    //   };
    //
    //   let first = zipper.next();
    //   let second = zipper
    //
    //   assert_eq!(z.get(9), false);
    //   assert_eq!(z.get(10), true);
    //   assert_eq!(z.get(11), false);
    //   assert_eq!(z.get(12), true);
  }
}
