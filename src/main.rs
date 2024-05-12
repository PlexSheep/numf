//! # numf
//!
//! This binary should just take any amount of numbers and print them out formatted to some other
//! system.

use clap::Parser;

mod format;
use format::*;

fn main() {
    let options = FormatOptions::parse();

    let mut out: Vec<String> = Vec::new();

    for num in options.numbers() {
        out.push(options.format().format(*num, &options));
    }
    for o in out {
        println!("{o}")
    }
}
