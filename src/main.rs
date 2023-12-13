mod args;
use args::Args;
use std::io::{ErrorKind, Read, Result, Write};

const CHUNK_SIZE: usize = 16 * 1024;

fn main() -> Result<()> {
    let (mut reader, mut writer, silent) = Args::get();
    let mut total_bytes = 0;
    let mut buf = [0; CHUNK_SIZE];
    loop {
        let num_read = match reader.read(&mut buf) {
            Ok(0) => break,
            Ok(x) => Ok(x),
            Err(e) => Err(e),
        }?;
        total_bytes += num_read;
        if !silent {
            eprint!("\rbytes read: {}", total_bytes);
        }
        if let Err(e) = writer.write_all(&mut buf) {
            if e.kind() == ErrorKind::BrokenPipe {
                break;
            }
            return Err(e);
        }
    }
    Ok(())
}
