extern crate binary_nn;

mod bititer_tests {
  use binary_nn::backend::bititer::*;
  use binary_nn::backend::bitpack::*;

  #[test]
  fn bititer_cursor() {
    let x = vec![Bitpack32::falses(), Bitpack32::falses(), Bitpack32::falses()];
    let xi = BitIter::new(&x[..], 0, 3, 1);
    let mut c = xi.iter();

    assert_eq!(c.len(), 3);
    assert_eq!(c.finish(), false);

    assert_eq!(c.next(), Some(0));
    assert_eq!(c.finish(), false);

    assert_eq!(c.next(), Some(1));
    assert_eq!(c.finish(), false);

    assert_eq!(c.next(), Some(2));
    assert_eq!(c.finish(), true);

    assert_eq!(c.next(), None);
  }

  #[test]
  fn bititer_cursor_row() {
    // Matrix(3 x 3).row(1)
    let mut c = BitIterCursor::new(3, 3, 1);
    assert_eq!(c.len(), 3);
    assert_eq!(c.finish(), false);

    assert_eq!(c.next(), Some(3));
    assert_eq!(c.finish(), false);

    assert_eq!(c.next(), Some(4));
    assert_eq!(c.finish(), false);

    assert_eq!(c.next(), Some(5));
    assert_eq!(c.finish(), true);

    assert_eq!(c.next(), None);
  }

  #[test]
  fn bititer_cursor_col() {
    // Matrix(3 x 3).col(1)
    let mut c = BitIterCursor::new(0, 3, 3);
    assert_eq!(c.len(), 3);
    assert_eq!(c.finish(), false);

    assert_eq!(c.next(), Some(0));
    assert_eq!(c.finish(), false);

    assert_eq!(c.next(), Some(3));
    assert_eq!(c.finish(), false);

    assert_eq!(c.next(), Some(6));
    assert_eq!(c.finish(), true);

    assert_eq!(c.next(), None);
  }

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
      let mut xi = BitIterMut::new(&mut x[..], 0, 2, 1);
      let yi = BitIter::new(&y[..], 0, 2, 1);
      xi.union(&yi);
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
