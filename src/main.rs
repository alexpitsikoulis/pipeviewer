use std::io::{self, Read, Write, Result, ErrorKind};

const CHUNK_SIZE: usize = 16 * 1024;

fn main() -> Result<()> {
    let silent = !std::env::var("PV_SILENT").unwrap_or_default().is_empty();
    let mut total_bytes = 0;
    let mut buf = [0; CHUNK_SIZE];
    loop {
        let num_read = match io::stdin().read(&mut buf) {
            Ok(0) => break,
            Ok(x) => Ok(x),
            Err(e) => Err(e),
        }?;
        total_bytes += num_read;
        if !silent {
            eprint!("\rbytes read: {}", total_bytes);
        }
        if let Err(e) = io::stdout().write_all(&buf[..num_read]) {
            if e.kind() == ErrorKind::BrokenPipe {
                break;
            }
            return Err(e);
        }
    }
    Ok(())
}
