use crossbeam::channel::Receiver;
use std::{
    fs::File,
    io::{self, BufWriter, ErrorKind, Result, Write},
};

pub struct PipeWriter(Box<dyn Write>);

impl PipeWriter {
    pub fn new(outfile: Option<String>) -> Result<Self> {
        let inner: Box<dyn Write> = match outfile {
            Some(outfile) => Box::new(File::create(outfile)?),
            None => Box::new(BufWriter::new(io::stdout())),
        };

        Ok(PipeWriter(inner))
    }

    pub fn write(&mut self, write_rx: Receiver<Vec<u8>>) -> Result<()> {
        loop {
            let buf = write_rx.recv().unwrap();
            if buf.is_empty() {
                break;
            }
            if let Err(e) = self.0.write_all(&buf) {
                if e.kind() == ErrorKind::BrokenPipe {
                    return Ok(());
                };
                return Err(e);
            }
        }
        Ok(())
    }
}
