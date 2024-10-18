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
//! assert_eq!(Format::Raw.format(0x1337, &options), vec![0x00, 0x13, 0x37]);
//! assert_eq!(Format::Hex.format(0x1337, &options), vec![48, 120, 49, 51, 51, 55]);
//!
//! options.set_prefix(false);
//! options.set_padding(false);
//!
//! assert_eq!(Format::Hex.format_str(0x1337, &options), "1337");
//! assert_eq!(Format::Base32.format_str(0x41414242, &options), "IFAUEQQ=");
//! assert_eq!(Format::Base64.format_str(0x41414242, &options), "QUFCQg==");
//!
//! assert_eq!(Format::Raw.format(0x1337, &options), vec![0x13, 0x37]);
//! assert_eq!(Format::Hex.format(0x1337, &options), vec![49, 51, 51, 55]);
//! ```

#![allow(dead_code)]
use std::fmt::Display;

// this is exported to lib.rs
use anyhow::anyhow;
use clap::{ArgGroup, Parser};
use libpt::bintols::{join, split};
use libpt::cli::args::VerbosityLevel;
use libpt::log::{debug, trace};

/// The number type [numf](crate) uses
pub type NumberType = u128;

/// Describes a format for numbers
///
/// [Format] can be used to convert unsigned integers into a textual or other representation. See
/// [Format::format_str] for more. It is also possible to parse the various represenations to
/// a rust integer, see [numf_parser_str] for that.
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, Default)]
pub enum Format {
    Dec,
    #[default]
    Hex,
    Bin,
    Octal,
    Base64,
    Base32,
    /// Write raw data, not text
    Raw,
}

