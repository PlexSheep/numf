use numf::format::*;

#[test]
fn format() {
    let options = FormatOptions::default();
    assert_eq!(Format::Dec.format_str(1337, &options), "1337");
    assert_eq!(
        Format::Dec.format_str(u128::MAX, &options),
        format!("{}", u128::MAX)
    );

    assert_eq!(Format::Hex.format_str(0x1337, &options), "1337");
    assert_eq!(
        Format::Hex.format_str(u128::MAX, &options),
        format!("{:X}", u128::MAX)
    );

    assert_eq!(
        Format::Bin.format_str(0b1010001001010010010100111, &options),
        "1010001001010010010100111"
    );
    assert_eq!(
        Format::Bin.format_str(u128::MAX, &options),
        format!("{:b}", u128::MAX)
    );

    assert_eq!(Format::Octal.format_str(0o13377331, &options), "13377331");
    assert_eq!(
        Format::Octal.format_str(u128::MAX, &options),
        format!("{:o}", u128::MAX)
    );

    assert_eq!(Format::Base32.format_str(0x41414242, &options), "IFAUEQQ=");
    assert_eq!(
        Format::Base32.format_str(0x4141414141414141, &options),
        "IFAUCQKBIFAUC==="
    );

    assert_eq!(Format::Base64.format_str(0x41414242, &options), "QUFCQg==");
    assert_eq!(
        Format::Base64.format_str(0x4141414141414141, &options),
        "QUFBQUFBQUE="
    );

    assert_eq!(Format::Raw.format(0x1337, &options), vec![0x13, 0x37]);
    assert_eq!(Format::Raw.format(0x0, &options), vec![0x0]);
}

#[test]
fn format_padding() {
    let mut options = FormatOptions::default();
    options.set_padding(true);

    assert_eq!(Format::Dec.format_str(1337, &options), "1337");
    assert_eq!(
        Format::Dec.format_str(u128::MAX, &options),
        format!("{}", u128::MAX)
    );

    assert_eq!(Format::Hex.format_str(0xFFF, &options), "0FFF");
    assert_eq!(Format::Hex.format_str(0xFFFF, &options), "FFFF");
    assert_eq!(
        Format::Hex.format_str(u128::MAX, &options),
        format!("{:X}", u128::MAX)
    );

    assert_eq!(
        Format::Bin.format_str(0b11110000_00001111, &options),
        "1111000000001111"
    );
    assert_eq!(
        Format::Bin.format_str(0b110000_00001111, &options),
        "0011000000001111"
    );
    assert_eq!(
        Format::Bin.format_str(u128::MAX, &options),
        format!("{:b}", u128::MAX)
    );

    assert_eq!(Format::Octal.format_str(0o13377331, &options), "13377331");
    assert_eq!(
        Format::Octal.format_str(u128::MAX, &options),
        format!("{:o}", u128::MAX)
    );

    assert_eq!(Format::Base32.format_str(0x41414242, &options), "IFAUEQQ=");
    assert_eq!(
        Format::Base32.format_str(0x4141414141414141, &options),
        "IFAUCQKBIFAUC==="
    );

    assert_eq!(Format::Base64.format_str(0x41414242, &options), "QUFCQg==");
    assert_eq!(
        Format::Base64.format_str(0x4141414141414141, &options),
        "QUFBQUFBQUE="
    );

    assert_eq!(Format::Raw.format(0x1337, &options), vec![0x13, 0x37]);
    assert_eq!(Format::Raw.format(0x0, &options), vec![0x0]);
}

#[test]
fn format_prefix() {
    let mut options = FormatOptions::default();
    options.set_prefix(true);

    assert_eq!(Format::Dec.format_str(1337, &options), "0d1337");
    assert_eq!(
        Format::Dec.format_str(u128::MAX, &options),
        format!("0d{}", u128::MAX)
    );

    assert_eq!(Format::Hex.format_str(0x1337, &options), "0x1337");
    assert_eq!(
        Format::Hex.format_str(u128::MAX, &options),
        format!("0x{:X}", u128::MAX)
    );

    assert_eq!(
        Format::Bin.format_str(0b1010001001010010010100111, &options),
        "0b1010001001010010010100111"
    );
    assert_eq!(
        Format::Bin.format_str(u128::MAX, &options),
        format!("0b{:b}", u128::MAX)
    );

    assert_eq!(Format::Octal.format_str(0o13377331, &options), "0o13377331");
    assert_eq!(
        Format::Octal.format_str(u128::MAX, &options),
        format!("0o{:o}", u128::MAX)
    );

    assert_eq!(
        Format::Base32.format_str(0x41414242, &options),
        "032sIFAUEQQ="
    );
    assert_eq!(
        Format::Base32.format_str(0x4141414141414141, &options),
        "032sIFAUCQKBIFAUC==="
    );

    assert_eq!(
        Format::Base64.format_str(0x41414242, &options),
        "0sQUFCQg=="
    );
    assert_eq!(
        Format::Base64.format_str(0x4141414141414141, &options),
        "0sQUFBQUFBQUE="
    );

    assert_eq!(Format::Raw.format(0x1337, &options), vec![0x0, 0x13, 0x37]);
    assert_eq!(Format::Raw.format(0x0, &options), vec![0x0, 0x0]);
}

