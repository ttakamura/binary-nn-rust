extern crate binary_nn;
use binary_nn::*;

mod bitpack_tests {
  use binary_nn::backend::bitpack::*;

  #[test]
  fn bitpack_from_bool() {
    let x = Bitpack32::from_bool(vec![false, true, true, false]);
    assert_eq!(x.get(0), Some(false));
    assert_eq!(x.get(1), Some(true));
    assert_eq!(x.get(2), Some(true));
    assert_eq!(x.get(3), Some(false));
    assert_eq!(x.get(4), None);

    let mut vec: Vec<bool> = vec![];
    for i in 0..32 {
      if i < 30 {
        vec.push(false);
      } else {
        vec.push(true);
      }
    }
    let x = Bitpack32::from_bool(vec);
    assert_eq!(x.get(29), Some(false));
    assert_eq!(x.get(30), Some(true));
    assert_eq!(x.get(31), Some(true));
  }

  #[test]
  fn bitpack_get() {
    let x = Bitpack32 { storage: 1 };
    assert_eq!(x.get(30), Some(false));
    assert_eq!(x.get(31), Some(true));
    assert_eq!(x.get(32), None);

    let x = Bitpack32 { storage: 1 << 31 };
    assert_eq!(x.get(0), Some(true));
    assert_eq!(x.get(1), Some(false));
  }

  #[test]
  fn bitpack_set() {
    let mut x = Bitpack32::falses();
    assert_eq!(x.get(31), Some(false));
    x.set_true(31);
    assert_eq!(x.get(31), Some(true));
    x.set_false(31);
    assert_eq!(x.get(31), Some(false));
  }

  #[test]
  #[should_panic(expected = "index should smaller than self.nbits")]
  fn bitpack_set_overflow() {
    let mut x = Bitpack32::falses();
    x.set_true(32);
  }

  #[test]
  fn bitpack_falses() {
    let x = BitVec::falses();
    assert_eq!(x.get(0), Some(false));
    assert_eq!(x.get(1), Some(false));
    assert_eq!(x.get(31), Some(false));
  }

  #[test]
  fn bitpack_union() {
    let mut x = Bitpack32::falses();
    let mut y = Bitpack32::falses();
    x.set_true(29);
    x.set_true(30);
    y.set_true(30);
    y.set_true(31);
    x.mut_union(&y);
    assert_eq!(x.get(0), Some(false));
    assert_eq!(x.get(29), Some(true));
    assert_eq!(x.get(30), Some(true));
    assert_eq!(x.get(31), Some(true));
  }

  #[test]
  fn bitpack_intersect() {
    let mut x = Bitpack32::falses();
    let mut y = Bitpack32::falses();
    x.set_true(29);
    x.set_true(30);
    y.set_true(30);
    y.set_true(31);
    x.mut_intersect(&y);
    assert_eq!(x.get(0), Some(false));
    assert_eq!(x.get(29), Some(false));
    assert_eq!(x.get(30), Some(true));
    assert_eq!(x.get(31), Some(false));
  }

  #[test]
  fn bitvec_xor() {
    let mut x = Bitpack32::falses();
    let mut y = Bitpack32::falses();
    x.set_true(29);
    x.set_true(30);
    y.set_true(30);
    y.set_true(31);
    x.mut_xor(&y);
    assert_eq!(x.get(0), Some(false));
    assert_eq!(x.get(29), Some(true));
    assert_eq!(x.get(30), Some(false));
    assert_eq!(x.get(31), Some(true));
  }

}
