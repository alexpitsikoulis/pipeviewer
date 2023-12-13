use crate::CHUNK_SIZE;
use crossbeam::channel::Sender;
use std::{
    fs::File,
    io::{self, BufReader, Read, Result},
};

pub struct PipeReader(Box<dyn Read>);

impl PipeReader {
    pub fn new(infile: Option<String>) -> Result<Self> {
        let inner: Box<dyn Read> = match infile {
            Some(infile) => Box::new(BufReader::new(File::open(infile)?)),
            None => Box::new(BufReader::new(io::stdin())),
        };
        Ok(PipeReader(inner))
    }

    pub fn read(&mut self, stats_tx: Sender<usize>, write_tx: Sender<Vec<u8>>) -> Result<()> {
        let mut buf = [0; CHUNK_SIZE];
        loop {
            let num_read = match self.0.read(&mut buf) {
                Ok(0) => break,
                Ok(n) => n,
                Err(_) => break,
            };
            let _ = stats_tx.send(num_read);
            if write_tx.send(Vec::from(&buf[..num_read])).is_err() {
                break;
            }
        }
        let _ = stats_tx.send(0);
        let _ = write_tx.send(Vec::new());
        Ok(())
    }
}