impl Display for Format {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

/// Describes what the formatter should do exactly
///
/// Use [Self::default] to get a basic variant or create a object yourself.
///
/// This struct can be parsed with [clap] derive.
///
/// # Example
///
/// ```
/// use numf::format::{Format, FormatOptions};
/// let mut options = FormatOptions::default();
///
/// assert_eq!(Format::Bin.format_str(256, &options), "100000000");
/// assert_eq!(Format::Hex.format_str(256, &options), "100");
/// assert_eq!(Format::Base64.format_str(256, &options), "AQA=");
///
/// options.set_prefix(true);
/// options.set_padding(true);
///
/// assert_eq!(Format::Bin.format_str(256, &options), "0b0000000100000000");
/// assert_eq!(Format::Hex.format_str(256, &options), "0x0100");
/// assert_eq!(Format::Base64.format_str(256, &options), "0sAQA=");
///
/// ```
#[derive(Parser, Debug, Clone, PartialEq, Eq, Hash)]
#[command(
    author,
    version,
    about,
    long_about,
    help_template = libpt::cli::args::HELP_TEMPLATE)]
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
    #[arg(short = 'r', long, default_value_t = 0, value_parser=numf_parser_str::<NumberType>)]
    /// output random numbers
    ///
    /// Add a user defined amount of cryptographically pseudorandom numbers to the number list.
    rand: NumberType,
    #[arg(long, default_value_t = NumberType::MAX, value_parser=numf_parser_str::<NumberType>)]
    /// max for the random numbers
    ///
    /// Generated numbers will not be lower than this. Only has an effect with --rand set.
    rand_max: NumberType,
    #[arg(short = 'z', long)]
    /// format to base32
    base32: bool,
    #[clap(value_parser=numf_parser_str::<NumberType>, required=false)]
    /// numbers that should be formatted
    ///
    /// Any of the [Formats](Format::format) are supported, but the prefixes are needed for formats
    /// other than decimal.
    ///
    /// Formats: Decimal, Hexadecimal, Binary, Octal, Base64, Base32, Raw data
    ///
    /// Underscores will be completely ignored and are allowed for readability.
    ///
    /// Format Prefixes:
    ///
    /// * '0d' - Decimal, assumed for numeric values by default
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
    /// * If no format can be determined, the data will be assumed to be raw bytes.
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
    /// Get the perfix for that [Format] as [Vec<u8>].
    ///
    /// # Example
    ///
    /// ```
    /// # use numf::format::Format;
    /// assert_eq!(Format::Bin.prefix_str(), "0b");
    /// assert_eq!(Format::Dec.prefix_str(), "0d");
    /// assert_eq!(Format::Hex.prefix_str(), "0x");
    /// assert_eq!(Format::Octal.prefix_str(), "0o");
    /// assert_eq!(Format::Base64.prefix_str(), "0s");
    /// assert_eq!(Format::Base32.prefix_str(), "032s");
    /// assert_eq!(Format::Raw.prefix_str(), "\x00");
    /// ```
    pub fn prefix_str(&self) -> String {
        String::from_utf8_lossy(&self.prefix()).to_string()
    }

    /// Get the perfix for that [Format] as [Vec<u8>].
    ///
    /// # Example
    ///
    /// ```
    /// # use numf::format::Format;
    /// assert_eq!(Format::Bin.prefix(), b"0b");
    /// assert_eq!(Format::Dec.prefix(), b"0d");
    /// assert_eq!(Format::Hex.prefix(), b"0x");
    /// assert_eq!(Format::Octal.prefix(), b"0o");
    /// assert_eq!(Format::Base64.prefix(), b"0s");
    /// assert_eq!(Format::Base32.prefix(), b"032s");
    /// assert_eq!(Format::Raw.prefix(), vec![0x00]);
    /// ```
    pub fn prefix(&self) -> Vec<u8> {
        match self {
            // apperently used nowhere, sometimes 0 is used as a prefix but I
            // think this makes it more clear that this is decimal
            Format::Dec => b"0d".to_vec(),
            Format::Raw => [0x00].to_vec(),
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
    /// format a number with a [Format] and [FormatOptions] to a [String]
    ///
    /// If you need raw byte outputs, use [Format::format] instead.
    ///
    /// # Example
    ///
    /// ```
    /// use numf::format::{Format, FormatOptions};
    /// let mut options = FormatOptions::default();
    ///
    /// assert_eq!(Format::Bin.format_str(256, &options), "100000000");
    /// assert_eq!(Format::Hex.format_str(256, &options), "100");
    /// assert_eq!(Format::Base64.format_str(256, &options), "AQA=");
    ///
    /// options.set_prefix(true);
    /// options.set_padding(true);
    ///
    /// assert_eq!(Format::Bin.format_str(256, &options), "0b0000000100000000");
    /// assert_eq!(Format::Hex.format_str(256, &options), "0x0100");
    /// assert_eq!(Format::Base64.format_str(256, &options), "0sAQA=");
    ///
    /// ```
    pub fn format_str(&self, num: NumberType, options: &FormatOptions) -> String {
        String::from_utf8_lossy(&self.format(num, options)).to_string()
    }

    /// format a number with a [Format] and [FormatOptions] to a byte vector [Vec<u8>]
    ///
    /// If you need [String] outputs, use [Format::format_str] instead.
    ///
    /// # Example
    ///
    /// ```
    /// use numf::format::{Format, FormatOptions};
    /// let mut options = FormatOptions::default();
    ///
    /// assert_eq!(Format::Bin.format(256, &options), b"100000000");
    /// assert_eq!(Format::Hex.format(256, &options), b"100");
    /// assert_eq!(Format::Hex.format(256, &options), [49, 48, 48]);
    /// assert_eq!(Format::Base64.format(256, &options), b"AQA=");
    /// assert_eq!(Format::Raw.format(256, &options), [1, 0]);
    ///
    /// options.set_prefix(true);
    /// options.set_padding(true);
    ///
    /// assert_eq!(Format::Bin.format(256, &options), b"0b0000000100000000");
    /// assert_eq!(Format::Hex.format(256, &options), b"0x0100");
    /// assert_eq!(Format::Hex.format(256, &options), [48, 120, 48, 49, 48, 48]);
    /// assert_eq!(Format::Base64.format(256, &options), b"0sAQA=");
    /// assert_eq!(Format::Raw.format(256, &options), [0, 1, 0]);
    /// assert_eq!(Format::Raw.format(255, &options), [0, 255]);
    /// assert_eq!(Format::Raw.format(32000, &options), [0, 125, 0]);
    ///
    /// ```
    pub fn format(&self, num: NumberType, options: &FormatOptions) -> Vec<u8> {
        debug!("formatting mode: {self}");
        let mut buf: Vec<u8> = Vec::new();
        if options.prefix() {
            buf.append(&mut self.prefix());
            debug!("prefix the buffer: {buf:X?}");
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
            Format::Raw => buf.append(&mut split::unsigned_to_vec(num)),
        }
        buf
    }
}

/// Converts a &[str] into an unsigned integer value (like [u128]), according to one of the [Formats](Format)
///
/// The number is assumed to be base-10 by default, it is parsed as a different
/// [Format] if the number is prefixed with the [prefix](FormatOptions::prefix),
/// for that [Format]. So if the user inputs `0b1100` then this is parsed as
/// [Binary](Format::Bin) and so on.
///
/// If you also want to parse raw inputs, use [numf_parser].
///
/// # Returns
///
/// This parser will only output unsigned integers, it cannot be used with signed integers.
///
/// # Example
///
/// This allows base-10 addresses to be passed normally, or values formatted with any of the
/// [Formats](format::Format) defined by this crate to be passed when prefixed with the respective
/// prefix.
///
/// ```
/// use clap::Parser;
/// use numf::format::numf_parser_str;
///
/// #[derive(Parser)]
/// struct Args {
///     #[clap(short, long, value_parser=numf_parser_str::<u128>)]
///     address: u128,
/// }
/// let args = Args::parse_from(&["", "-a", "0x10"]);
/// assert_eq!(args.address, 16);
/// ```
pub fn numf_parser_str<T>(s: &str) -> anyhow::Result<T>
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
    numf_parser(s.as_bytes())
}

