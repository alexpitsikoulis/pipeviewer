use std::{
    io::Result,
    sync::{Arc, Mutex, mpsc::{Receiver, Sender}},
};

pub fn stats(silent: bool, stats_rx: Receiver<Vec<u8>>, write_tx: Sender<Vec<u8>>) -> Result<()> {
    let mut total_bytes = 0;
    loop {
        let buf = stats_rx.recv().unwrap();
        let num_bytes = buf.len();
        total_bytes += num_bytes;
        if !silent {
            eprint!("\rbytes read: {}", total_bytes);
        }
        if write_tx.send(buf).is_err() {
            break;
        }
        if num_bytes == 0 {
            break;
        }
    }
    if !silent {
        eprintln!();
    }
    Ok(())
}
