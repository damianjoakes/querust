#![feature(maybe_uninit_slice)]
#![feature(read_buf)]
#![feature(string_from_utf8_lossy_owned)]

#[cfg(test)]
mod tests;

pub(in crate) mod serialization;
mod internal;

pub use crate::serialization::encode as encode;
pub use crate::serialization::decode as decode;