/// Converts any data (as bytes) into an unsigned integer value `T` (like [u128]), according to one of the [Formats](Format)
///
/// If you only want to parse text data, use [numf_parser_str] instead.
///
/// The parser will first try to convert the data to a [String].
///
/// Then, the number is assumed to be base-10 by default, it is parsed as a different
/// [Format] if the number is prefixed with the [prefix](FormatOptions::prefix),
/// for that [Format]. So if the user inputs `0b1100` then this is parsed as
/// [Binary](Format::Bin) and so on.
///
/// If none of the text [Formats](Format) matches, the data will be assumed to be raw and converted
/// to the ingeger type directly.
///
/// Note: Underscores will be completely ignored, as they are assumed to just be there for
/// readability.
///
/// # Errors
///
/// If no text [Format] matches and the data is too long for the integer `T`.
///
/// # Returns
///
/// This parser will only output unsigned integers, it cannot be used with signed integers.
///
/// # Example
///
/// ```
/// use numf::format::numf_parser;
///
/// let data = &[0x15, 0x92, 0xff];
/// let result: u64 = 0x1592ff;
/// assert_eq!(result, numf_parser(data).unwrap());
///
/// let data = b"0x1337";
/// let result: u64 = 0x1337;
/// assert_eq!(result, numf_parser(data).unwrap());
///
/// let data = b"0b110011";
/// let result: u64 = 0b110011;
/// assert_eq!(result, numf_parser(data).unwrap());
/// ```
pub fn numf_parser<T>(data: &[u8]) -> anyhow::Result<T>
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
    let data_as_text = String::from_utf8_lossy(data).to_string().replace("_", "");

    if data_as_text.starts_with(&Format::Dec.prefix_str()) || data_as_text.parse::<T>().is_ok() {
        let s = match data_as_text.strip_prefix(&Format::Dec.prefix_str()) {
            Some(sr) => sr,
            None => &data_as_text,
        };
        match s.parse() {
            Ok(r) => Ok(r),
            Err(e) => {
                let e = format!("{e}");
                Err(anyhow!(e))
            }
        }
    } else if data_as_text.starts_with(&Format::Hex.prefix_str()) {
        let s = match data_as_text.strip_prefix(&Format::Hex.prefix_str()) {
            Some(sr) => sr,
            None => &data_as_text,
        };
        match T::from_str_radix(s, 16) {
            Ok(r) => Ok(r),
            Err(e) => {
                let e = format!("{e}");
                Err(anyhow!(e))
            }
        }
    } else if data_as_text.starts_with(&Format::Octal.prefix_str()) {
        let s = match data_as_text.strip_prefix(&Format::Octal.prefix_str()) {
            Some(sr) => sr,
            None => &data_as_text,
        };
        match T::from_str_radix(s, 8) {
            Ok(r) => Ok(r),
            Err(e) => {
                let e = format!("{e}");
                Err(anyhow!(e))
            }
        }
    } else if data_as_text.starts_with(&Format::Bin.prefix_str()) {
        let s = match data_as_text.strip_prefix(&Format::Bin.prefix_str()) {
            Some(sr) => sr,
            None => &data_as_text,
        };
        match T::from_str_radix(s, 2) {
            Ok(r) => Ok(r),
            Err(e) => {
                let e = format!("{e}");
                Err(anyhow!(e))
            }
        }
    } else if data_as_text.starts_with(&Format::Base64.prefix_str()) {
        let s = match data_as_text.strip_prefix(&Format::Base64.prefix_str()) {
            Some(sr) => sr,
            None => &data_as_text,
        };
        match fast32::base64::RFC4648.decode_str(s) {
            Ok(r) => Ok(join::array_to_unsigned::<T>(&r)?),
            Err(e) => {
                let e = format!("{e}");
                Err(anyhow!(e))
            }
        }
    } else if data_as_text.starts_with(&Format::Base32.prefix_str()) {
        let s = match data_as_text.strip_prefix(&Format::Base32.prefix_str()) {
            Some(sr) => sr,
            None => &data_as_text,
        };
        match fast32::base32::RFC4648.decode_str(s) {
            Ok(r) => Ok(join::array_to_unsigned::<T>(&r)?),
            Err(e) => {
                let e = format!("{e}");
                Err(anyhow!(e))
            }
        }
    } else {
        // what could go wrong with interpreting everything else as raw number input
        let s: Vec<u8> = if data.len() > 2 && data[0] == 0x00 {
            data.iter().skip(1).map(ToOwned::to_owned).collect()
        } else {
            data.as_ref().to_vec()
        };
        Ok(join::array_to_unsigned(&s)?)
    }
}
