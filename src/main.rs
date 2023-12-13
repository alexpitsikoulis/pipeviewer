use pipeviewer::{stats, Args, PipeReader, PipeWriter};
use std::{
    io::Result,
    sync::mpsc,
    thread,
};

fn main() -> Result<()> {
    let (infile, outfile, silent) = Args::get()?;

    let (stats_tx, stats_rx) = mpsc::channel();
    let (write_tx, write_rx) = mpsc::channel();
    
    let read_handle = thread::spawn(move || {
        let mut reader = PipeReader::new(infile)?;
        reader.read(stats_tx)
    });
    let stats_handle = thread::spawn(move || stats(silent, stats_rx, write_tx));
    let write_handle = thread::spawn(move || {
        let mut writer = PipeWriter::new(outfile)?;
        writer.write(write_rx)
    });

    let read_io_result = read_handle.join().unwrap();
    let stats_io_result = stats_handle.join().unwrap();
    let write_io_result = write_handle.join().unwrap();

    read_io_result?;
    stats_io_result?;
    write_io_result?;

    Ok(())
}
