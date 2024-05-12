use libpt::bintols::split;

pub type Num = u128;

/// formats supported by numf
#[derive(Copy, Clone, Debug)]
pub enum Format {
    Dec,
    Hex,
    Bin,
    Octal,
    Base64,
    Base32,
}

/// Options to use when formatting a number
///
/// Used by [Format::format].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct FormatOptions {
    /// add a prefix to the formatted number, such as `0x` for hex
    prefix: bool,
    /// fill the formatted number with zeros (or the equivalent) to make a whole byte
    padding: bool,
}

impl FormatOptions {
    /// set prefix
    pub fn prefix(mut self, value: bool) -> Self {
        self.prefix = value;
        self
    }
    /// set padding
    ///
    /// Does not apply to all formats
    pub fn padding(mut self, value: bool) -> Self {
        self.padding = value;
        self
    }
}

impl Format {
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
    pub fn format(&self, num: Num, options: FormatOptions) -> String {
        let mut buf = String::new();
        if options.prefix {
            buf += &self.prefix();
        }
        match self {
            Format::Hex => {
                if options.padding {
                    let tmp = &format!("{num:X}");
                    buf += &("0".repeat((2 - tmp.len() % 2) % 2) + tmp);
                } else {
                    buf += &format!("{num:X}");
                }
            }
            Format::Bin => {
                if options.padding {
                    let tmp = &format!("{num:b}");
                    buf += &("0".repeat((8 - tmp.len() % 8) % 8) + tmp);
                } else {
                    buf += &format!("{num:b}");
                }
            }
            Format::Octal => {
                buf += &format!("{num:o}");
            }
            Format::Dec => {
                buf += &format!("{num}");
            }
            Format::Base64 => buf += &fast32::base64::RFC4648.encode(&split::unsigned_to_vec(num)),
            Format::Base32 => buf += &fast32::base32::RFC4648.encode(&split::unsigned_to_vec(num)),
        }
        buf
    }
}
