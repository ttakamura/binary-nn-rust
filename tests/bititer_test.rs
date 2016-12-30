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
  fn bitzip_new() {
    let (_, y) = prepare_vector();
    let yi = y.iter();
    assert_eq!(yi.shape(), 34);
  }

  #[test]
  #[should_panic(expected = "iter.shape should be the same length")]
  fn bitzip_new_panic() {
    let x = BitVec::falses(34);
    let y = BitVec::falses(35);
    let op = BitOpUnion {};
    BitIterZip::new(op, x.iter(), y.iter());
  }

  #[test]
  fn bititer_union() {
    let (x, y) = prepare_vector();
    let mut zipper = x.iter().union(&y.iter());
    let fst = zipper.next().unwrap();
    let snd = zipper.next().unwrap();
    assert_eq!(fst.pretty_str(), "0000000000 0000000000 0000000001 10");
    assert_eq!(snd.pretty_str(), "0100000000 0000000000 0000000000 00");
    assert_eq!(zipper.next().is_none(), true);
  }

  #[test]
  fn bititer_count_ones() {
    let (mut x, y) = prepare_vector();
    {
      let z = x.iter().union(&y.iter());
      assert_eq!(z.count_ones(), 3);
    }
    x.set_true(32);
    {
      let z = x.iter().union(&y.iter());
      assert_eq!(z.count_ones(), 4);
    }
  }
}
