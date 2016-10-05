extern crate binary_nn;

mod bititer_tests {
  use binary_nn::backend::bititer::*;
  use binary_nn::backend::bitpack::*;

  #[test]
  fn bititer_union() {
    let mut x = vec![Bitpack32::falses(), Bitpack32::falses()];
    let mut y = vec![Bitpack32::falses(), Bitpack32::falses()];
    x[0].set_true(0);
    y[0].set_true(10);
    y[0].set_true(31);
    x[1].set_true(1);
    y[1].set_true(20);
    y[1].set_true(31);

    {
      let mut xi = BitIterMut::new((&mut x[..]).into_iter());
      let mut yi = BitIter::new((&y[..]).into_iter());
      xi.union(yi);
    }

    for j in 0..2 {
      for i in 0..32 {
        match (j, i) {
          (0, 0) => assert_eq!(x[j].get(i), true),
          (0, 10) => assert_eq!(x[j].get(i), true),
          (0, 31) => assert_eq!(x[j].get(i), true),
          (1, 1) => assert_eq!(x[j].get(i), true),
          (1, 20) => assert_eq!(x[j].get(i), true),
          (1, 31) => assert_eq!(x[j].get(i), true),
          _ => assert_eq!(x[j].get(i), false),
        }
      }
    }
  }
}
