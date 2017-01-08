extern crate binary_nn;

mod bitmatrix_tests {
  use binary_nn::backend::bititer::*;
  use binary_nn::backend::bitmatrix::*;
  use binary_nn::backend::bitmatrix_trait::*;

  fn prepare_matrix_for_union() -> (BitMatrix2, BitMatrix2) {
    let mut x = BitMatrix2::falses((3, 34));
    let mut y = BitMatrix2::falses((3, 34));
    x.set_true((0, 10));
    x.set_true((1, 30));
    x.set_true((1, 31));
    y.set_true((1, 31));
    y.set_true((1, 32));
    y.set_true((2, 33));
    return (x, y);
  }

  #[test]
  fn bitmatrix_col_vec() {
    let mut x = BitMatrix2::falses((3, 34));
    x.set_true((1, 10));

    let y = x.col_vec(9);
    assert_eq!(y.len(), 3);
    assert_eq!(y.get(0), false);
    assert_eq!(y.get(1), false);
    assert_eq!(y.get(2), false);

    let y = x.col_vec(10);
    assert_eq!(y.len(), 3);
    assert_eq!(y.get(0), false);
    assert_eq!(y.get(1), true);
    assert_eq!(y.get(2), false);
  }

  #[test]
  fn bitmatrix_dot() {
    let mut x = BitMatrix2::falses((3, 34));
    let mut y = BitMatrix2::falses((34, 2));
    let z = x.dot(&y);
    assert_eq!(z.len(), 3);
    assert_eq!(z[0].len(), 2);
    assert_eq!(z[0][0], 34);
    assert_eq!(z[1][0], 34);
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
    let (x, y) = prepare_matrix_for_union();
    let z = BitMatrix2::from_iter(x.iter().union(&y.iter()));
    assert_eq!(z.get((0, 10)), true);
    assert_eq!(z.get((1, 0)), false);
    assert_eq!(z.get((1, 30)), true);
    assert_eq!(z.get((1, 31)), true);
    assert_eq!(z.get((1, 32)), true);
    assert_eq!(z.get((1, 33)), false);
    assert_eq!(z.get((2, 33)), true);
  }

  #[test]
  fn bitmatrix_xnor() {
    let (x, y) = prepare_matrix_for_union();
    let z = BitMatrix2::from_iter(x.iter().xnor(&y.iter()));
    assert_eq!(z.get((0, 10)), false);
    assert_eq!(z.get((1, 0)), true);
    assert_eq!(z.get((1, 30)), false);
    assert_eq!(z.get((1, 31)), true);
    assert_eq!(z.get((1, 32)), false);
    assert_eq!(z.get((1, 33)), true);
    assert_eq!(z.get((2, 33)), false);
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

  // TODO
  // #[test]
  // fn bitmatrix_row_iter() {
  //   let mut x = BitMatrix2::falses((3, 34));
  //   x.set_true((1, 0));
  //   x.set_true((1, 31));
  //   x.set_true((1, 33));
  //
  //   let mut y = BitMatrix2::falses((3, 34));
  //   y.set_true((1, 10));
  //   y.set_true((1, 20));
  //
  //   {
  //     let mut xrow = x.mut_row_iter(1);
  //     let yrow = y.row_iter(1);
  //     xrow.union(&yrow);
  //   }
  //
  //   assert_eq!(x.get((1, 0)), true);
  //   assert_eq!(x.get((1, 1)), false);
  //   assert_eq!(x.get((1, 9)), false);
  //   assert_eq!(x.get((1, 10)), true);
  //   assert_eq!(x.get((1, 11)), false);
  //   assert_eq!(x.get((1, 19)), false);
  //   assert_eq!(x.get((1, 20)), true);
  //   assert_eq!(x.get((1, 30)), false);
  //   assert_eq!(x.get((1, 31)), true);
  //   assert_eq!(x.get((1, 32)), false);
  //   assert_eq!(x.get((1, 33)), true);
  //   for i in 0..34 {
  //     assert_eq!(x.get((0, i)), false);
  //     assert_eq!(x.get((2, i)), false);
  //   }
  // }
}
