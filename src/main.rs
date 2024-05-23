use std::io::{IsTerminal, Read};
use std::process::exit;

use clap::{CommandFactory, Parser};

mod format;
use format::*;
use numf::format::numf_parser;

fn main() -> anyhow::Result<()> {
    // try to read from stdin first, appending the numbers we read to the FormatOptions
    let mut options = FormatOptions::parse();
    let mut stdin_nums = Vec::new();
    let stdin = std::io::stdin();
    if !stdin.is_terminal() {
        match stdin.lock().read_to_end(&mut stdin_nums) {
            Ok(_) => {
                let whole: String = match String::from_utf8(stdin_nums) {
                    Ok(r) => r,
                    Err(e) => {
                        eprintln!("{}", FormatOptions::command().render_usage());
                        eprintln!("stdin for this program only accepts text: {e:#?}");
                        exit(1);
                    }
                };
                let split = whole.split_whitespace();
                for s in split {
                    let number = match numf_parser(s) {
                        Ok(n) => n,
                        Err(e) => {
                            eprintln!("{}", FormatOptions::command().render_usage());
                            eprintln!("could not parse number from stdin: {e:#?}");
                            exit(2);
                        }
                    };
                    options.push_number(number)
                }
            }
            Err(e) => {
                eprintln!("{}", FormatOptions::command().render_usage());
                eprintln!("could not read from stdin: {e:#?}");
                exit(2);
            }
        };
    }

    if options.rand() > 0 {
        use rand::prelude::*;
        let mut rand = rand::rngs::OsRng;
        for _i in 0..options.rand() {
            options.push_number(rand.gen_range(0..options.rand_max()));
        }
    }

    if options.numbers().is_empty() {
        eprintln!("{}", FormatOptions::command().render_usage());
        eprintln!("no numbers have been provided");
        exit(1);
    }

    let mut out: Vec<String> = Vec::new();

    for num in options.numbers() {
        out.push(options.format().format(*num, &options));
    }
    for o in out {
        println!("{o}")
    }
    Ok(())
}
