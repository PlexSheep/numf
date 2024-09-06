//! This module implements the actual formatting in [numf](crate).
//!
//! You can use it in your own program to convert numbers to formats.
//!
//! # Example
//!
//! The following example shows how to use numf to format your integers.
//!
//! ```
//! use numf::format::{Format, FormatOptions};
//!
//! let mut options = FormatOptions::default();
//! options.set_prefix(true);
//! options.set_padding(true);
//!
//! assert_eq!(Format::Hex.format_str(0x1337, &options), "0x1337");
//! assert_eq!(Format::Base32.format_str(0x41414242, &options), "032sIFAUEQQ=");
//! assert_eq!(Format::Base64.format_str(0x41414242, &options), "0sQUFCQg==");
//! // sometimes you might need the raw bytes instead of a String
//! assert_eq!(Format::Raw.format(0x1337, &options), vec![0x13, 0x37]);
//! assert_eq!(Format::Hex.format(0x1337, &options), vec![48, 120, 49, 51, 51, 55]);
//! ```

#![allow(dead_code)]
use std::default;
use std::fmt::Display;

// this is exported to lib.rs
use anyhow::anyhow;
use clap::{ArgGroup, Parser};
use libpt::bintols::{join, split};
use libpt::cli::args::VerbosityLevel;
use libpt::log::{debug, trace};

/// The number type [numf](crate) uses
pub type NumberType = u128;

/// formats supported by numf
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, Default)]
pub enum Format {
    Dec,
    #[default]
    Hex,
    Bin,
    Octal,
    Base64,
    Base32,
    /// Write raw data to stdout, not text
    Raw,
}

impl Display for Format {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

/// Describes what the formatter should do
///
/// Use [Self::default] to get a basic variant or create a object yourself.
///
/// This struct can be parsed with [clap] derive.
#[derive(Parser, Debug, Clone, PartialEq, Eq, Hash)]
#[clap(author, version, about, long_about = None)]
#[command(
    author,
    version,
    about,
    long_about,
    help_template = r#"{about-section}
{usage-heading} {usage}
{all-args}{tab}

{name}: {version}
Author: {author-with-newline}
"#
)]
#[clap(group(
            ArgGroup::new("format")
                .args(&["hex", "bin", "oct", "dec", "base64", "base32", "raw"]),
        ))]
pub struct FormatOptions {
    #[arg(short, long)]
    /// add a prefix (like "0x" for hex)
    prefix: bool,
    #[arg(short = 'P', long)]
    /// add a padding to make the number at least one byte long
    ///
    /// For example, `0b1100` will be `0b00001100` with this.
    /// This does not apply to all formats, only hexadecimal and binary.
    padding: bool,
    #[arg(short = 'x', long)]
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
    #[arg(short = 's', long)]
    /// format to base64
    base64: bool,
    #[arg(short = 'a', long)]
    /// format raw, no text
    raw: bool,
    #[arg(short = 'r', long, default_value_t = 0, value_parser=numf_parser::<NumberType>)]
    /// output random numbers
    ///
    /// Add a user defined amount of cryptographically pseudorandom numbers to the number list.
    rand: NumberType,
    #[arg(long, default_value_t = NumberType::MAX, value_parser=numf_parser::<NumberType>)]
    /// max for the random numbers
    ///
    /// Generated numbers will not be lower than this. Only has an effect with --rand set.
    rand_max: NumberType,
    #[arg(short = 'z', long)]
    /// format to base32
    base32: bool,
    #[clap(value_parser=numf_parser::<NumberType>, required=false)]
    /// numbers that should be formatted
    ///
    /// Any of the [Formats](Format::format) are supported, but the prefixes are needed for formats
    /// other than decimal.
    ///
    /// Formats:
    ///
    /// * '0x' - Hexadecimal
    ///
    /// * '0b' - Binary
    ///
    /// * '0o' - Octal
    ///
    /// * '0s' - Base64
    ///
    /// * '032s' - Base32
    ///
    /// The numbers may be left empty at first, if numbers are provided from the stdin.
    numbers: Vec<NumberType>,

    #[command(flatten)]
    pub(crate) verbosity: VerbosityLevel,
}

impl FormatOptions {
    /// get the format that the user has configured
    pub fn format(&self) -> Format {
        trace!("self.hex: {}", self.hex);
        if self.oct {
            Format::Octal
        } else if self.bin {
            Format::Bin
        } else if self.dec {
            Format::Dec
        } else if self.base64 {
            Format::Base64
        } else if self.base32 {
            Format::Base32
        } else if self.hex {
            Format::Hex
        } else if self.raw {
            Format::Raw
        } else {
            // none was explicitly selected
            debug!("no mode was explicitly selected, going with the default");
            Format::default()
        }
    }

