extern crate binary_nn;

mod bitpack_tests {
  use binary_nn::backend::bitpack::*;

  #[test]
  fn bitpack_from_bool() {
    let x = Bitpack32::from_bool(vec![false, true, true, false]);
    assert_eq!(x.get(0), false);
    assert_eq!(x.get(1), true);
    assert_eq!(x.get(2), true);
    assert_eq!(x.get(3), false);

    let mut vec: Vec<bool> = vec![];
    for i in 0..32 {
      if i < 30 {
        vec.push(false);
      } else {
        vec.push(true);
      }
    }
    let x = Bitpack32::from_bool(vec);
    assert_eq!(x.get(29), false);
    assert_eq!(x.get(30), true);
    assert_eq!(x.get(31), true);
  }

  #[test]
  fn bitpack_get() {
    let x = Bitpack32::new(1);
    assert_eq!(x.get(30), false);
    assert_eq!(x.get(31), true);

    let x = Bitpack32::new(1 << 31);
    assert_eq!(x.get(0), true);
    assert_eq!(x.get(1), false);
  }

  #[test]
  fn bitpack_set() {
    let mut x = Bitpack32::falses();
    assert_eq!(x.get(31), false);
    x.set_true(31);
    assert_eq!(x.get(31), true);
    x.set_false(31);
    assert_eq!(x.get(31), false);
  }

  #[test]
  #[should_panic(expected = "index should smaller than limit_index")]
  fn bitpack_set_overflow() {
    let mut x = Bitpack32::falses();
    x.set_true(32);
  }

  #[test]
  fn bitpack_falses() {
    let x = Bitpack32::falses();
    assert_eq!(x.get(0), false);
    assert_eq!(x.get(1), false);
    assert_eq!(x.get(31), false);
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
    assert_eq!(x.get(0), false);
    assert_eq!(x.get(29), true);
    assert_eq!(x.get(30), true);
    assert_eq!(x.get(31), true);
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
    assert_eq!(x.get(0), false);
    assert_eq!(x.get(29), false);
    assert_eq!(x.get(30), true);
    assert_eq!(x.get(31), false);
  }

  #[test]
  fn bitpack_xor() {
    let mut x = Bitpack32::falses();
    let mut y = Bitpack32::falses();
    x.set_true(29);
    x.set_true(30);
    y.set_true(30);
    y.set_true(31);
    x.mut_xor(&y);
    assert_eq!(x.get(0), false);
    assert_eq!(x.get(29), true);
    assert_eq!(x.get(30), false);
    assert_eq!(x.get(31), true);
  }
}