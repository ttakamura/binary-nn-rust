use std::iter::FromIterator;
use backend::bitpack::Bitpack32;
use backend::bitpack::Bitpack;
use backend::bitmatrix_trait::*;
use backend::bititer::*;
use backend::bitvec::*;

#[derive(Debug)]
pub struct BitMatrix2 {
  storage: Vec<Bitpack32>,
  nbits: (u32, u32), // (row, col)
}

impl BitMatrix for BitMatrix2 {
  type Index = (u32, u32);

  fn new(vec: Vec<Bitpack32>, nbits: Self::Index) -> Self {
    return BitMatrix2 {
      storage: vec,
      nbits: nbits,
    };
  }

  fn offset_of(&self, index: Self::Index) -> (u32, u32) {
    let (nrow, ncol) = self.nbits;
    let (irow, icol) = index;
    if irow >= nrow || icol >= ncol {
      panic!("index should smaller than self.nbits")
    }
    let w = (irow * self.block_per_row()) + icol / Bitpack32::limit_index();
    let b = icol % Bitpack32::limit_index();
    return (w, b);
  }

  fn len(&self) -> Self::Index {
    self.nbits
  }

  fn block_len(&self) -> u32 {
    let (nrow, _) = self.nbits;
    return nrow * self.block_per_row();
  }

  fn as_slice(&self) -> &[Bitpack32] {
    &self.storage[..]
  }
}

impl BitMatrixMut for BitMatrix2 {
  fn as_mut_slice(&mut self) -> &mut [Bitpack32] {
    &mut self.storage[..]
  }
}

impl BitMatrix2 {
  pub fn falses(nbits: <BitMatrix2 as BitMatrix>::Index) -> Self {
    let (nrow, ncol) = nbits;
    let block_num = nrow * BitMatrix2::block_per_row_of(ncol);
    let mut vec: Vec<Bitpack32> = vec![];
    for _ in 0..block_num {
      vec.push(Bitpack32::falses());
    }
    return BitMatrix2::new(vec, nbits);
  }

  pub fn row_iter(&self, irow: u32) -> BitIter<u32> {
    let (_, ncol) = self.nbits;
    let (w, _) = self.offset_of((irow, 0));
    let ew = w + self.block_per_row();
    let slice = &self.storage[w as usize..ew as usize];
    return BitIter::new(slice.into_iter(), ncol);
  }

  pub fn row_vec(&self, irow: u32) -> BitVec {
    BitVec::from_iter(self.row_iter(irow))
  }

  pub fn from_iter<I>(iter: I) -> Self
    where I: BitIterator<Item = Bitpack32, Index = (u32, u32)>
  {
    let nbits = iter.shape().clone();
    Self::new(Vec::from_iter(iter), nbits)
  }

  pub fn iter(&self) -> BitIter<(u32, u32)> {
    return BitIter::new(self.as_slice().into_iter(), self.nbits);
  }

  // TODO: BitVec の iter をリピートしてフラットにできれば、もっと効率よくなる？
  pub fn dot_vec(&self, other: &BitVec) -> Vec<u32> {
    let (nrow, _) = self.nbits;
    return (0..nrow).map(|i| self.row_vec(i).dot(&other)).collect();
  }

  #[inline]
  fn block_per_row(&self) -> u32 {
    let (_, ncol) = self.nbits;
    return BitMatrix2::block_per_row_of(ncol);
  }

  #[inline]
  fn block_per_row_of(ncol: u32) -> u32 {
    return ncol / Bitpack32::limit_index() + 1;
  }
}
