//! # numf
//!
//! This binary should just take any amount of numbers and print them out formatted to some other
//! system.

use std::io::Read;
use std::process::exit;

use clap::{CommandFactory, Parser};

mod format;
use format::*;
use numf::format::numf_parser;

fn main() {
    // try to read from stdin first, appending the numbers we read to the FormatOptions
    let mut options = FormatOptions::parse();
    let mut stdin_nums = Vec::new();
    match std::io::stdin().lock().read_to_end(&mut stdin_nums) {
        Ok(_) => {
            let whole: String = match String::from_utf8(stdin_nums) {
                Ok(r) => r,
                Err(e) => {
                    eprintln!("stdin for this program only accepts text: {e:#?}");
                    exit(1);
                }
            };
            let split = whole.split_whitespace();
            for s in split {
                let number = match numf_parser(s) {
                    Ok(n) => n,
                    Err(e) => {
                        eprintln!("could not parse number from stdin: {e:#?}");
                        exit(2);
                    }
                };
                options.push_number(number)
            }
        }
        Err(e) => {
            eprintln!("could not read from stdin: {e:#?}");
            exit(2);
        }
    };

    if options.numbers().is_empty() {
        format!("{}", FormatOptions::command().render_usage());
        exit(1);
    }

    let mut out: Vec<String> = Vec::new();

    for num in options.numbers() {
        out.push(options.format().format(*num, &options));
    }
    for o in out {
        println!("{o}")
    }
}