#[test]
fn format_padded_prefix() {
    let mut options = FormatOptions::default();
    options.set_prefix(true);
    options.set_padding(true);

    assert_eq!(Format::Dec.format_str(1337, &options), "0d1337");
    assert_eq!(
        Format::Dec.format_str(u128::MAX, &options),
        format!("0d{}", u128::MAX)
    );

    assert_eq!(Format::Hex.format_str(0xFFF, &options), "0x0FFF");
    assert_eq!(Format::Hex.format_str(0xFFFF, &options), "0xFFFF");
    assert_eq!(
        Format::Hex.format_str(u128::MAX, &options),
        format!("0x{:X}", u128::MAX)
    );

    assert_eq!(
        Format::Bin.format_str(0b11110000_00001111, &options),
        "0b1111000000001111"
    );
    assert_eq!(
        Format::Bin.format_str(0b110000_00001111, &options),
        "0b0011000000001111"
    );
    assert_eq!(
        Format::Bin.format_str(u128::MAX, &options),
        format!("0b{:b}", u128::MAX)
    );

    assert_eq!(Format::Octal.format_str(0o13377331, &options), "0o13377331");
    assert_eq!(
        Format::Octal.format_str(u128::MAX, &options),
        format!("0o{:o}", u128::MAX)
    );

    assert_eq!(
        Format::Base32.format_str(0x41414242, &options),
        "032sIFAUEQQ="
    );
    assert_eq!(
        Format::Base32.format_str(0x4141414141414141, &options),
        "032sIFAUCQKBIFAUC==="
    );

    assert_eq!(
        Format::Base64.format_str(0x41414242, &options),
        "0sQUFCQg=="
    );
    assert_eq!(
        Format::Base64.format_str(0x4141414141414141, &options),
        "0sQUFBQUFBQUE="
    );

    assert_eq!(Format::Raw.format(0x1337, &options), vec![0x0, 0x13, 0x37]);
    assert_eq!(Format::Raw.format(0x0, &options), vec![0x0, 0x0]);
}

#[test]
fn set_format_checker() {
    let mut options = FormatOptions::default();
    assert_eq!(options.format(), Format::Hex);
    options.set_format(Format::Base32);
    assert_eq!(options.format(), Format::Base32);
    options.set_format(Format::Base64);
    assert_eq!(options.format(), Format::Base64);
    options.set_format(Format::Raw);
    assert_eq!(options.format(), Format::Raw);
}

#[test]
fn parser_dec() {
    assert_eq!(numf_parser_str::<u32>("1337").unwrap(), 1337);
    assert_eq!(numf_parser_str::<u32>("0d1337").unwrap(), 1337);
}

#[test]
fn parser_bin() {
    assert_eq!(numf_parser_str::<u32>("0b11001").unwrap(), 0b11001);
    assert_eq!(numf_parser_str::<u32>("0b11001").unwrap(), 0b11001);
}

#[test]
fn parser_hex() {
    assert_eq!(numf_parser_str::<u32>("0xdeadbeef").unwrap(), 0xdeadbeef);
}

#[test]
fn parser_oct() {
    assert_eq!(numf_parser_str::<u32>("0o771171").unwrap(), 0o771171);
}

#[test]
fn parser_b64() {
    assert_eq!(numf_parser_str::<u32>("0sQUFCQg==").unwrap(), 0x41414242);
}

#[test]
fn parser_b32() {
    assert_eq!(numf_parser_str::<u32>("032sIFAUEQQ=").unwrap(), 0x41414242);
}

#[test]
fn parser_raw() {
    assert_eq!(numf_parser_str::<u32>("\x00\x50\x60").unwrap(), 0x5060);
}

#[test]
fn parser_generics() {
    assert_eq!(numf_parser_str::<u8>("55").unwrap(), 55);
    assert_eq!(numf_parser_str::<u16>("55").unwrap(), 55);
    assert_eq!(numf_parser_str::<u32>("55").unwrap(), 55);
    assert_eq!(numf_parser_str::<u64>("55").unwrap(), 55);
    assert_eq!(numf_parser_str::<u128>("55").unwrap(), 55);
}

#[test]
fn parser_underscores() {
    assert_eq!(numf_parser_str::<u16>("5_500").unwrap(), 5_500);
    assert_eq!(
        numf_parser_str::<u64>("0xffffffff_00110011").unwrap(),
        0xffffffff_00110011
    );
}
