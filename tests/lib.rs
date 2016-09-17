extern crate binary_nn;
use binary_nn::*;

#[test]
fn add_test() {
  assert!(sandbox::add(3, 5) == 8);
}

mod bitvec_tests {
  use binary_nn::backend::bitvec::*;

  #[test]
  fn bitvec_new() {
    let x = BitVec::new(vec![(1 << 31)], 1);
    assert_eq!(x.get(0), Some(true));
    assert_eq!(x.get(1), None);
  }

  #[test]
  fn bitvec_from_bool() {
    let x = BitVec::from_bool(vec![false, true, true, false]);
    assert_eq!(x.get(0), Some(false));
    assert_eq!(x.get(1), Some(true));
    assert_eq!(x.get(2), Some(true));
    assert_eq!(x.get(3), Some(false));
    assert_eq!(x.get(4), None);

    let mut vec: Vec<bool> = vec![];
    for i in 0..33 {
      if i < 31 {
        vec.push(false);
      } else {
        vec.push(true);
      }
    }
    let x = BitVec::from_bool(vec);
    assert_eq!(x.get(30), Some(false));
    assert_eq!(x.get(31), Some(true));
    assert_eq!(x.get(32), Some(true));
    assert_eq!(x.get(33), None);
  }

  #[test]
  fn bitvec_get() {
    let x = BitVec::new(vec![1], 32);
    assert_eq!(x.get(30), Some(false));
    assert_eq!(x.get(31), Some(true));
    assert_eq!(x.get(32), None);

    let x = BitVec::new(vec![1, (1 << 31)], 33);
    assert_eq!(x.get(30), Some(false));
    assert_eq!(x.get(31), Some(true));
    assert_eq!(x.get(32), Some(true));
    assert_eq!(x.get(33), None);
  }

  #[test]
  fn bitvec_set() {
    let mut x = BitVec::falses(33);
    assert_eq!(x.get(32), Some(false));
    x.set_true(32);
    assert_eq!(x.get(32), Some(true));
    x.set_false(32);
    assert_eq!(x.get(32), Some(false));
  }

  #[test]
  #[should_panic(expected = "index should smaller than self.nbits")]
  fn bitvec_set_overflow() {
    let mut x = BitVec::falses(33);
    x.set_true(33);
  }

  #[test]
  fn bitvec_len() {
    let mut tmp_x: Vec<bool> = vec![];
    for _ in 0..34 {
      tmp_x.push(false);
    }
    let x = BitVec::from_bool(tmp_x);
    assert_eq!(x.len(), 34);
  }

  #[test]
  fn bitvec_falses() {
    let x = BitVec::falses(33);
    assert_eq!(x.get(0), Some(false));
    assert_eq!(x.get(1), Some(false));
    assert_eq!(x.get(31), Some(false));
    assert_eq!(x.get(32), Some(false));
    assert_eq!(x.get(33), None);
    assert_eq!(x.get(34), None);
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

    assert_eq!(x.get(0), Some(false));
    assert_eq!(x.get(30), Some(true));
    assert_eq!(x.get(31), Some(true));
    assert_eq!(x.get(32), Some(true));
    assert_eq!(x.get(33), Some(false));
    assert_eq!(x.get(34), None);
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

    assert_eq!(x.get(0), Some(false));
    assert_eq!(x.get(30), Some(false));
    assert_eq!(x.get(31), Some(true));
    assert_eq!(x.get(32), Some(false));
    assert_eq!(x.get(33), Some(false));
    assert_eq!(x.get(34), None);
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

    assert_eq!(x.get(0), Some(false));
    assert_eq!(x.get(30), Some(true));
    assert_eq!(x.get(31), Some(false));
    assert_eq!(x.get(32), Some(true));
    assert_eq!(x.get(33), Some(false));
    assert_eq!(x.get(34), None);
  }
}
