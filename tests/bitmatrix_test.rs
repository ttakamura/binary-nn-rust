extern crate binary_nn;

mod bitmatrix_tests {
  use binary_nn::backend::bitmatrix::*;
  use binary_nn::backend::bitpack::Bitpack32;

  fn prepare_matrix_for_union() -> (BitMatrix2, BitMatrix2) {
    let mut x = BitMatrix2::falses((3, 34));
    let mut y = BitMatrix2::falses((3, 34));
    x.set_true((1, 30));
    x.set_true((1, 31));
    y.set_true((1, 31));
    y.set_true((1, 32));
    return (x, y);
  }

  #[test]
  fn bitmatrix_set() {
    let mut x = BitMatrix2::falses((3, 40));
    assert_eq!(x.get((1, 33)), false);
    x.set_true((1, 33));
    assert_eq!(x.get((1, 33)), true);
    x.set_false((1, 33));
    assert_eq!(x.get((1, 33)), false);
  }

  #[test]
  fn bitmatrix_union() {
    let (mut x, y) = prepare_matrix_for_union();
    x.mut_union(&y);
    assert_eq!(x.get((1, 0)), false);
    assert_eq!(x.get((1, 30)), true);
    assert_eq!(x.get((1, 31)), true);
    assert_eq!(x.get((1, 32)), true);
    assert_eq!(x.get((1, 33)), false);
  }

  #[test]
  fn bitmatrix_intersect() {
    let (mut x, y) = prepare_matrix_for_union();
    x.mut_intersect(&y);
    assert_eq!(x.get((1, 0)), false);
    assert_eq!(x.get((1, 30)), false);
    assert_eq!(x.get((1, 31)), true);
    assert_eq!(x.get((1, 32)), false);
    assert_eq!(x.get((1, 33)), false);
  }

  #[test]
  fn bitmatrix_xor() {
    let (mut x, y) = prepare_matrix_for_union();
    x.mut_xor(&y);
    assert_eq!(x.get((1, 0)), false);
    assert_eq!(x.get((1, 30)), true);
    assert_eq!(x.get((1, 31)), false);
    assert_eq!(x.get((1, 32)), true);
    assert_eq!(x.get((1, 33)), false);
  }

  // #[test]
  // fn bitmatrix_row() {
  //   let mut x = BitMatrix2::falses((3, 34));
  //   x.set_true((1, 0));
  //   x.set_true((1, 10));
  //   x.set_true((1, 31));
  //   x.set_true((1, 33));
  //
  //   let row = x.row(1);
  //   assert_eq!(row.get(0), true);
  //   assert_eq!(row.get(1), false);
  //   assert_eq!(row.get(9), false);
  //   assert_eq!(row.get(10), true);
  //   assert_eq!(row.get(11), false);
  //   assert_eq!(row.get(30), false);
  //   assert_eq!(row.get(31), true);
  //   assert_eq!(row.get(32), false);
  //   assert_eq!(row.get(33), true);
  // }
}
