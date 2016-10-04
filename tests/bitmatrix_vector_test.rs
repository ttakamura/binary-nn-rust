extern crate binary_nn;

mod bitmatrix_vector_tests {
  use binary_nn::backend::bitmatrix::*;
  use binary_nn::backend::bitvec::*;
  use binary_nn::backend::bitmatrix_trait::*;

  fn prepare_matrix() -> BitMatrix2 {
    let mut x = BitMatrix2::falses((3, 34));
    x.set_true((0, 28));
    x.set_true((0, 29));
    x.set_true((1, 30));
    x.set_true((1, 31));
    x.set_true((2, 32));
    x.set_true((2, 33));
    return x;
  }

  fn prepare_vector() -> BitVec {
    let mut y = BitVec::falses(34);
    y.set_true(29);
    y.set_true(30);
    return y;
  }

  #[test]
  fn bitmatrix_xnor_vector() {
    let mut x = prepare_matrix();
    let y = prepare_vector();

    x.mut_row_iter(0).xnor(&y.iter());
    x.mut_row_iter(1).xnor(&y.iter());
    x.mut_row_iter(2).xnor(&y.iter());

    assert_eq!(x.get((0, 28)), false);
    assert_eq!(x.get((0, 29)), true);
    assert_eq!(x.get((0, 30)), false);
    assert_eq!(x.get((0, 31)), true);

    assert_eq!(x.get((1, 28)), true);
    assert_eq!(x.get((1, 29)), false);
    assert_eq!(x.get((1, 30)), true);
    assert_eq!(x.get((1, 31)), false);

    assert_eq!(x.get((2, 28)), true);
    assert_eq!(x.get((2, 29)), false);
    assert_eq!(x.get((2, 30)), false);
    assert_eq!(x.get((2, 31)), true);
    assert_eq!(x.get((2, 32)), false);
    assert_eq!(x.get((2, 33)), false);
  }
}
