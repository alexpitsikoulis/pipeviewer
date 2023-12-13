use std::{
    fs::File,
    io::{self, BufWriter, ErrorKind, Result, Write},
};

pub struct PipeWriter(Box<dyn Write>);

impl PipeWriter {
    pub fn new(outfile: Option<String>) -> Result<Self> {
        let inner: Box<dyn Write> = match outfile {
            Some(outfile) => match File::open(&outfile) {
                Ok(file) => Box::new(BufWriter::new(file)),
                Err(_) => Box::new(BufWriter::new(File::create(&outfile)?)),
            },
            None => Box::new(BufWriter::new(io::stdout())),
        };

        Ok(PipeWriter(inner))
    }

    pub fn write(&mut self, buf: Vec<u8>) -> Result<bool> {
        if let Err(e) = self.0.write_all(&buf) {
            if e.kind() == ErrorKind::BrokenPipe {
                return Ok(false);
            };
            return Err(e);
        }
        Ok(true)
    }
}
