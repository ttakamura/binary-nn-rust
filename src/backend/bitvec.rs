use backend::bitpack::Bitpack32;
use backend::bitpack::Bitpack;

#[derive(Debug)]
pub struct BitVec {
  storage: Vec<Bitpack32>,
  nbits: usize,
}

impl BitVec {
  pub fn new(vec: Vec<Bitpack32>, nbits: usize) -> BitVec {
    return BitVec {
      storage: vec,
      nbits: nbits,
    };
  }

  pub fn falses(nbits: usize) -> BitVec {
    let block_num = nbits / Bitpack32::limit_index() + 1;
    let mut vec: Vec<Bitpack32> = vec![];
    for _ in 0..block_num {
      vec.push(Bitpack32::falses());
    }
    return BitVec::new(vec, nbits);
  }

  pub fn get(&self, index: usize) -> bool {
    let (w, b) = self.offset_of(index);
    return self.storage[w].get(b);
  }

  pub fn set_true(&mut self, index: usize) {
    let (w, b) = self.offset_of(index);
    return self.storage[w].set_true(b);
  }

  pub fn set_false(&mut self, index: usize) {
    let (w, b) = self.offset_of(index);
    return self.storage[w].set_false(b);
  }

  pub fn mut_union(&mut self, other: &BitVec) {
    self.process(other, |a, b| a.mut_union(b))
  }

  pub fn mut_intersect(&mut self, other: &BitVec) {
    self.process(other, |a, b| a.mut_intersect(b))
  }

  pub fn mut_xor(&mut self, other: &BitVec) {
    self.process(other, |a, b| a.mut_xor(b))
  }

  #[inline]
  pub fn len(&self) -> usize {
    self.nbits
  }

  #[inline]
  fn process<F>(&mut self, other: &BitVec, mut op: F)
    where F: FnMut(&mut Bitpack32, &Bitpack32)
  {
    assert_eq!(self.len(), other.len());
    for (a, b) in self.storage.iter_mut().zip(other.storage.iter()) {
      op(a, b);
    }
  }

  #[inline]
  fn offset_of(&self, index: usize) -> (usize, usize) {
    if index >= self.nbits {
      panic!("index should smaller than self.nbits")
    }
    let w: usize = index / Bitpack32::limit_index();
    let b: usize = index % Bitpack32::limit_index();
    return (w, b);
  }
}

// -------------------------------------------------------------------------------------------------
#[test]
fn offset_of_test() {
  let x = BitVec::new(vec![Bitpack32::falses(), Bitpack32::falses()], 40);
  assert_eq!(x.offset_of(0), (0, 0));
  assert_eq!(x.offset_of(1), (0, 1));
  assert_eq!(x.offset_of(31), (0, 31));
  assert_eq!(x.offset_of(32), (1, 0));
  assert_eq!(x.offset_of(34), (1, 2));
  assert_eq!(x.offset_of(39), (1, 7));
}
