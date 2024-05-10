# numf

`numf` is a number formatter. It formats the numbers provided to it.

Current formats are:

- Hexadecimal
- Binary
- Octal
- Decimal

`numf` also has the option of prepending a prefix for each format, such as
`0x` for hexadecimal.

## Example

```bash
$ numf -xp 1337 505 0xaabb
0x539
0x1F9
0xAABB
```
