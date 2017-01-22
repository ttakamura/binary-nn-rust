use std::f32;
use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::error::Error;
use std::mem;

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
  println!("x is {:#?}", x);
  return x;
}
