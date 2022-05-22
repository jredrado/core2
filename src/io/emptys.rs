use crate::io::Read;
use crate::io::Result;

pub struct Empty {}

impl Read for Empty {
    fn read(&mut self, _: &mut [u8]) -> Result<usize> {
        Ok(0)
    }
}

pub fn empty() -> Empty {
    Empty {}
}