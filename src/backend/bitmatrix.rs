use std::cmp::PartialEq;
use std::slice::Iter;
use std::slice::IterMut;
use backend::bitpack::Bitpack32;
use backend::bitpack::Bitpack;
use backend::bitvec::BitVec;

#[derive(Debug)]
pub struct BitMatrix2 {
  storage: Vec<Bitpack32>,
  nbits: (usize, usize), // (row, col)
}

pub trait BitMatrix {
  type Index: PartialEq;

  fn get(&self, index: Self::Index) -> bool {
    let (w, b) = self.offset_of(index);
    return self.get_block(w).get(b);
  }

  fn set_true(&mut self, index: Self::Index) {
    let (w, b) = self.offset_of(index);
    return self.get_mut_block(w).set_true(b);
  }

  fn set_false(&mut self, index: Self::Index) {
    let (w, b) = self.offset_of(index);
    return self.get_mut_block(w).set_false(b);
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

  fn process<F>(&mut self, other: &Self, mut op: F)
    where F: FnMut(&mut Bitpack32, &Bitpack32)
  {
    if self.len() != other.len() {
      panic!("self.len should be the same as other.len()");
    }
    for (a, b) in self.get_mut_iter().zip(other.get_iter()) {
      op(a, b);
    }
  }

  fn falses(nbits: Self::Index) -> Self where Self: Sized;
  fn offset_of(&self, index: Self::Index) -> (usize, usize);
  fn len(&self) -> Self::Index;
  fn get_iter(&self) -> Iter<Bitpack32>;
  fn get_mut_iter(&mut self) -> IterMut<Bitpack32>;
  fn get_block(&self, index: usize) -> &Bitpack32;
  fn get_mut_block(&mut self, index: usize) -> &mut Bitpack32;
}

impl BitMatrix for BitMatrix2 {
  type Index = (usize, usize);

  fn falses(nbits: Self::Index) -> Self {
    let (nrow, ncol) = nbits;
    let block_num = nrow * BitMatrix2::block_per_row_of(ncol);
    let mut vec: Vec<Bitpack32> = vec![];
    for _ in 0..block_num {
      vec.push(Bitpack32::falses());
    }
    return BitMatrix2::new(vec, nbits);
  }

  fn offset_of(&self, index: Self::Index) -> (usize, usize) {
    let (nrow, ncol) = self.nbits;
    let (irow, icol) = index;
    if irow >= nrow || icol >= ncol {
      panic!("index should smaller than self.nbits")
    }
    let w: usize = (irow * self.block_per_row()) + icol / Bitpack32::limit_index();
    let b: usize = icol % Bitpack32::limit_index();
    return (w, b);
  }

  // pub fn row(&self, index: usize) -> &BitVec {
  //  let (_, ncol) = self.nbits;
  //  let s = index * self.block_per_row();
  //  let e = s + self.block_per_row();
  //  let v = self.storage[s..e].to_vec();
  //  let bv = BitVec::new(v, ncol);
  //  return &bv;
  // }

  #[inline]
  fn len(&self) -> Self::Index {
    self.nbits
  }

  #[inline]
  fn get_iter(&self) -> Iter<Bitpack32> {
    self.storage.iter()
  }

  #[inline]
  fn get_mut_iter(&mut self) -> IterMut<Bitpack32> {
    self.storage.iter_mut()
  }

  #[inline]
  fn get_block(&self, index: usize) -> &Bitpack32 {
    &self.storage[index]
  }

  #[inline]
  fn get_mut_block(&mut self, index: usize) -> &mut Bitpack32 {
    &mut self.storage[index]
  }
}

impl BitMatrix2 {
  fn new(vec: Vec<Bitpack32>, nbits: <BitMatrix2 as BitMatrix>::Index) -> Self {
    return BitMatrix2 {
      storage: vec,
      nbits: nbits,
    };
  }

  // pub fn row(&self, irow: usize) -> BitVec {
  //   let (nrow, ncol) = self.nbits;
  //   let (w, b) = self.offset_of((irow, 0));
  //   let vec: Vec<Bitpack32> = vec![];
  //
  //   for x in self.storage[w..w + (self.block_per_row())].to_iter() {
  //
  //   }
  //
  //   return BitVec::new(vec, ncol);
  // }

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
