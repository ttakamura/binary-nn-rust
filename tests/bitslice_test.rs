extern crate binary_nn;

mod bitslice_tests {
  use binary_nn::backend::bitvec::*;
  use binary_nn::backend::bitslice::*;
  use binary_nn::backend::bitpack::Bitpack32;
  use binary_nn::backend::bitmatrix::*;

  #[test]
  fn bitslice_get() {
    let mut xx = BitVec::new(vec![Bitpack32::new(1), Bitpack32::new(1 << 31)], 33);
    let mut x = xx.as_slice();
    assert_eq!(x.get(30), false);
    assert_eq!(x.get(31), true);
    assert_eq!(x.get(32), true);
  }

  #[test]
  fn bitslice_set() {
    let mut xx = BitVec::falses(33);
    let mut x = xx.as_slice();
    assert_eq!(x.get(32), false);
    x.set_true(32);
    assert_eq!(x.get(32), true);
    x.set_false(32);
    assert_eq!(x.get(32), false);
  }

  #[test]
  fn bitslice_xor() {
    let mut xx = BitVec::falses(34);
    let mut yy = BitVec::falses(34);
    let mut x = xx.as_slice();
    let mut y = yy.as_slice();
    x.set_true(30);
    x.set_true(31);
    y.set_true(31);
    y.set_true(32);
    x.mut_xor(&y);
    assert_eq!(x.get(0), false);
    assert_eq!(x.get(30), true);
    assert_eq!(x.get(31), false);
    assert_eq!(x.get(32), true);
    assert_eq!(x.get(33), false);
  }
}
