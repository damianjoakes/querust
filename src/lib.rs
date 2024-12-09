#![feature(maybe_uninit_slice)]
#![feature(read_buf)]
#![feature(string_from_utf8_lossy_owned)]

#[cfg(test)]
mod tests;
mod internal;
mod build;