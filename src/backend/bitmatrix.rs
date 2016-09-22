use backend::bitpack::Bitpack32;
use backend::bitpack::Bitpack;

pub struct BitMatrix {
  storage: Vec<Bitpack32>,
  nbits: (usize, usize), // (row, col)
}

impl BitMatrix {
  pub fn new(vec: Vec<Bitpack32>, nbits: (usize, usize)) -> BitMatrix {
    return BitMatrix {
      storage: vec,
      nbits: nbits,
    };
  }

  pub fn falses(nbits: (usize, usize)) -> BitMatrix {
    let (nrow, ncol) = nbits;
    let block_num = (nrow * ncol) / Bitpack32::limit_index() + 1;
    let mut vec: Vec<Bitpack32> = vec![];
    for _ in 0..block_num {
      vec.push(Bitpack32::falses());
    }
    return BitMatrix::new(vec, nbits);
  }

  pub fn get(&self, index: (usize, usize)) -> bool {
    let (w, b) = self.offset_of(index);
    return self.storage[w].get(b);
  }

  // pub fn set_true(&mut self, index: usize) {
  //   let (w, b) = self.offset_of(index);
  //   return self.storage[w].set_true(b);
  // }
  //
  // pub fn set_false(&mut self, index: usize) {
  //   let (w, b) = self.offset_of(index);
  //   return self.storage[w].set_false(b);
  // }
  //
  // pub fn mut_union(&mut self, other: &BitVec) {
  //   self.process(other, |a, b| a.mut_union(b))
  // }
  //
  // pub fn mut_intersect(&mut self, other: &BitVec) {
  //   self.process(other, |a, b| a.mut_intersect(b))
  // }
  //
  // pub fn mut_xor(&mut self, other: &BitVec) {
  //   self.process(other, |a, b| a.mut_xor(b))
  // }
  //
  // #[inline]
  // pub fn len(&self) -> usize {
  //   self.nbits
  // }
  //
  // #[inline]
  // fn process<F>(&mut self, other: &BitVec, mut op: F)
  //   where F: FnMut(&mut Bitpack32, &Bitpack32)
  // {
  //   assert_eq!(self.len(), other.len());
  //   for (a, b) in self.storage.iter_mut().zip(other.storage.iter()) {
  //     op(a, b);
  //   }
  // }

  #[inline]
  fn offset_of(&self, index: (usize, usize)) -> (usize, usize) {
    let (nrow, ncol) = self.nbits;
    let (irow, icol) = index;
    if irow >= nrow || icol >= ncol {
      panic!("index should smaller than self.nbits")
    }
    let w: usize = (irow * self.block_per_row()) + icol / Bitpack32::limit_index();
    let b: usize = icol % Bitpack32::limit_index();
    return (w, b);
  }

  #[inline]
  fn block_per_row(&self) -> usize {
    let (_, ncol) = self.nbits;
    return ncol / Bitpack32::limit_index() + 1;
  }
}

// -------------------------------------------------------------------------------------------------
#[test]
fn offset_of_test() {
  let x = BitMatrix::falses((3, 35));
  assert_eq!(x.offset_of((0, 0)), (0, 0));
  assert_eq!(x.offset_of((0, 10)), (0, 10));
  assert_eq!(x.offset_of((0, 32 + 1)), (1, 1));
  assert_eq!(x.offset_of((0, 32 + 2)), (1, 2));
  assert_eq!(x.offset_of((1, 5)), (2, 5));
  assert_eq!(x.offset_of((1, 32 + 2)), (3, 2));
  assert_eq!(x.offset_of((2, 5)), (4, 5));
  assert_eq!(x.offset_of((2, 32 + 2)), (5, 2));
}
