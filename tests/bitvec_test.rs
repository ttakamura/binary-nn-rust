extern crate binary_nn;

mod bitvec_tests {
  use binary_nn::backend::bitvec::*;
  use binary_nn::backend::bitpack::Bitpack32;

  #[test]
  fn bitvec_new() {
    let x = BitVec::new(vec![Bitpack32::new(1 << 31)], 2);
    assert_eq!(x.get(0), true);
    assert_eq!(x.get(1), false);
  }

  #[test]
  fn bitvec_get() {
    let x = BitVec::new(vec![Bitpack32::new(1)], 32);
    assert_eq!(x.get(30), false);
    assert_eq!(x.get(31), true);

    let x = BitVec::new(vec![Bitpack32::new(1), Bitpack32::new(1 << 31)], 33);
    assert_eq!(x.get(30), false);
    assert_eq!(x.get(31), true);
    assert_eq!(x.get(32), true);
  }

  #[test]
  #[should_panic(expected = "index should smaller than self.nbits")]
  fn bitvec_get_overflow() {
    let x = BitVec::falses(33);
    assert_eq!(x.get(32), false);
    assert!(x.get(33));
  }

  #[test]
  fn bitvec_set() {
    let mut x = BitVec::falses(33);
    assert_eq!(x.get(32), false);
    x.set_true(32);
    assert_eq!(x.get(32), true);
    x.set_false(32);
    assert_eq!(x.get(32), false);
  }

  #[test]
  #[should_panic(expected = "index should smaller than self.nbits")]
  fn bitvec_set_overflow() {
    let mut x = BitVec::falses(33);
    x.set_true(33);
  }

  #[test]
  fn bitvec_falses() {
    let x = BitVec::falses(33);
    assert_eq!(x.get(0), false);
    assert_eq!(x.get(1), false);
    assert_eq!(x.get(31), false);
    assert_eq!(x.get(32), false);
  }

  #[test]
  fn bitvec_union() {
    let mut x = BitVec::falses(34);
    let mut y = BitVec::falses(34);
    x.set_true(30);
    x.set_true(31);
    y.set_true(31);
    y.set_true(32);
    x.mut_union(&y);
    assert_eq!(x.get(0), false);
    assert_eq!(x.get(30), true);
    assert_eq!(x.get(31), true);
    assert_eq!(x.get(32), true);
    assert_eq!(x.get(33), false);
  }

  #[test]
  fn bitvec_intersect() {
    let mut x = BitVec::falses(34);
    let mut y = BitVec::falses(34);
    x.set_true(30);
    x.set_true(31);
    y.set_true(31);
    y.set_true(32);
    x.mut_intersect(&y);
    assert_eq!(x.get(0), false);
    assert_eq!(x.get(30), false);
    assert_eq!(x.get(31), true);
    assert_eq!(x.get(32), false);
    assert_eq!(x.get(33), false);
  }

  #[test]
  fn bitvec_xor() {
    let mut x = BitVec::falses(34);
    let mut y = BitVec::falses(34);
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
