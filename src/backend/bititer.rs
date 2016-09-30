use backend::bitpack::Bitpack32;
use backend::bitpack::Bitpack;

// Cursor ---------------------------------------------------------------
#[derive(Debug)]
pub struct BitIterCursor {
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
    // println!("{:?}", self);
    return self.index >= self.from + (self.length * self.step);
  }

  pub fn next_index(&mut self) -> usize {
    let i = self.index;
    self.index += self.step;
    return i;
  }
}

impl Iterator for BitIterCursor {
  type Item = usize;

  fn next(&mut self) -> Option<Self::Item> {
    if self.finish() {
      return None;
    } else {
      return Some(self.next_index());
    }
  }
}

// Iterator -----------------------------------------------------------
pub struct BitIter<'a> {
  storage: &'a [Bitpack32],
  from: usize,
  length: usize,
  step: usize,
}

impl<'a> BitIter<'a> {
  pub fn new(storage: &'a [Bitpack32], from: usize, length: usize, step: usize) -> BitIter {
    BitIter {
      storage: storage,
      from: from,
      length: length,
      step: step,
    }
  }

  pub fn iter(&self) -> BitIterCursor {
    BitIterCursor::new(self.from, self.length, self.step)
  }
}

pub struct BitIterMut<'a> {
  storage: &'a mut [Bitpack32],
  from: usize,
  length: usize,
  step: usize,
}

impl<'a> BitIterMut<'a> {
  pub fn new(storage: &'a mut [Bitpack32], from: usize, length: usize, step: usize) -> BitIterMut {
    BitIterMut {
      storage: storage,
      from: from,
      length: length,
      step: step,
    }
  }

  fn iter(&self) -> BitIterCursor {
    BitIterCursor::new(self.from, self.length, self.step)
  }

  pub fn union(&mut self, other: &BitIter) {
    self.mut_process(other, |a, b| a.mut_union(b))
  }

  pub fn intersect(&mut self, other: &BitIter) {
    self.mut_process(other, |a, b| a.mut_intersect(b))
  }

  pub fn xor(&mut self, other: &BitIter) {
    self.mut_process(other, |a, b| a.mut_xor(b))
  }

  pub fn mut_process<F>(&mut self, other: &BitIter, mut op: F)
    where F: FnMut(&mut Bitpack32, &Bitpack32)
  {
    let self_cursor = self.iter();
    let mut other_cursor = other.iter();
    if self_cursor.len() != other_cursor.len() {
      panic!("self lenth should be the same as other lenth");
    }
    for i in self_cursor {
      let j = other_cursor.next_index();
      println!("{}, {:?}", i, j);
      op(&mut self.storage[i], &other.storage[j]);
    }
  }
}
