use backend::bitpack::Bitpack32;
use backend::bitpack::Bitpack;
use backend::bitmatrix_trait::*;
use backend::bititer::*;

#[derive(Debug)]
pub struct BitMatrix2 {
  storage: Vec<Bitpack32>,
  nbits: (u32, u32), // (row, col)
}

impl BitMatrix for BitMatrix2 {
  type Index = (u32, u32);

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

  fn iter(&self) -> BitIter {
    let (nrow, ncol) = self.nbits;
    return BitIter::new(self.as_slice().into_iter(), nrow * ncol);
  }
}

impl BitMatrixMut for BitMatrix2 {
  fn as_mut_slice(&mut self) -> &mut [Bitpack32] {
    &mut self.storage[..]
  }

  fn mut_iter(&mut self) -> BitIterMut {
    let (nrow, ncol) = self.nbits;
    return BitIterMut::new(self.as_mut_slice().into_iter(), nrow * ncol);
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
  // pub fn row_iter(&self, irow: u32) -> BitIter {
  //   let (w, _) = self.offset_of((irow, 0));
  //   return BitIter::new(self.as_slice(), w, self.block_per_row(), 1, 1);
  // }
  //
  // pub fn mut_row_iter(&mut self, irow: u32) -> BitIterMut {
  //   let (w, _) = self.offset_of((irow, 0));
  //   let length = self.block_per_row();
  //   return BitIterMut::new(self.as_mut_slice(), w, length, 1, 1);
  // }

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
