pub trait Bitpack: Clone {
  fn limit_index() -> u32;
  fn from_bool(vec: Vec<bool>) -> Self where Self: Sized;
  fn falses() -> Self where Self: Sized;
  fn trues() -> Self where Self: Sized;
  fn get(&self, index: u32) -> bool;
  fn set_true(&mut self, index: u32);
  fn set_false(&mut self, index: u32);
  fn mut_union(&mut self, other: &Self);
  fn mut_intersect(&mut self, other: &Self);
  fn mut_xor(&mut self, other: &Self);
  fn mut_xnor(&mut self, other: &Self);
  fn count_ones(&self) -> u32;
  fn count_ones_mask(&self, mask_after: u32) -> u32;

  fn xnor(&self, other: &Self) -> Self
    where Self: Sized
  {
    let mut x = (*self).clone();
    x.mut_xnor(other);
    return x;
  }

  fn xor(&self, other: &Self) -> Self
    where Self: Sized
  {
    let mut x = (*self).clone();
    x.mut_xor(other);
    return x;
  }

  fn union(&self, other: &Self) -> Self
    where Self: Sized
  {
    let mut x = (*self).clone();
    x.mut_union(other);
    return x;
  }

  fn intersect(&self, other: &Self) -> Self
    where Self: Sized
  {
    let mut x = (*self).clone();
    x.mut_intersect(other);
    return x;
  }

  fn pretty_str(&self) -> String {
    let mut v: Vec<String> = vec![];
    for i in 0..Self::limit_index() {
      v.push(if self.get(i) {
        "1".to_owned()
      } else {
        "0".to_owned()
      });
      if (i % 10) == 9 {
        v.push(" ".to_owned());
      }
    }
    return v.concat();
  }
}

#[derive(Clone,Debug)]
pub struct Bitpack32 {
  storage: u32,
}

impl Bitpack32 {
  pub fn new(val: u32) -> Bitpack32 {
    Bitpack32 { storage: val }
  }

  #[inline]
  fn check_index(index: u32) {
    if index >= Bitpack32::limit_index() {
      panic!("index should smaller than limit_index");
    }
  }
}

impl Bitpack for Bitpack32 {
  #[inline]
  fn limit_index() -> u32 {
    32
  }

  fn from_bool(vec: Vec<bool>) -> Bitpack32 {
    let val = bool_to_int(vec);
    return Bitpack32 { storage: val };
  }

  fn falses() -> Bitpack32 {
    let tmp: Vec<bool> = vec![false; Bitpack32::limit_index() as usize];
    return Bitpack32::from_bool(tmp);
  }

  fn get(&self, index: u32) -> bool {
    return bit_to_bool(self.storage, index);
  }

  fn set_true(&mut self, index: u32) {
    self.storage |= single_bit_int(index);
  }

  fn set_false(&mut self, index: u32) {
    self.storage &= !(single_bit_int(index));
  }

  fn mut_union(&mut self, other: &Bitpack32) {
    self.storage = self.storage | other.storage;
  }

  fn mut_intersect(&mut self, other: &Bitpack32) {
    self.storage = self.storage & other.storage;
  }

  fn mut_xor(&mut self, other: &Bitpack32) {
    self.storage = self.storage ^ other.storage;
  }

  fn mut_xnor(&mut self, other: &Bitpack32) {
    self.storage = !(self.storage ^ other.storage);
  }

  fn count_ones(&self) -> u32 {
    self.storage.count_ones()
  }

  fn count_ones_mask(&self, mask_after: u32) -> u32 {
    self.storage.count_ones()
  }
}

#[inline]
fn bool_to_int(vec: Vec<bool>) -> u32 {
  let mut tmp: u32 = 0;
  for i in 0..vec.len() {
    if vec[i] {
      tmp = tmp | single_bit_int(i as u32)
    }
  }
  return tmp;
}

#[inline]
fn bit_to_bool(x: u32, index: u32) -> bool {
  Bitpack32::check_index(index);
  (x & single_bit_int(index)) != 0
}

#[inline]
fn single_bit_int(index: u32) -> u32 {
  Bitpack32::check_index(index);
  (1 << (Bitpack32::limit_index() - (index + 1)))
}

// -------------------------------------------------------------------------------------------------
#[test]
fn bool_to_int_test() {
  assert_eq!(bool_to_int(vec![true, false]), 1 << 31);
  assert_eq!(bool_to_int(vec![true, true, true]), 7 << 29);
  assert_eq!(bool_to_int(vec![false, false, true]), 1 << 29);
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
#[should_panic(expected = "index should smaller than limit_index")]
fn bit_to_bool_panic_test() {
  assert!(bit_to_bool(0, 32));
}
