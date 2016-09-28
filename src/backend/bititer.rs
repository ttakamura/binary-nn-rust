use backend::bitpack::Bitpack32;
use backend::bitpack::Bitpack;

// Cursor ---------------------------------------------------------------
struct BitIterCursor {
  from: usize,
  length: usize,
  step: usize,
  index: usize,
}

impl BitIterCursor {
  // Matrix(5 x 10).row(4) => from: 40, length: 10, step: 1
  // Matrix(5 x 10).col(9) => from: 9, length: 5, step: 10
  pub fn new(from: usize, length: usize, step: usize) -> BitIterCursor {
    BitIterCursor {
      from: from,
      length: length,
      step: step,
      index: from,
    }
  }

  pub fn len(&self) -> usize {
    self.length
  }

  pub fn finish(&self) -> bool {
    self.index < self.from + (self.length * self.step)
  }

  pub fn next(&mut self) -> usize {
    let i = self.index;
    self.index += self.step;
    return i;
  }
}

// Iterator -----------------------------------------------------------
pub struct BitIter<'a> {
  storage: &'a [Bitpack32],
  cursor: BitIterCursor,
}

impl<'a> Iterator for BitIter<'a> {
  type Item = &'a Bitpack32;

  fn next(&mut self) -> Option<&'a Bitpack32> {
    if self.cursor.finish() {
      return None;
    } else {
      return Some(&self.storage[self.cursor.next()]);
    }
  }
}

pub struct BitIterMut<'a, 'b> {
  storage: &'b mut [Bitpack32],
  cursor: &'a BitIterCursor,
}

impl<'a, 'b> Iterator for BitIterMut<'a, 'b> {
  type Item = &'b mut Bitpack32;

  fn next(&'a mut self) -> Option<&'b mut Bitpack32> {
    if self.cursor.finish() {
      return None;
    } else {
      return Some(&mut self.storage[self.cursor.next()]);
    }
  }
}

impl<'a, 'b> BitIterMut<'a, 'b> {
  pub fn xor(&mut self, other: BitIter) {
    self.mut_process(other, |a, b| a.mut_xor(b))
  }

  pub fn mut_process<F>(&mut self, other: BitIter, mut op: F)
    where F: FnMut(&mut Bitpack32, &Bitpack32)
  {
    if self.cursor.len() != other.cursor.len() {
      panic!("self.len should be the same as other.len()");
    }
    for _ in 0..self.cursor.len() {
      op(self.next(), other.next());
    }
  }
}
