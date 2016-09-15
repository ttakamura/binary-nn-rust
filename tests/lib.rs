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
