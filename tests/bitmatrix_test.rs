extern crate binary_nn;

mod bitmatrix_tests {
  use binary_nn::backend::bitmatrix::*;

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

  #[test]
  fn bitmatrix_offset_of_test() {
    let x = BitMatrix2::falses((3, 40));

    assert_eq!(x.offset_of((0, 0)), (0, 0));
    assert_eq!(x.offset_of((0, 10)), (0, 10));
    assert_eq!(x.offset_of((0, 31)), (0, 31));

    assert_eq!(x.offset_of((0, 32 + 0)), (1, 0));
    assert_eq!(x.offset_of((0, 32 + 7)), (1, 7));

    assert_eq!(x.offset_of((1, 0)), (2, 0));
    assert_eq!(x.offset_of((1, 5)), (2, 5));
    assert_eq!(x.offset_of((1, 31)), (2, 31));

    assert_eq!(x.offset_of((1, 32 + 0)), (3, 0));
    assert_eq!(x.offset_of((1, 32 + 7)), (3, 7));

    assert_eq!(x.offset_of((2, 0)), (4, 0));
    assert_eq!(x.offset_of((2, 5)), (4, 5));
    assert_eq!(x.offset_of((2, 31)), (4, 31));

    assert_eq!(x.offset_of((2, 32 + 0)), (5, 0));
    assert_eq!(x.offset_of((2, 32 + 7)), (5, 7));
  }

  #[test]
  fn bitmatrix_from_falses() {
    let x = BitMatrix2::falses((3, 40));
    assert_eq!(x.get((0, 0)), false);
    assert_eq!(x.get((1, 20)), false);
    assert_eq!(x.get((2, 39)), false);
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
