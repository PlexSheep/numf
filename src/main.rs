use std::io::{IsTerminal, Read, Write};
use std::process::exit;

use clap::{CommandFactory, Parser};
use numf::format::numf_parser_str;

mod format;
use crate::format::{numf_parser, Format};
use format::*;
use libpt::log::{debug, error};

fn main() -> anyhow::Result<()> {
    // try to read from stdin first, appending the numbers we read to the FormatOptions
    let mut options = FormatOptions::parse();
    let _logger = libpt::log::Logger::builder()
        .set_level(options.verbosity.level())
        .display_time(false)
        .build()
        .map_err(|e| {
            error!("could not initialize logger: {e}");
        });
    debug!("logger active");

    let mut stdin_nums = Vec::new();
    let stdin = std::io::stdin();
    // only accept numbers from stdin if the stdin is not an interactive terminal
    if !stdin.is_terminal() {
        match stdin.lock().read_to_end(&mut stdin_nums) {
            Ok(_) => {
                let whole: String = match String::from_utf8(stdin_nums.clone()) {
                    Ok(r) => r,
                    Err(_) => {
                        let number = match numf_parser(&stdin_nums) {
                            Ok(n) => n,
                            Err(e) => {
                                eprintln!("{}", FormatOptions::command().render_usage());
                                error!("could raw inputs from stdin as numbers: {e:#?}");
                                exit(2);
                            }
                        };
                        options.push_number(number);
                        String::new()
                    }
                };
                let split = whole.split_whitespace();
                for s in split {
                    let number = match numf_parser_str(s) {
                        Ok(n) => n,
                        Err(e) => {
                            eprintln!("{}", FormatOptions::command().render_usage());
                            error!("could not parse number from stdin: {e:#?}");
                            exit(2);
                        }
                    };
                    options.push_number(number)
                }
            }
            Err(e) => {
                eprintln!("{}", FormatOptions::command().render_usage());
                error!("could not read from stdin: {e:#?}");
                exit(2);
            }
        };
    }

    // add random numbers to the number list, according to how many are requested
    if options.rand() > 0 {
        use rand::prelude::*;
        let mut rand = rand::rngs::OsRng;
        for _i in 0..options.rand() {
            options.push_number(rand.gen_range(0..options.rand_max()));
        }
    }

    // exit with error if no numbers are to be formatted
    if options.numbers().is_empty() {
        eprintln!("{}", FormatOptions::command().render_usage());
        error!("no numbers have been provided");
        exit(1);
    }

    let mut out: Vec<Vec<u8>> = Vec::new();

    for num in options.numbers() {
        out.push(options.format().format(*num, &options));
    }
    for o in out {
        let mut stdout = std::io::stdout();
        stdout.write_all(&o)?;
        if options.format() != Format::Raw {
            stdout.write_all(b"\n")?;
        }
        stdout.flush()?;
    }
    Ok(())
}
