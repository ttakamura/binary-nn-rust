use backend::bitpack::Bitpack32;
use backend::bitpack::Bitpack;
use backend::bitmatrix_trait::*;

#[derive(Debug)]
pub struct BitMatrix2 {
  storage: Vec<Bitpack32>,
  nbits: (usize, usize), // (row, col)
}

impl BitMatrix for BitMatrix2 {
  type Index = (usize, usize);

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

  fn len(&self) -> Self::Index {
    self.nbits
  }

  fn block_len(&self) -> usize {
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
  pub fn new(vec: Vec<Bitpack32>, nbits: <BitMatrix2 as BitMatrix>::Index) -> Self {
    return BitMatrix2 {
      storage: vec,
      nbits: nbits,
    };
  }

  pub fn falses(nbits: <BitMatrix2 as BitMatrix>::Index) -> Self {
    let (nrow, ncol) = nbits;
    let block_num = nrow * BitMatrix2::block_per_row_of(ncol);
    let mut vec: Vec<Bitpack32> = vec![];
    for _ in 0..block_num {
      vec.push(Bitpack32::falses());
    }
    return BitMatrix2::new(vec, nbits);
  }

  // TODO
  // pub fn row_iter(&self, irow: usize) -> BitIter {
  //   let (w, _) = self.offset_of((irow, 0));
  //   return BitIter::new(self.as_slice(), w, self.block_per_row(), 1, 1);
  // }
  //
  // pub fn mut_row_iter(&mut self, irow: usize) -> BitIterMut {
  //   let (w, _) = self.offset_of((irow, 0));
  //   let length = self.block_per_row();
  //   return BitIterMut::new(self.as_mut_slice(), w, length, 1, 1);
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
