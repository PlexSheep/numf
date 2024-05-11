use fast32;

pub type Num = u128;

#[derive(Copy, Clone, Debug)]
pub enum Format {
    Dec,
    Hex,
    Bin,
    Octal,
    Base64,
    Base32,
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
            Format::Base64 => buf += &fast32::base64::RFC4648.encode(&u128_to_u8_slice(num)),
            Format::Base32 => buf += &fast32::base32::RFC4648.encode(&u128_to_u8_slice(num)),
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
