use std::f32;
use std::io::prelude::*;
use std::fs::File;
use std::error::Error;
use std::mem;

extern crate binary_nn;
use binary_nn::backend::bitmatrix::BitMatrix2;
use binary_nn::backend::bitmatrix_trait::*;

pub fn load_f32_as_bitmatrix(path: String, nrow: u32, ncol: u32) -> BitMatrix2 {
  let f32_vec = load_f32(path);
  let mut bit_mat = BitMatrix2::falses((nrow, ncol));
  let mut idx = 0;
  for irow in 0..nrow {
    for icol in 0..ncol {
      if f32_vec[idx] > 0.0 {
        bit_mat.set_true((irow, icol));
      } else {
        bit_mat.set_false((irow, icol));
      }
      idx += 1
    }
  }
  return bit_mat;
}

pub fn load_f32(path: String) -> Vec<f32> {
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

pub fn pack(buffer: &[u8]) -> f32 {
  let mut buf: [u8; 4] = [0; 4];
  buf[0] = buffer[0];
  buf[1] = buffer[1];
  buf[2] = buffer[2];
  buf[3] = buffer[3];
  let x: f32 = unsafe { mem::transmute(buf) };
  //  println!("x is {:#?}", x);
  return x;
}
