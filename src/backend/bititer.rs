use backend::bitpack::Bitpack32;
use backend::bitpack::Bitpack;

pub struct BitIterMut<'a> {
  storage: &'a mut Vec<Bitpack32>,
  from: usize,
  length: usize,
  step: usize,
  index: usize,
}

impl<'a> BitIterMut<'a> {
  pub fn new(storage: &'a mut Vec<Bitpack32>,
             from: usize,
             length: usize,
             step: usize)
             -> BitIterMut<'a> {
    BitIterMut {
      storage: storage,
      from: from,
      length: length,
      step: step,
      index: from,
    }
  }

  pub fn union(&mut self, other: &[Bitpack32]) {
    self.mut_process(other, |a, b| a.mut_union(b))
  }

  pub fn intersect(&mut self, other: &[Bitpack32]) {
    self.mut_process(other, |a, b| a.mut_intersect(b))
  }

  pub fn xor(&mut self, other: &[Bitpack32]) {
    self.mut_process(other, |a, b| a.mut_xor(b))
  }

  pub fn mut_process<F>(&mut self, other: &[Bitpack32], mut op: F)
    where F: FnMut(&mut Bitpack32, &Bitpack32)
  {
    // TODO
    // if self.len() != other.len() {
    // panic!("self.len should be the same as other.len()");
    // }
    while self.index < self.from + (self.length * self.step) {
      op(&mut self.storage[self.index], &other[self.index]);
      self.index += self.step;
    }
  }
}
