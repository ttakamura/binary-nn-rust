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
  fn bitvec_or() {
    let mut tmp_x: Vec<bool> = vec![];
    let mut tmp_y: Vec<bool> = vec![];
    for _ in 0..31 {
      tmp_x.push(false);
      tmp_y.push(false);
    }
    // 31
    tmp_x.push(true);
    tmp_y.push(false);
    // 32
    tmp_x.push(false);
    tmp_y.push(true);
    // 33
    tmp_x.push(false);
    tmp_y.push(false);

    let mut x = BitVec::from_bool(tmp_x);
    let y = BitVec::from_bool(tmp_y);
    x.mut_union(&y);

    assert_eq!(x.get(0), Some(false));
    assert_eq!(x.get(1), Some(false));
    assert_eq!(x.get(2), Some(false));
    assert_eq!(x.get(30), Some(false));
    assert_eq!(x.get(31), Some(true));
    assert_eq!(x.get(32), Some(true));
    assert_eq!(x.get(33), Some(false));
    assert_eq!(x.get(34), None);
    assert_eq!(x.get(35), None);
  }
}
