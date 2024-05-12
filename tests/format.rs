use numf::format::*;

#[test]
fn format() {
    let options = FormatOptions::default();
    assert_eq!(Format::Dec.format(1337, &options), "1337");
    assert_eq!(Format::Dec.format(u128::MAX, &options), format!("{}", u128::MAX));

    assert_eq!(Format::Hex.format(0x1337, &options), "1337");
    assert_eq!(Format::Hex.format(u128::MAX, &options), format!("{:X}", u128::MAX));

    assert_eq!(Format::Bin.format(0b1010001001010010010100111, &options), "1010001001010010010100111");
    assert_eq!(Format::Bin.format(u128::MAX, &options), format!("{:b}", u128::MAX));

    assert_eq!(Format::Octal.format(0o13377331, &options), "13377331");
    assert_eq!(Format::Octal.format(u128::MAX, &options), format!("{:o}", u128::MAX));

    assert_eq!(Format::Base32.format(0x41414242, &options), "IFAUEQQ=");
    assert_eq!(Format::Base32.format(0x4141414141414141, &options), "IFAUCQKBIFAUC===");

    assert_eq!(Format::Base64.format(0x41414242, &options), "QUFCQg==");
    assert_eq!(Format::Base64.format(0x4141414141414141, &options), "QUFBQUFBQUE=");
}
