use crossbeam::channel::Receiver;
use std::io::{self, Result, Stderr, Write};
use crossterm::{
    cursor, execute,
    style::{self, Color, PrintStyledContent, Stylize},
    terminal::{Clear, ClearType},
};
use crate::timer::{Timer, TimeOutput};

pub fn stats(silent: bool, stats_rx: Receiver<usize>) -> Result<()> {
    let mut total_bytes = 0;
    let mut timer = Timer::new();
    let mut stderr = io::stderr();
    loop {
        let num_bytes = stats_rx.recv().unwrap();
        timer.update();
        let bytes_per_second = num_bytes as f64 / timer.delta.as_secs_f64();
        total_bytes += num_bytes;
        if !silent && timer.ready {
            timer.ready = false;
            output_progress(
                &mut stderr,
                total_bytes,
                timer.start.elapsed().as_secs().as_time(),
                bytes_per_second
            );
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

fn output_progress(stderr: &mut Stderr, bytes: usize, elapsed: String, rate: f64) {
    let bytes = style::style(format!("bytes read: {}\t", bytes)).with(Color::Red);
    let elapsed = style::style(format!("time elapsed: {}\t", elapsed)).with(Color::Green);
    let rate = style::style(format!("[{:.02} bytes/second]", rate)).with(Color::Cyan);
    let _ = execute!(
        stderr,
        cursor::MoveToColumn(0),
        Clear(ClearType::CurrentLine),
        PrintStyledContent(bytes),
        PrintStyledContent(elapsed),
        PrintStyledContent(rate),
    );
    let _ = stderr.flush();
}