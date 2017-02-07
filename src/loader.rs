use std::f32;
use std::io::prelude::*;
use std::fs::File;
use std::error::Error;
use std::mem;

extern crate binary_nn;
use binary_nn::backend::bitmatrix::BitMatrix2;
use binary_nn::backend::bitvec::BitVec;
use binary_nn::backend::bitmatrix_trait::*;

pub struct BatchNormWeight {
  pub avg_mean: Vec<f32>,
  pub avg_var: Vec<f32>,
  pub beta: Vec<f32>,
  pub gamma: Vec<f32>,
}

pub fn load_batch_norm_weight(path: String, nrow: usize) -> BatchNormWeight {
  let mut avg_mean = load_f32(path);
  let mut avg_var = avg_mean.split_off(nrow);
  let mut beta = avg_var.split_off(nrow);
  let gamma = beta.split_off(nrow);
  return BatchNormWeight {
    avg_mean: avg_mean,
    avg_var: avg_var,
    beta: beta,
    gamma: gamma,
  };
}

pub fn load_f32_as_bitvec(path: String, nbits: u32) -> BitVec {
  let f32_vec = load_f32(path);
  return load_vec(&f32_vec, 0, nbits);
}

pub fn load_f32_as_bitmatrix(path: String, nrow: u32, ncol: u32) -> BitMatrix2 {
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

// -- PRIVATE --

fn load_vec(f32_vec: &Vec<f32>, offset: u32, nbits: u32) -> BitVec {
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

fn load_f32(path: String) -> Vec<f32> {
  let mut buffer: Vec<u8> = Vec::new();

  let mut f = match File::open(path) {
    Ok(file) => file,
    Err(why) => panic!("couldn't open {}", Error::description(&why)),
  };

  match f.read_to_end(&mut buffer) {
    Ok(_) => println!("success load"),
    Err(why) => panic!("couldn't open {}", Error::description(&why)),
  };

  let mut result: Vec<f32> = Vec::new();
  for chunk in buffer.chunks(4) {
    result.push(pack(chunk));
  }
  return result;
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
