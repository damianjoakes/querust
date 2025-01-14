use std::error::Error;

pub trait Connector {
    fn push(&self, bytes: &[u8]) -> Result<usize, Box<dyn Error>>;
    fn pull(&self, buffer: &[u8]) -> Result<usize, Box<dyn Error>>;
}