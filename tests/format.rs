use numf::format::*;

#[test]
fn format() {
    let options = FormatOptions::default();
    assert_eq!(Format::Dec.format(1337, &options), "1337");
    assert_eq!(
        Format::Dec.format(u128::MAX, &options),
        format!("{}", u128::MAX)
    );

    assert_eq!(Format::Hex.format(0x1337, &options), "1337");
    assert_eq!(
        Format::Hex.format(u128::MAX, &options),
        format!("{:X}", u128::MAX)
    );

    assert_eq!(
        Format::Bin.format(0b1010001001010010010100111, &options),
        "1010001001010010010100111"
    );
    assert_eq!(
        Format::Bin.format(u128::MAX, &options),
        format!("{:b}", u128::MAX)
    );

    assert_eq!(Format::Octal.format(0o13377331, &options), "13377331");
    assert_eq!(
        Format::Octal.format(u128::MAX, &options),
        format!("{:o}", u128::MAX)
    );

    assert_eq!(Format::Base32.format(0x41414242, &options), "IFAUEQQ=");
    assert_eq!(
        Format::Base32.format(0x4141414141414141, &options),
        "IFAUCQKBIFAUC==="
    );

    assert_eq!(Format::Base64.format(0x41414242, &options), "QUFCQg==");
    assert_eq!(
        Format::Base64.format(0x4141414141414141, &options),
        "QUFBQUFBQUE="
    );
}

#[test]
fn format_padding() {
    let mut options = FormatOptions::default();
    options.set_padding(true);

    assert_eq!(Format::Dec.format(1337, &options), "1337");
    assert_eq!(
        Format::Dec.format(u128::MAX, &options),
        format!("{}", u128::MAX)
    );

    assert_eq!(Format::Hex.format(0xFFF, &options), "0FFF");
    assert_eq!(Format::Hex.format(0xFFFF, &options), "FFFF");
    assert_eq!(
        Format::Hex.format(u128::MAX, &options),
        format!("{:X}", u128::MAX)
    );

    assert_eq!(
        Format::Bin.format(0b11110000_00001111, &options),
        "1111000000001111"
    );
    assert_eq!(
        Format::Bin.format(0b110000_00001111, &options),
        "0011000000001111"
    );
    assert_eq!(
        Format::Bin.format(u128::MAX, &options),
        format!("{:b}", u128::MAX)
    );

    assert_eq!(Format::Octal.format(0o13377331, &options), "13377331");
    assert_eq!(
        Format::Octal.format(u128::MAX, &options),
        format!("{:o}", u128::MAX)
    );

    assert_eq!(Format::Base32.format(0x41414242, &options), "IFAUEQQ=");
    assert_eq!(
        Format::Base32.format(0x4141414141414141, &options),
        "IFAUCQKBIFAUC==="
    );

    assert_eq!(Format::Base64.format(0x41414242, &options), "QUFCQg==");
    assert_eq!(
        Format::Base64.format(0x4141414141414141, &options),
        "QUFBQUFBQUE="
    );
}

#[test]
fn format_prefix() {
    let mut options = FormatOptions::default();
    options.set_prefix(true);

    assert_eq!(Format::Dec.format(1337, &options), "0d1337");
    assert_eq!(
        Format::Dec.format(u128::MAX, &options),
        format!("0d{}", u128::MAX)
    );

    assert_eq!(Format::Hex.format(0x1337, &options), "0x1337");
    assert_eq!(
        Format::Hex.format(u128::MAX, &options),
        format!("0x{:X}", u128::MAX)
    );

    assert_eq!(
        Format::Bin.format(0b1010001001010010010100111, &options),
        "0b1010001001010010010100111"
    );
    assert_eq!(
        Format::Bin.format(u128::MAX, &options),
        format!("0b{:b}", u128::MAX)
    );

    assert_eq!(Format::Octal.format(0o13377331, &options), "0o13377331");
    assert_eq!(
        Format::Octal.format(u128::MAX, &options),
        format!("0o{:o}", u128::MAX)
    );

    assert_eq!(Format::Base32.format(0x41414242, &options), "032sIFAUEQQ=");
    assert_eq!(
        Format::Base32.format(0x4141414141414141, &options),
        "032sIFAUCQKBIFAUC==="
    );

    assert_eq!(Format::Base64.format(0x41414242, &options), "0sQUFCQg==");
    assert_eq!(
        Format::Base64.format(0x4141414141414141, &options),
        "0sQUFBQUFBQUE="
    );
}

#[test]
fn format_padded_prefix() {
    let mut options = FormatOptions::default();
    options.set_prefix(true);
    options.set_padding(true);

    assert_eq!(Format::Dec.format(1337, &options), "0d1337");
    assert_eq!(
        Format::Dec.format(u128::MAX, &options),
        format!("0d{}", u128::MAX)
    );

    assert_eq!(Format::Hex.format(0xFFF, &options), "0x0FFF");
    assert_eq!(Format::Hex.format(0xFFFF, &options), "0xFFFF");
    assert_eq!(
        Format::Hex.format(u128::MAX, &options),
        format!("0x{:X}", u128::MAX)
    );

    assert_eq!(
        Format::Bin.format(0b11110000_00001111, &options),
        "0b1111000000001111"
    );
    assert_eq!(
        Format::Bin.format(0b110000_00001111, &options),
        "0b0011000000001111"
    );
    assert_eq!(
        Format::Bin.format(u128::MAX, &options),
        format!("0b{:b}", u128::MAX)
    );

    assert_eq!(Format::Octal.format(0o13377331, &options), "0o13377331");
    assert_eq!(
        Format::Octal.format(u128::MAX, &options),
        format!("0o{:o}", u128::MAX)
    );

    assert_eq!(Format::Base32.format(0x41414242, &options), "032sIFAUEQQ=");
    assert_eq!(
        Format::Base32.format(0x4141414141414141, &options),
        "032sIFAUCQKBIFAUC==="
    );

    assert_eq!(Format::Base64.format(0x41414242, &options), "0sQUFCQg==");
    assert_eq!(
        Format::Base64.format(0x4141414141414141, &options),
        "0sQUFBQUFBQUE="
    );
}


#[test]
fn set_format_checker() {
    let mut options = FormatOptions::default();
    assert_eq!(options.format(), Format::Hex);
    options.set_format(Format::Base32);
    assert_eq!(options.format(), Format::Base32);
    options.set_format(Format::Base64);
    assert_eq!(options.format(), Format::Base64);
}
