use clap::Parser;
use crossbeam::channel::{bounded, unbounded};
use pipeviewer::{stats, PipeReader, PipeWriter};
use std::{io::Result, thread};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(short, long)]
    infile: Option<String>,
    #[arg(short, long)]
    outfile: Option<String>,
    #[clap(short, long, action)]
    silent: bool,
}

fn main() -> Result<()> {
    let Args{infile, outfile, silent} = Args::parse();

    let (stats_tx, stats_rx) = unbounded();
    let (write_tx, write_rx) = bounded(1024);

    let read_handle = thread::spawn(move || {
        let mut reader = PipeReader::new(infile)?;
        reader.read(stats_tx, write_tx)
    });
    let stats_handle = thread::spawn(move || stats(silent, stats_rx));
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
