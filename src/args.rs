use std::io::{Read, Write, BufReader, BufWriter, self};

use clap::Parser;

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

impl Args {
    pub fn get() -> (Box<dyn Read>, Box<dyn Write>, bool) {
        match Args::parse() {
            Args {
                infile,
                outfile,
                silent,
            } => (
                match infile {
                    Some(infile) => Box::new(BufReader::new(std::fs::File::open(infile).unwrap())),
                    None => Box::new(io::stdin()),
                },
                match outfile {
                    Some(outfile) => Box::new(BufWriter::new(match std::fs::File::open(outfile.clone()) {
                        Ok(file) => file,
                        Err(_) => std::fs::File::create(outfile).unwrap(),
                    })),
                    None => Box::new(io::stdout()),
                },
                silent,
            ),
        }
    }
}