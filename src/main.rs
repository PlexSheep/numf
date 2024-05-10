//! # numf
//!
//! This binary should just take any amount of numbers and print them out formatted to some other
//! system.

use clap::{ArgGroup, Parser};
use clap_num::maybe_hex;

pub type Num = u128;

#[derive(Copy, Clone, Debug)]
enum Format {
    Dec,
    Hex,
    Bin,
    Octal,
}

impl Format {
    fn prefix(&self) -> String {
        match self {
            Format::Dec => "0d",
            Format::Hex => "0x",
            Format::Bin => "0b",
            Format::Octal => "0o",
        }
        .to_string()
    }
    fn format(&self, num: Num, prefix: bool) -> String {
        let mut buf = String::new();
        if prefix {
            buf += &self.prefix();
        }
        match self {
            Format::Hex => {
                buf += &format!("{num:X}");
            }
            Format::Bin => {
                buf += &format!("{num:b}");
            }
            Format::Octal => {
                buf += &format!("{num:o}");
            }
            Format::Dec => {
                buf += &format!("{num}");
            }
        }
        buf
    }
}

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
    let _ = formatter();
}

fn formatter() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let mut out: Vec<String> = Vec::new();

    for num in &cli.numbers {
        out.push(cli.format().format(*num, cli.prefix));
    }
    for o in out {
        println!("{o}")
    }

    Ok(())
}