    /// set the format manually
    pub fn set_format(&mut self, format: Format) {
        self.bin = false;
        self.oct = false;
        self.dec = false;
        self.hex = false;
        self.base64 = false;
        self.raw = false;
        self.base32 = false;
        match format {
            Format::Bin => self.bin = true,
            Format::Raw => self.raw = true,
            Format::Hex => self.hex = true,
            Format::Octal => self.oct = true,
            Format::Base64 => self.base64 = true,
            Format::Base32 => self.base32 = true,
            Format::Dec => self.dec = true,
        }
    }

    /// get numbers
    pub fn numbers(&self) -> &[u128] {
        self.numbers.as_ref()
    }

    /// set numbers manually
    pub fn set_numbers(&mut self, numbers: Vec<NumberType>) {
        self.numbers = numbers;
    }

    /// set padding manually
    pub fn set_padding(&mut self, value: bool) {
        self.padding = value
    }

    /// get padding
    pub fn padding(&self) -> bool {
        self.padding
    }

    /// get prefix
    pub fn prefix(&self) -> bool {
        self.prefix
    }

    /// set prefix manually
    pub fn set_prefix(&mut self, value: bool) {
        self.prefix = value;
    }

    /// manually add a number
    pub fn push_number(&mut self, value: NumberType) {
        self.numbers.push(value)
    }

    /// get rand
    pub fn rand(&self) -> NumberType {
        self.rand
    }

    /// set amount of extra random numbers manually
    pub fn set_rand(&mut self, rand: NumberType) {
        self.rand = rand;
    }

    /// get highes allowed random value
    pub fn rand_max(&self) -> NumberType {
        self.rand_max
    }

    /// set highes allowed random value
    pub fn set_rand_max(&mut self, rand_max: NumberType) {
        self.rand_max = rand_max;
    }
}

impl Default for FormatOptions {
    fn default() -> Self {
        Self {
            padding: false,
            prefix: false,
            oct: false,
            hex: false,
            bin: false,
            raw: false,
            base32: false,
            base64: false,
            dec: false,
            numbers: vec![],
            rand: 0,
            rand_max: NumberType::MAX,
            verbosity: VerbosityLevel::default(),
        }
    }
}

impl Format {
    pub fn prefix_str(&self) -> String {
        String::from_utf8_lossy(&self.prefix()).to_string()
    }

    /// Get the perfix for that [Format]
    pub fn prefix(&self) -> Vec<u8> {
        match self {
            // apperently used nowhere, sometimes 0 is used as a prefix but I
            // think this makes it more clear that this is decimal
            Format::Dec => b"0d".to_vec(),
            Format::Raw => [].to_vec(), // TODO: find a better way to deal with this
            // very common
            Format::Hex => b"0x".to_vec(),
            // very common
            Format::Bin => b"0b".to_vec(),
            // somewhat common
            Format::Octal => b"0o".to_vec(),
            // perl and a few other programs seem to use this too
            Format::Base64 => b"0s".to_vec(),
            // no idea, I made this up
            Format::Base32 => b"032s".to_vec(),
        }
    }
    /// format a number with a [Format] and [FormatOptions] to [String]
    pub fn format_str(&self, num: NumberType, options: &FormatOptions) -> String {
        String::from_utf8_lossy(&self.format(num, options)).to_string()
    }

    /// format a number with a [Format] and [FormatOptions]
    pub fn format(&self, num: NumberType, options: &FormatOptions) -> Vec<u8> {
        debug!("formatting mode: {self}");
        let mut buf: Vec<u8> = Vec::new();
        if options.prefix() {
            buf.append(&mut self.prefix());
        }
        match self {
            Format::Hex => {
                if options.padding() {
                    let tmp = &format!("{num:X}");
                    let tmp1 = &("0".repeat((2 - tmp.len() % 2) % 2) + tmp);
                    buf.append(&mut tmp1.as_bytes().to_owned());
                } else {
                    buf.append(&mut format!("{num:X}").as_bytes().to_owned());
                }
            }
            Format::Bin => {
                if options.padding() {
                    let tmp = &format!("{num:b}");
                    let tmp1 = &("0".repeat((8 - tmp.len() % 8) % 8) + tmp);
                    buf.append(&mut tmp1.as_bytes().to_owned());
                } else {
                    buf.append(&mut format!("{num:b}").as_bytes().to_owned());
                }
            }
            Format::Octal => buf.append(&mut format!("{num:o}").as_bytes().to_owned()),
            Format::Dec => buf.append(&mut format!("{num}").as_bytes().to_owned()),
            Format::Base64 => buf.append(
                &mut fast32::base64::RFC4648
                    .encode(&split::unsigned_to_vec(num))
                    .as_bytes()
                    .to_owned(),
            ),
            Format::Base32 => buf.append(
                &mut fast32::base32::RFC4648
                    .encode(&split::unsigned_to_vec(num))
                    .as_bytes()
                    .to_owned(),
            ),
            // Format::Raw => buf.append(&mut split::unsigned_to_vec(num)),
            Format::Raw => {
                debug!("do the raw thing");
                buf.append(&mut split::unsigned_to_vec(num))
            }
        }
        buf
    }
}

