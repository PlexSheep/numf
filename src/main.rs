//! # numf
//!
//! This binary should just take any amount of numbers and print them out formatted to some other
//! system.

use clap::{ArgGroup, Parser};
use clap_num::maybe_hex;

mod format;
use format::*;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
#[clap(group(
            ArgGroup::new("format")
                .args(&["hex", "bin", "oct", "dec"]),
        ))]
struct Cli {
    #[arg(short, long)]
    /// add a prefix (like "0x" for hex)
    prefix: bool,
    #[arg(short = 'x', long, default_value_t = true)]
    /// format to hexadecimal
    hex: bool,
    #[arg(short, long)]
    /// format to binary
    bin: bool,
    #[arg(short, long)]
    /// format to decimal
    dec: bool,
    #[arg(short, long)]
    /// format to octal
    oct: bool,
    #[clap(value_parser=maybe_hex::<Num>)]
    /// at least one number that should be formatted
    ///
    /// supports either base 10 or base 16 inputs (with 0xaaaa)
    numbers: Vec<Num>,
}

impl Cli {
    fn format(&self) -> Format {
        if self.oct {
            Format::Octal
        } else if self.bin {
            Format::Bin
        } else if self.dec {
            Format::Dec
        } else if self.hex {
            Format::Hex
        } else {
            unreachable!()
        }
    }
}

fn main() {
    let cli = Cli::parse();

    let mut out: Vec<String> = Vec::new();

    for num in &cli.numbers {
        out.push(cli.format().format(*num, cli.prefix));
    }
    for o in out {
        println!("{o}")
    }
}

