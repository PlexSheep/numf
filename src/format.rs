use base64::prelude::*;

pub type Num = u128;

#[derive(Copy, Clone, Debug)]
pub enum Format {
    Dec,
    Hex,
    Bin,
    Octal,
    Base64,
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
        }
        .to_string()
    }
    pub fn format(&self, num: Num, prefix: bool) -> String {
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
            Format::Base64 => buf += &BASE64_STANDARD.encode(u128_to_u8_slice(num)),
        }
        buf
    }
}

fn u128_to_u8_slice(mut num: u128) -> Vec<u8> {
    if num == 0 {
        return vec![0];
    }
    let mut buf: Vec<u8> = Vec::new();
    while num > 0 {
        buf.push(num as u8);
        num >>= 8;
    }
    buf
}
