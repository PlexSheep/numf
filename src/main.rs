//! # numf
//!
//! This binary should just take any amount of numbers and print them out formatted to some other
//! system.

use std::io::Read;
use std::process::exit;

use clap::Parser;

mod format;
use format::*;

fn main() {
    // try to read from stdin first, appending the numbers we read to the FormatOptions
    let mut args: Vec<String> = std::env::args_os()
        .map(|x| x.into_string().unwrap())
        .collect();
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
            let whole = whole.replace('\n', "");
            for s in whole.split(' ') {
                args.push(s.to_string());
            }
        }
        Err(e) => {
            eprintln!("could not read from stdin: {e:#?}");
            exit(2);
        }
    };

    let options = FormatOptions::parse_from(args);

    let mut out: Vec<String> = Vec::new();

    for num in options.numbers() {
        out.push(options.format().format(*num, &options));
    }
    for o in out {
        println!("{o}")
    }
}
