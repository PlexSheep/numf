# numf


![Project badge](https://img.shields.io/badge/language-Rust-blue.svg)
![Crates.io License](https://img.shields.io/crates/l/numf)
![Gitea Release](https://img.shields.io/gitea/v/release/PlexSheep/numf?gitea_url=https%3A%2F%2Fgit.cscherr.de)
![Gitea language count](https://img.shields.io/gitea/languages/count/PlexSheep/numf?gitea_url=https%3A%2F%2Fgit.cscherr.de)
[![cargo checks and tests](https://github.com/PlexSheep/numf/actions/workflows/cargo.yaml/badge.svg)](https://github.com/PlexSheep/numf/actions/workflows/cargo.yaml)

* [Original Repository](https://git.cscherr.de/PlexSheep/numf)
* [GitHub Mirror](https://github.com/PlexSheep/numf)
* [Codeberg Mirror](https://codeberg.org/PlexSheep/numf)
* [crates.io](https://crates.io/crates/numf)
* [docs.rs](https://docs.rs/crate/numf/)

`numf` is a number formatter. It formats the numbers provided to it.

Current formats are:

- Hexadecimal
- Binary
- Octal
- Decimal
- Base32
- Base64

`numf` also has the onumfion of prepending a prefix for each format, such as
`0x` for hexadecimal. Numbers may also be provided from the stdin. See `--help`
flag for more information.

## Example

```bash
$ numf -xp 1337 505 0xaabb
0x539
0x1F9
0xAABB
```
