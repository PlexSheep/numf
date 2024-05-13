#![allow(dead_code)] // this is exported to lib.rs
use anyhow::anyhow;
use clap::{ArgGroup, Parser};
use libpt::bintols::{join, split};

pub type NumberType = u128;

/// formats supported by numf
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub enum Format {
    Dec,
    Hex,
    Bin,
    Octal,
    Base64,
    Base32,
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
                .args(&["hex", "bin", "oct", "dec", "base64", "base32"]),
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
    #[arg(short = 's', long)]
    /// format to base64
    base64: bool,
    #[arg(short = 'z', long)]
    /// format to base32
    base32: bool,
    #[clap(value_parser=numf_parser::<NumberType>, required=false)]
    /// at least one number that should be formatted
    ///
    /// Any of the [Formats](Format::format) are supported, but the prefixes are needed for formats
    /// other than decimal.
    ///
    /// Formats:
    ///
    /// - '0x' - Hexadecimal
    /// - '0b' - Binary
    /// - '0o' - Octal
    /// - '0s' - Base64
    /// - '032s' - Base32
    ///
    /// The numbers may be left empty at first, if numbers are provided with the stdin.
    numbers: Vec<NumberType>,
}

impl FormatOptions {
    /// get the format that the user has configured
    pub fn format(&self) -> Format {
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
        } else {
            unreachable!()
        }
    }

    /// set the format manually
    pub fn set_format(&mut self, format: Format) {
        self.bin = false;
        self.oct = false;
        self.dec = false;
        self.hex = false;
        self.base64 = false;
        self.base32 = false;
        match format {
            Format::Bin => self.bin = true,
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
}

impl Default for FormatOptions {
    fn default() -> Self {
        Self {
            padding: false,
            prefix: false,
            oct: false,
            hex: true,
            bin: false,
            base32: false,
            base64: false,
            dec: false,
            numbers: vec![],
        }
    }
}

impl Format {
    /// Get the perfix for that [Format]
    pub fn prefix(&self) -> String {
        match self {
            // apperently used nowhere, sometimes 0 is used as a prefix but I
            // think this makes it more clear that this is decimal
            Format::Dec => "0d",
            // very common
            Format::Hex => "0x",
            // very common
            Format::Bin => "0b",
            // somewhat common
            Format::Octal => "0o",
            // perl and a few other programs seem to use this too
            Format::Base64 => "0s",
            // no idea, I made this up
            Format::Base32 => "032s",
        }
        .to_string()
    }
    /// format a number with a [Format] and [FormatOptions]
    pub fn format(&self, num: NumberType, options: &FormatOptions) -> String {
        let mut buf = String::new();
        if options.prefix() {
            buf += &self.prefix();
        }
        match self {
            Format::Hex => {
                if options.padding() {
                    let tmp = &format!("{num:X}");
                    buf += &("0".repeat((2 - tmp.len() % 2) % 2) + tmp);
                } else {
                    buf += &format!("{num:X}");
                }
            }
            Format::Bin => {
                if options.padding() {
                    let tmp = &format!("{num:b}");
                    buf += &("0".repeat((8 - tmp.len() % 8) % 8) + tmp);
                } else {
                    buf += &format!("{num:b}");
                }
            }
            Format::Octal => buf += &format!("{num:o}"),
            Format::Dec => buf += &format!("{num}"),
            Format::Base64 => buf += &fast32::base64::RFC4648.encode(&split::unsigned_to_vec(num)),
            Format::Base32 => buf += &fast32::base32::RFC4648.encode(&split::unsigned_to_vec(num)),
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
    if s.starts_with(&Format::Dec.prefix()) || s.parse::<T>().is_ok() {
        let s = match s.strip_prefix(&Format::Dec.prefix()) {
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
    } else if s.starts_with(&Format::Hex.prefix()) {
        let s = match s.strip_prefix(&Format::Hex.prefix()) {
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
    } else if s.starts_with(&Format::Octal.prefix()) {
        let s = match s.strip_prefix(&Format::Octal.prefix()) {
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
    } else if s.starts_with(&Format::Bin.prefix()) {
        let s = match s.strip_prefix(&Format::Bin.prefix()) {
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
    } else if s.starts_with(&Format::Base64.prefix()) {
        let s = match s.strip_prefix(&Format::Base64.prefix()) {
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
    } else if s.starts_with(&Format::Base32.prefix()) {
        let s = match s.strip_prefix(&Format::Base32.prefix()) {
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
    } else {
        let e = "could not determine the format of the value".to_string();
        Err(anyhow!(e))
    }
}
