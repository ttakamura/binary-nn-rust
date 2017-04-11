use std::f32;
use std::io::prelude::*;
use std::fs::File;
use std::error::Error;
use std::mem;

use backend::bitmatrix::BitMatrix2;
use backend::bitvec::BitVec;
use backend::bitmatrix_trait::BitMatrixMut;

pub fn load_text_as_i32(path: &str) -> Vec<i32> {
  let buffer = load_text(path);
  let mut vec = Vec::new();
  for line in buffer.lines() {
    vec.push(line.parse::<i32>().unwrap());
  }
  return vec;
}

pub fn load_text_as_f32(path: &str) -> Vec<f32> {
  let buffer = load_text(path);
  let mut vec = Vec::new();
  for line in buffer.lines() {
    vec.push(line.parse::<f32>().unwrap());
  }
  return vec;
}

pub fn load_f32_as_bitvec(path: &str, nbits: u32) -> BitVec {
  let f32_vec = load_f32(path);
  return load_as_bitvec(&f32_vec, 0, nbits);
}

pub fn load_f32_as_bitmatrix(path: &str, nrow: u32, ncol: u32) -> BitMatrix2 {
  let f32_vec = load_f32(path);
  let mut bit_mat = BitMatrix2::falses((nrow, ncol));
  let mut idx = 0;
  for irow in 0..nrow {
    for icol in 0..ncol {
      set_bit(f32_vec[idx], &mut bit_mat, (irow, icol));
      idx += 1
    }
  }
  return bit_mat;
}

pub fn load_f32(path: &str) -> Vec<f32> {
  let buffer: Vec<u8> = load_binary(path);
  let mut result: Vec<f32> = Vec::new();
  for chunk in buffer.chunks(4) {
    result.push(pack(chunk));
  }
  return result;
}

pub fn load_binary(path: &str) -> Vec<u8> {
  let mut buffer: Vec<u8> = Vec::new();
  let mut f = match File::open(path) {
    Ok(file) => file,
    Err(why) => panic!("couldn't open {}, {}", path, Error::description(&why)),
  };
  match f.read_to_end(&mut buffer) {
    Ok(_) => println!("success load"),
    Err(why) => panic!("couldn't read {}, {}", path, Error::description(&why)),
  };
  return buffer;
}

// -- PRIVATE --

fn load_text(path: &str) -> String {
  let mut buffer = String::new();
  let mut f = match File::open(path) {
    Ok(file) => file,
    Err(why) => panic!("Couldn't open {}, {}", path, Error::description(&why)),
  };
  match f.read_to_string(&mut buffer) {
    Ok(_) => {}
    Err(why) => panic!("Couldn't read {}, {}", path, Error::description(&why)),
  };
  return buffer;
}

fn load_as_bitvec(f32_vec: &Vec<f32>, offset: u32, nbits: u32) -> BitVec {
  let mut bit_vec = BitVec::falses(nbits);
  for idx in 0..nbits {
    set_bit(f32_vec[(idx + offset) as usize], &mut bit_vec, idx);
  }
  return bit_vec;
}

fn set_bit<T>(value: f32, mat: &mut T, index: T::Index)
  where T: BitMatrixMut
{
  if value > 0.0 {
    mat.set_true(index);
  } else {
    mat.set_false(index);
  }
}

fn pack(buffer: &[u8]) -> f32 {
  let mut buf: [u8; 4] = [0; 4];
  buf[0] = buffer[0];
  buf[1] = buffer[1];
  buf[2] = buffer[2];
  buf[3] = buffer[3];
  let x: f32 = unsafe { mem::transmute(buf) };
  //  println!("x is {:#?}", x);
  return x;
}