/// Validates an unsigned integer value that can be one of [Format](format::Format).
///
/// The number is assumed to be base-10 by default, it is parsed as a different
/// [Format](format::Format) if the number is prefixed with the [prefix](format::FormatOptions::prefix),
/// case sensitive. So if the user inputs `0b1100` then this is parsed as
/// [Binary](format::Format::Bin) and so on.
///
/// # Example
///
/// This allows base-10 addresses to be passed normally, or values formatted with any of the
/// [Formats](format::Format) defined by this crate to be passed when prefixed with the respective
/// prefix.
///
/// ```
/// use clap::Parser;
/// use numf::format::numf_parser;
///
/// #[derive(Parser)]
/// struct Args {
///     #[clap(short, long, value_parser=numf_parser::<u128>)]
///     address: u128,
/// }
/// let args = Args::parse_from(&["", "-a", "0x10"]);
/// assert_eq!(args.address, 16);
/// ```
pub fn numf_parser<T>(s: &str) -> anyhow::Result<T>
where
    T: std::str::FromStr + std::convert::TryFrom<u128>,
    <T as std::str::FromStr>::Err: std::fmt::Display,
    T: num::Num,
    <T as num::Num>::FromStrRadixErr: std::fmt::Display,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
    u128: std::convert::From<T>,
    <T as std::str::FromStr>::Err: std::error::Error,
    <T as std::convert::TryFrom<u128>>::Error: std::error::Error,
    <T as std::convert::TryFrom<u128>>::Error: std::marker::Send,
    <T as std::convert::TryFrom<u128>>::Error: std::marker::Sync,
    <T as std::convert::TryFrom<u128>>::Error: 'static,
{
    if s.starts_with(&Format::Dec.prefix_str()) || s.parse::<T>().is_ok() {
        let s = match s.strip_prefix(&Format::Dec.prefix_str()) {
            Some(sr) => sr,
            None => s,
        };
        match s.parse() {
            Ok(r) => Ok(r),
            Err(e) => {
                let e = format!("{e}");
                Err(anyhow!(e))
            }
        }
    } else if s.starts_with(&Format::Hex.prefix_str()) {
        let s = match s.strip_prefix(&Format::Hex.prefix_str()) {
            Some(sr) => sr,
            None => s,
        };
        match T::from_str_radix(s, 16) {
            Ok(r) => Ok(r),
            Err(e) => {
                let e = format!("{e}");
                Err(anyhow!(e))
            }
        }
    } else if s.starts_with(&Format::Octal.prefix_str()) {
        let s = match s.strip_prefix(&Format::Octal.prefix_str()) {
            Some(sr) => sr,
            None => s,
        };
        match T::from_str_radix(s, 8) {
            Ok(r) => Ok(r),
            Err(e) => {
                let e = format!("{e}");
                Err(anyhow!(e))
            }
        }
    } else if s.starts_with(&Format::Bin.prefix_str()) {
        let s = match s.strip_prefix(&Format::Bin.prefix_str()) {
            Some(sr) => sr,
            None => s,
        };
        match T::from_str_radix(s, 2) {
            Ok(r) => Ok(r),
            Err(e) => {
                let e = format!("{e}");
                Err(anyhow!(e))
            }
        }
    } else if s.starts_with(&Format::Base64.prefix_str()) {
        let s = match s.strip_prefix(&Format::Base64.prefix_str()) {
            Some(sr) => sr,
            None => s,
        };
        match fast32::base64::RFC4648.decode_str(s) {
            Ok(r) => Ok(join::array_to_unsigned::<T>(&r)?),
            Err(e) => {
                let e = format!("{e}");
                Err(anyhow!(e))
            }
        }
    } else if s.starts_with(&Format::Base32.prefix_str()) {
        let s = match s.strip_prefix(&Format::Base32.prefix_str()) {
            Some(sr) => sr,
            None => s,
        };
        match fast32::base32::RFC4648.decode_str(s) {
            Ok(r) => Ok(join::array_to_unsigned::<T>(&r)?),
            Err(e) => {
                let e = format!("{e}");
                Err(anyhow!(e))
            }
        }
    } else if s.starts_with(&Format::Raw.prefix_str()) {
        let s = match s.strip_prefix(&Format::Raw.prefix_str()) {
            Some(sr) => sr,
            None => s,
        };
        todo!("reading raw not implemented")
    } else {
        let e = "could not determine the format of the value".to_string();
        Err(anyhow!(e))
    }
}
