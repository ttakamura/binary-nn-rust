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
    for index in 0..32 {
      assert_eq!(x.get(index), false);
    }
  }

  #[test]
  fn bitpack_tures() {
    let x = Bitpack32::trues();
    for index in 0..32 {
      assert_eq!(x.get(index), true);
    }
  }

  #[test]
  fn bitpack_masked_tures() {
    let x = Bitpack32::masked_trues(9);
    for index in 0..10 {
      assert_eq!(x.get(index), true);
    }
    for index in 10..32 {
      assert_eq!(x.get(index), false);
    }
  }

  #[test]
  fn bitpack_mask() {
    let x = Bitpack32::trues();
    let y = x.mask(24);
    for index in 0..25 {
      assert_eq!(y.get(index), true);
    }
    for index in 25..32 {
      assert_eq!(y.get(index), false);
    }
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

  #[test]
  fn bitpack_xnor() {
    let mut x = Bitpack32::falses();
    let mut y = Bitpack32::falses();
    x.set_true(29);
    x.set_true(30);
    y.set_true(30);
    y.set_true(31);

    let z = x.xnor(&y);
    assert_eq!(z.get(28), true);
    assert_eq!(z.get(29), false);
    assert_eq!(z.get(30), true);
    assert_eq!(z.get(31), false);

    x.mut_xnor(&y);
    assert_eq!(x.get(28), true);
    assert_eq!(x.get(29), false);
    assert_eq!(x.get(30), true);
    assert_eq!(x.get(31), false);
  }

  #[test]
  fn bitpack_pretty_str() {
    let mut x = Bitpack32::falses();
    x.set_true(0);
    x.set_true(5);
    x.set_true(29);
    x.set_true(31);
    assert_eq!(x.pretty_str(), "1000010000 0000000000 0000000001 01");
  }

  #[test]
  fn bitpack_count_ones() {
    let mut x = Bitpack32::falses();
    assert_eq!(x.count_ones(), 0);
    x.set_true(0);
    assert_eq!(x.count_ones(), 1);
    x.set_true(5);
    assert_eq!(x.count_ones(), 2);
    x.set_true(31);
    assert_eq!(x.count_ones(), 3);
  }
}
