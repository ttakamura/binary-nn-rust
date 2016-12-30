extern crate binary_nn;

mod bitmatrix_vector_tests {
  use binary_nn::backend::bititer::*;
  use binary_nn::backend::bitmatrix::*;
  use binary_nn::backend::bitvec::*;
  use binary_nn::backend::bitpack::*;
  use binary_nn::backend::bitmatrix_trait::*;

  fn prepare_matrix() -> (BitMatrix2, BitVec) {
    let mut x = BitMatrix2::falses((3, 34));
    let mut y = BitVec::falses(34);
    x.set_true((0, 9));
    x.set_true((1, 30));
    x.set_true((1, 31));
    x.set_true((2, 20));
    y.set_true(10);
    y.set_true(30);
    return (x, y);
  }

  #[test]
  fn bitmatrix_row_vec_union() {
    let (mut x, y) = prepare_matrix();
    let z0 = BitVec::from_iter(x.row_iter(0).union(&y.iter()));
    assert_eq!(z0.get(9), true);
    assert_eq!(z0.get(10), true);
    assert_eq!(z0.get(11), false);
    assert_eq!(z0.get(29), false);
    assert_eq!(z0.get(30), true);
    assert_eq!(z0.get(31), false);
    let z1 = BitVec::from_iter(x.row_iter(1).union(&y.iter()));
    assert_eq!(z1.get(9), false);
    assert_eq!(z1.get(10), true);
    assert_eq!(z1.get(11), false);
    assert_eq!(z1.get(29), false);
    assert_eq!(z1.get(30), true);
    assert_eq!(z1.get(31), true);
  }

  #[test]
  fn bitmatrix_row_vec_xnor() {
    let (mut x, y) = prepare_matrix();
    let z1 = BitVec::from_iter(x.row_iter(1).xnor(&y.iter()));
    assert_eq!(z1.get(9), true);
    assert_eq!(z1.get(10), false);
    assert_eq!(z1.get(11), true);
    assert_eq!(z1.get(29), true);
    assert_eq!(z1.get(30), true);
    assert_eq!(z1.get(31), false);
  }

  #[test]
  fn bitmatrix_row_vec_dot() {
    let (mut x, y) = prepare_matrix();
    let z = BitVec::from_iter(x.row_iter(1).xnor(&y.iter()));
    assert_eq!(z.block(0).pretty_str(),
               "1111111111 0111111111 1111111111 10");
    assert_eq!(z.block(1).pretty_str(),
               "1111111111 1111111111 1111111111 11");
    // let total = z.count_ones();
    // assert_eq!(total, 32);
  }
}
