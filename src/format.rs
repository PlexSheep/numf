pub type Num = u128;

#[derive(Copy, Clone, Debug)]
pub enum Format {
    Dec,
    Hex,
    Bin,
    Octal,
}

impl Format {
    pub fn prefix(&self) -> String {
        match self {
            Format::Dec => "0d",
            Format::Hex => "0x",
            Format::Bin => "0b",
            Format::Octal => "0o",
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
        }
        buf
    }
}
