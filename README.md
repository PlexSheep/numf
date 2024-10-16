# numf

![Project badge](https://img.shields.io/badge/language-Rust-blue.svg)
![Crates.io License](https://img.shields.io/crates/l/numf)
![GitHub Release](https://img.shields.io/github/v/release/PlexSheep/numf)
![GitHub language count](https://img.shields.io/github/languages/count/PlexSheep/numf)
[![Rust CI](https://github.com/PlexSheep/numf/actions/workflows/cargo.yaml/badge.svg)](https://github.com/PlexSheep/numf/actions/workflows/cargo.yaml)

* [GitHub](https://github.com/PlexSheep/numf)
* [crates.io](https://crates.io/crates/numf)
* [docs.rs](https://docs.rs/numf/latest/numf/)

`numf` is a number formatter. It formats the numbers provided to it.

Current formats are:

- Hexadecimal
- Binary
- Octal
- Decimal
- Base32
- Base64
- Raw

`numf` also has the option of prepending a prefix for the formats, such as
`0x` for hexadecimal. Numbers may also be provided from the stdin. See `--help`
flag for more information.

## Example

```bash
$ numf -xp 1337 505 0xaabb
0x539
0x1F9
0xAABB
$ numf -a 505 | hedxump -C
00000000  01 f9                                             |..|
00000002
$ numf -a 505 | numf
1F9
$ numf -a 505 | numf -d
505
$ numf -a 505 | numf -b
111111001
$ echo -ne "\x20\xff\xb4" | numf -xpP
0x20FFB4
$ echo -ne "\x20\xff\xb4" | numf -d
2162612
$ base64='aGVsbG8gd29ybGQuCg==' ; echo "0s$base64" | numf -d
8271117963529473544792763018762
$ base64='aGVsbG8gd29ybGQuCg==' ; echo "0s$base64" | numf -s
aGVsbG8gd29ybGQuCg==
$ echo "0b100100101010" | numf -d
2346
$ echo "0b100100101010" | numf -bPp
0b0000100100101010
```
## Installing

### Cargo

`numf` is on [crates.io](https://crates.io).

```
cargo install numf
```
