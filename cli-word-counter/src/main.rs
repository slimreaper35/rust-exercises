use clap::Parser;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[derive(Parser)]
struct Args {
    #[arg(
        short = 'l',
        long,
        help = "Print the number of lines in the given input"
    )]
    lines: bool,

    #[arg(
        short = 'w',
        long,
        help = "Print the number of words in the given input"
    )]
    words: bool,

    #[arg(
        short = 'm',
        long,
        help = "Print the number of chars in the given input"
    )]
    chars: bool,

    #[arg(
        short = 'c',
        long,
        help = "Print the number of bytes in the given input"
    )]
    bytes: bool,

    #[arg(
        short = 'L',
        long,
        help = "Print the length of the longest line in the given input"
    )]
    max_line_length: bool,

    file: Option<String>,
}

impl Args {
    fn no_params(&self) -> bool {
        !self.lines && !self.words && !self.chars && !self.bytes && !self.max_line_length
    }
}

fn print_info(args: Args, reader: impl BufRead) {
    let mut lines = 0;
    let mut words = 0;
    let mut chars = 0;
    let mut bytes = 0;
    let mut max_line_length = 0;

    for line in reader.lines().map_while(|line| line.ok()) {
        lines += 1;
        words += line.split_whitespace().count();

        // + 1 to include newline byte (or CRLF) removed by `lines()`
        chars += line.chars().count() + 1;
        bytes += line.len() + 1;

        max_line_length = max_line_length.max(line.chars().count());
    }

    let print_defaults = args.no_params();

    if args.lines || print_defaults {
        print!("{} ", lines);
    }
    if args.words || print_defaults {
        print!("{} ", words);
    }
    if args.chars {
        print!("{} ", chars);
    }
    if args.bytes || print_defaults {
        print!("{} ", bytes);
    }
    if args.max_line_length {
        print!("{} ", max_line_length);
    }

    println!("{}", args.file.unwrap_or_default());
}

fn main() -> io::Result<()> {
    let args = Args::parse();
    match &args.file {
        Some(filename) => {
            let file = File::open(filename)?;
            let reader = BufReader::new(file);
            print_info(args, reader);
        }
        None => {
            let reader = BufReader::new(std::io::stdin());
            print_info(args, reader);
        }
    }
    Ok(())
}
