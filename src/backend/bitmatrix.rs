use backend::bitpack::Bitpack32;
use backend::bitpack::Bitpack;

pub struct BitMatrix2 {
  storage: Vec<Bitpack32>,
  nbits: (usize, usize), // (row, col)
}

pub trait BitMatrix {
  type Index;
  fn new(vec: Vec<Bitpack32>, nbits: Self::Index) -> Self where Self: Sized;
  fn falses(nbits: Self::Index) -> Self where Self: Sized;
  fn get(&self, index: Self::Index) -> bool;
  fn set_true(&mut self, index: Self::Index);
  fn set_false(&mut self, index: Self::Index);
  fn len(&self) -> Self::Index;
  fn mut_union(&mut self, other: &Self) where Self: Sized;
  fn mut_intersect(&mut self, other: &Self) where Self: Sized;
  fn mut_xor(&mut self, other: &Self) where Self: Sized;
}

impl BitMatrix for BitMatrix2 {
  type Index = (usize, usize);

  fn new(vec: Vec<Bitpack32>, nbits: Self::Index) -> Self {
    return BitMatrix2 {
      storage: vec,
      nbits: nbits,
    };
  }

  fn falses(nbits: Self::Index) -> Self {
    let (nrow, ncol) = nbits;
    let block_num = nrow * BitMatrix2::block_per_row_of(ncol);
    let mut vec: Vec<Bitpack32> = vec![];
    for _ in 0..block_num {
      vec.push(Bitpack32::falses());
    }
    return BitMatrix2::new(vec, nbits);
  }

  fn get(&self, index: Self::Index) -> bool {
    let (w, b) = self.offset_of(index);
    return self.storage[w].get(b);
  }

  fn set_true(&mut self, index: Self::Index) {
    let (w, b) = self.offset_of(index);
    return self.storage[w].set_true(b);
  }

  fn set_false(&mut self, index: Self::Index) {
    let (w, b) = self.offset_of(index);
    return self.storage[w].set_false(b);
  }

  fn mut_union(&mut self, other: &Self) {
    self.process(other, |a, b| a.mut_union(b))
  }

  fn mut_intersect(&mut self, other: &Self) {
    self.process(other, |a, b| a.mut_intersect(b))
  }

  fn mut_xor(&mut self, other: &Self) {
    self.process(other, |a, b| a.mut_xor(b))
  }

  #[inline]
  fn len(&self) -> Self::Index {
    self.nbits
  }
}

impl BitMatrix2 {
  #[inline]
  fn process<F>(&mut self, other: &Self, mut op: F)
    where F: FnMut(&mut Bitpack32, &Bitpack32)
  {
    assert_eq!(self.len(), other.len());
    for (a, b) in self.storage.iter_mut().zip(other.storage.iter()) {
      op(a, b);
    }
  }

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
    return BitMatrix2::block_per_row_of(ncol);
  }

  #[inline]
  fn block_per_row_of(ncol: usize) -> usize {
    return ncol / Bitpack32::limit_index() + 1;
  }
}


// -------------------------------------------------------------------------------------------------
#[test]
fn offset_of_test() {
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
  assert_eq!(x.storage.len(), 6);
  assert_eq!(x.get((0, 0)), false);
  assert_eq!(x.get((1, 20)), false);
  assert_eq!(x.get((2, 39)), false);
}
