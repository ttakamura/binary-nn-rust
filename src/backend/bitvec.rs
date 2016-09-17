const U32_SIZE: usize = 32;

pub struct BitVec {
  storage: Vec<u32>,
  nbits: usize,
}

impl BitVec {
  pub fn new(vec: Vec<u32>, nbits: usize) -> BitVec {
    return BitVec {
      storage: vec,
      nbits: nbits,
    };
  }

  pub fn from_bool(vec: Vec<bool>) -> BitVec {
    let nbits = vec.len();
    let int_vec = bool_to_int_vec(vec);
    return BitVec {
      storage: int_vec,
      nbits: nbits,
    };
  }

  pub fn falses(nbits: usize) -> BitVec {
    let tmp: Vec<bool> = vec![false; nbits];
    return BitVec::from_bool(tmp);
  }

  pub fn get(&self, index: usize) -> Option<bool> {
    if index >= self.nbits {
      return None;
    }
    let (w, b) = offset_of(index);
    let x = self.storage[w];
    return Some(bit_to_bool(x, b));
  }

  pub fn set_true(&mut self, index: usize) {
    if index >= self.nbits {
      panic!("index should smaller than self.nbits")
    }
    let (w, b) = offset_of(index);
    self.storage[w] |= single_bit_int(b);
  }

  pub fn set_false(&mut self, index: usize) {
    if index >= self.nbits {
      panic!("index should smaller than self.nbits")
    }
    let (w, b) = offset_of(index);
    self.storage[w] &= !(single_bit_int(b));
  }

  pub fn mut_union(&mut self, other: &BitVec) {
    self.process(other, |a, b| a | b)
  }

  pub fn mut_intersect(&mut self, other: &BitVec) {
    self.process(other, |a, b| a & b)
  }

  pub fn mut_xor(&mut self, other: &BitVec) {
    self.process(other, |a, b| a ^ b)
  }

  #[inline]
  pub fn len(&self) -> usize {
    self.nbits
  }

  #[inline]
  fn process<F>(&mut self, other: &BitVec, mut op: F)
    where F: FnMut(u32, u32) -> u32
  {
    assert_eq!(self.len(), other.len());
    for (a, b) in self.storage.iter_mut().zip(other.storage.iter()) {
      *a = op(*a, *b);
    }
  }
}

// #[inline]
// fn block_of(index: usize) -> usize {
//  index / U32_SIZE
// }

#[inline]
fn bool_to_int_vec(vec: Vec<bool>) -> Vec<u32> {
  let mut int_vec: Vec<u32> = vec![];
  for block in vec.chunks(U32_SIZE) {
    let mut tmp: u32 = 0;
    for i in 0..block.len() {
      if block[i] {
        tmp = tmp | single_bit_int(i)
      }
    }
    int_vec.push(tmp);
  }
  return int_vec;
}

#[inline]
fn offset_of(index: usize) -> (usize, usize) {
  let w: usize = index / U32_SIZE;
  let b: usize = index % U32_SIZE;
  return (w, b);
}

#[inline]
fn bit_to_bool(x: u32, index: usize) -> bool {
  if index >= U32_SIZE {
    panic!("index should smaller than 32");
  }
  (x & single_bit_int(index)) != 0
}

#[inline]
fn single_bit_int(index: usize) -> u32 {
  (1 << (U32_SIZE - (index + 1)))
}


// -------------------------------------------------------------------------------------------------

#[test]
fn bool_to_int_vec_test() {
  assert_eq!(bool_to_int_vec(vec![true, false]), vec![1 << 31]);
  assert_eq!(bool_to_int_vec(vec![true, true, true]), vec![7 << 29]);
  assert_eq!(bool_to_int_vec(vec![false, false, true]), vec![1 << 29]);

  let mut vec: Vec<bool> = vec![];
  for i in 0..33 {
    if i < 31 {
      vec.push(false);
    } else {
      vec.push(true);
    }
  }
  assert_eq!(bool_to_int_vec(vec), vec![1, 1 << 31]);
}

#[test]
fn offset_of_test() {
  assert_eq!(offset_of(0), (0, 0));
  assert_eq!(offset_of(1), (0, 1));
  assert_eq!(offset_of(31), (0, 31));
  assert_eq!(offset_of(32), (1, 0));
  assert_eq!(offset_of(34), (1, 2));
  assert_eq!(offset_of(63), (1, 31));
  assert_eq!(offset_of(64), (2, 0));
}

#[test]
fn bit_to_bool_test() {
  assert_eq!(bit_to_bool(1 << 30, 0), false);
  assert_eq!(bit_to_bool(1 << 30, 1), true);
  assert_eq!(bit_to_bool(1 << 30, 2), false);

  assert_eq!(bit_to_bool(2, 29), false);
  assert_eq!(bit_to_bool(2, 30), true);
  assert_eq!(bit_to_bool(2, 31), false);

  assert_eq!(bit_to_bool(1, 30), false);
  assert_eq!(bit_to_bool(1, 31), true);

  assert_eq!(bit_to_bool(0, 0), false);
  assert_eq!(bit_to_bool(0, 31), false);
}

#[test]
#[should_panic(expected = "index should smaller than 32")]
fn bit_to_bool_panic_test() {
  assert!(bit_to_bool(0, 32));
}

// -------------------------------------------------------------------------------------------------
