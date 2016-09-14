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
    let y: u32 = 1 << 31;
    let x = BitVec::new(vec![y], 1);
    assert_eq!(x.get(0), Some(true));
    assert_eq!(x.get(1), None);
  }
}
