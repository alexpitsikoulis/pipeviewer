use crate::CHUNK_SIZE;
use std::fs::File;
use std::io::{self, BufReader, Read, Result};

pub struct PipeReader(Box<dyn Read>);

impl PipeReader {
    pub fn new(infile: Option<String>) -> Result<Self> {
        let inner: Box<dyn Read> = match infile {
            Some(infile) => Box::new(BufReader::new(File::open(infile)?)),
            None => Box::new(BufReader::new(io::stdin())),
        };
        Ok(PipeReader(inner))
    }

    pub fn read(&mut self) -> Result<Vec<u8>> {
        let mut buf = [0; CHUNK_SIZE];
        let num_read = self.0.read(&mut buf)?;
        Ok(Vec::from(&buf[..num_read]))
    }
}
