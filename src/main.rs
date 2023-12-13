use pipeviewer::{stats, Args, PipeReader, PipeWriter};
use std::io::Result;

fn main() -> Result<()> {
    let (infile, outfile, silent) = Args::get()?;
    let mut reader = PipeReader::new(infile)?;
    let mut writer = PipeWriter::new(outfile)?;
    let mut total_bytes = 0;
    loop {
        let buf = match reader.read() {
            Ok(x) if x.is_empty() => break,
            Ok(x) => x,
            Err(_) => break,
        };
        stats(silent, buf.len(), &mut total_bytes, false);
        if !writer.write(buf)? {
            break;
        }
    }
    Ok(())
}
