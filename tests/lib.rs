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
  fn bitvec_new_bool() {
    let x = BitVec::new_bool(vec![false, true, true, false]);
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
    let x = BitVec::new_bool(vec);
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
}
