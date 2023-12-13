use clap::Parser;
use std::io::Result;

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
    pub fn get() -> Result<(Option<String>, Option<String>, bool)> {
        let (reader, writer, silent) = match Args::parse() {
            Args {
                infile,
                outfile,
                silent,
            } => (infile, outfile, silent),
        };
        Ok((reader, writer, silent))
    }
}
