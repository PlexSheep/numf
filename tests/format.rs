use numf::format::*;

#[test]
fn test_format() {
    let options = FormatOptions::default();
    assert_eq!(Format::Dec.format_str(1337, &options).unwrap(), "1337");
    assert_eq!(
        Format::Dec.format_str(u128::MAX, &options).unwrap(),
        format!("{}", u128::MAX)
    );

    assert_eq!(Format::Hex.format_str(0x1337, &options).unwrap(), "1337");
    assert_eq!(
        Format::Hex.format_str(u128::MAX, &options).unwrap(),
        format!("{:X}", u128::MAX)
    );

    assert_eq!(
        Format::Bin
            .format_str(0b1010001001010010010100111, &options)
            .unwrap(),
        "1010001001010010010100111"
    );
    assert_eq!(
        Format::Bin.format_str(u128::MAX, &options).unwrap(),
        format!("{:b}", u128::MAX)
    );

    assert_eq!(
        Format::Octal.format_str(0o13377331, &options).unwrap(),
        "13377331"
    );
    assert_eq!(
        Format::Octal.format_str(u128::MAX, &options).unwrap(),
        format!("{:o}", u128::MAX)
    );

    assert_eq!(
        Format::Base32.format_str(0x41414242, &options).unwrap(),
        "IFAUEQQ="
    );
    assert_eq!(
        Format::Base32
            .format_str(0x4141414141414141, &options)
            .unwrap(),
        "IFAUCQKBIFAUC==="
    );

    assert_eq!(
        Format::Base64.format_str(0x41414242, &options).unwrap(),
        "QUFCQg=="
    );
    assert_eq!(
        Format::Base64
            .format_str(0x4141414141414141, &options)
            .unwrap(),
        "QUFBQUFBQUE="
    );

    assert_eq!(
        Format::Raw.format(0x1337, &options).unwrap(),
        vec![0x13, 0x37]
    );
    assert_eq!(Format::Raw.format(0x0, &options).unwrap(), vec![0x0]);

    assert_eq!(
        Format::DecSigned(16 /* 16 bit integer */)
            .format_str(0xffff, &options)
            .unwrap(),
        "-1"
    );
    assert_eq!(
        Format::DecSigned(16 /* 16 bit integer */)
            .format_str(0xfffe, &options)
            .unwrap(),
        "-2"
    );
    assert_eq!(
        Format::DecSigned(8 /* 8 bit integer */)
            .format_str(0xff, &options)
            .unwrap(),
        "-1"
    );
    assert_eq!(
        Format::DecSigned(4 /* 8 bit integer */)
            .format_str(0xf, &options)
            .unwrap(),
        "-1"
    );
    assert_eq!(
        Format::DecSigned(4 /* 8 bit integer */)
            .format_str(0xe, &options)
            .unwrap(),
        "-2"
    );
    assert_eq!(
        Format::DecSigned(5 /* 5 bit integer */)
            .format_str(0x1f, &options)
            .unwrap(),
        "-1"
    );
}

#[test]
fn test_format_padding() {
    let mut options = FormatOptions::default();
    options.set_padding(true);

    assert_eq!(Format::Dec.format_str(1337, &options).unwrap(), "1337");
    assert_eq!(
        Format::Dec.format_str(u128::MAX, &options).unwrap(),
        format!("{}", u128::MAX)
    );

    assert_eq!(Format::Hex.format_str(0xFFF, &options).unwrap(), "0FFF");
    assert_eq!(Format::Hex.format_str(0xFFFF, &options).unwrap(), "FFFF");
    assert_eq!(
        Format::Hex.format_str(u128::MAX, &options).unwrap(),
        format!("{:X}", u128::MAX)
    );

    assert_eq!(
        Format::Bin
            .format_str(0b11110000_00001111, &options)
            .unwrap(),
        "1111000000001111"
    );
    assert_eq!(
        Format::Bin.format_str(0b110000_00001111, &options).unwrap(),
        "0011000000001111"
    );
    assert_eq!(
        Format::Bin.format_str(u128::MAX, &options).unwrap(),
        format!("{:b}", u128::MAX)
    );

    assert_eq!(
        Format::Octal.format_str(0o13377331, &options).unwrap(),
        "13377331"
    );
    assert_eq!(
        Format::Octal.format_str(u128::MAX, &options).unwrap(),
        format!("{:o}", u128::MAX)
    );

    assert_eq!(
        Format::Base32.format_str(0x41414242, &options).unwrap(),
        "IFAUEQQ="
    );
    assert_eq!(
        Format::Base32
            .format_str(0x4141414141414141, &options)
            .unwrap(),
        "IFAUCQKBIFAUC==="
    );

    assert_eq!(
        Format::Base64.format_str(0x41414242, &options).unwrap(),
        "QUFCQg=="
    );
    assert_eq!(
        Format::Base64
            .format_str(0x4141414141414141, &options)
            .unwrap(),
        "QUFBQUFBQUE="
    );

    assert_eq!(
        Format::Raw.format(0x1337, &options).unwrap(),
        vec![0x13, 0x37]
    );
    assert_eq!(Format::Raw.format(0x0, &options).unwrap(), vec![0x0]);
}

#[test]
fn test_format_prefix() {
    let mut options = FormatOptions::default();
    options.set_prefix(true);

    assert_eq!(Format::Dec.format_str(1337, &options).unwrap(), "0d1337");
    assert_eq!(
        Format::Dec.format_str(u128::MAX, &options).unwrap(),
        format!("0d{}", u128::MAX)
    );

    assert_eq!(Format::Hex.format_str(0x1337, &options).unwrap(), "0x1337");
    assert_eq!(
        Format::Hex.format_str(u128::MAX, &options).unwrap(),
        format!("0x{:X}", u128::MAX)
    );

    assert_eq!(
        Format::Bin
            .format_str(0b1010001001010010010100111, &options)
            .unwrap(),
        "0b1010001001010010010100111"
    );
    assert_eq!(
        Format::Bin.format_str(u128::MAX, &options).unwrap(),
        format!("0b{:b}", u128::MAX)
    );

    assert_eq!(
        Format::Octal.format_str(0o13377331, &options).unwrap(),
        "0o13377331"
    );
    assert_eq!(
        Format::Octal.format_str(u128::MAX, &options).unwrap(),
        format!("0o{:o}", u128::MAX)
    );

    assert_eq!(
        Format::Base32.format_str(0x41414242, &options).unwrap(),
        "032sIFAUEQQ="
    );
    assert_eq!(
        Format::Base32
            .format_str(0x4141414141414141, &options)
            .unwrap(),
        "032sIFAUCQKBIFAUC==="
    );

    assert_eq!(
        Format::Base64.format_str(0x41414242, &options).unwrap(),
        "0sQUFCQg=="
    );
    assert_eq!(
        Format::Base64
            .format_str(0x4141414141414141, &options)
            .unwrap(),
        "0sQUFBQUFBQUE="
    );

    assert_eq!(
        Format::Raw.format(0x1337, &options).unwrap(),
        vec![0x0, 0x13, 0x37]
    );
    assert_eq!(Format::Raw.format(0x0, &options).unwrap(), vec![0x0, 0x0]);
}

#[test]
fn test_format_padded_prefix() {
    let mut options = FormatOptions::default();
    options.set_prefix(true);
    options.set_padding(true);

    assert_eq!(Format::Dec.format_str(1337, &options).unwrap(), "0d1337");
    assert_eq!(
        Format::Dec.format_str(u128::MAX, &options).unwrap(),
        format!("0d{}", u128::MAX)
    );

    assert_eq!(Format::Hex.format_str(0xFFF, &options).unwrap(), "0x0FFF");
    assert_eq!(Format::Hex.format_str(0xFFFF, &options).unwrap(), "0xFFFF");
    assert_eq!(
        Format::Hex.format_str(u128::MAX, &options).unwrap(),
        format!("0x{:X}", u128::MAX)
    );

    assert_eq!(
        Format::Bin
            .format_str(0b11110000_00001111, &options)
            .unwrap(),
        "0b1111000000001111"
    );
    assert_eq!(
        Format::Bin.format_str(0b110000_00001111, &options).unwrap(),
        "0b0011000000001111"
    );
    assert_eq!(
        Format::Bin.format_str(u128::MAX, &options).unwrap(),
        format!("0b{:b}", u128::MAX)
    );

    assert_eq!(
        Format::Octal.format_str(0o13377331, &options).unwrap(),
        "0o13377331"
    );
    assert_eq!(
        Format::Octal.format_str(u128::MAX, &options).unwrap(),
        format!("0o{:o}", u128::MAX)
    );

    assert_eq!(
        Format::Base32.format_str(0x41414242, &options).unwrap(),
        "032sIFAUEQQ="
    );
    assert_eq!(
        Format::Base32
            .format_str(0x4141414141414141, &options)
            .unwrap(),
        "032sIFAUCQKBIFAUC==="
    );

    assert_eq!(
        Format::Base64.format_str(0x41414242, &options).unwrap(),
        "0sQUFCQg=="
    );
    assert_eq!(
        Format::Base64
            .format_str(0x4141414141414141, &options)
            .unwrap(),
        "0sQUFBQUFBQUE="
    );

    assert_eq!(
        Format::Raw.format(0x1337, &options).unwrap(),
        vec![0x0, 0x13, 0x37]
    );
    assert_eq!(Format::Raw.format(0x0, &options).unwrap(), vec![0x0, 0x0]);
}

#[test]
fn test_set_format_checker() {
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
fn test_parser_dec() {
    assert_eq!(numf_parser_str::<u32>("1337").unwrap(), 1337);
    assert_eq!(numf_parser_str::<u32>("0d1337").unwrap(), 1337);
}

#[test]
fn test_parser_bin() {
    assert_eq!(numf_parser_str::<u32>("0b11001").unwrap(), 0b11001);
    assert_eq!(numf_parser_str::<u32>("0b11001").unwrap(), 0b11001);
}

#[test]
fn test_parser_hex() {
    assert_eq!(numf_parser_str::<u32>("0xdeadbeef").unwrap(), 0xdeadbeef);
}

#[test]
fn test_parser_oct() {
    assert_eq!(numf_parser_str::<u32>("0o771171").unwrap(), 0o771171);
}

#[test]
fn test_parser_b64() {
    assert_eq!(numf_parser_str::<u32>("0sQUFCQg==").unwrap(), 0x41414242);
}

#[test]
fn test_parser_b32() {
    assert_eq!(numf_parser_str::<u32>("032sIFAUEQQ=").unwrap(), 0x41414242);
}

#[test]
fn test_parser_raw() {
    assert_eq!(numf_parser_str::<u32>("\x00\x50\x60").unwrap(), 0x5060);
}

#[test]
fn test_parser_dec_signed() {
    assert_eq!(numf_parser_str::<u16>("-1").unwrap(), 0xffff);
    assert_eq!(numf_parser_str::<u16>("-2").unwrap(), 0xfffe);
    assert_eq!(numf_parser_str::<u16>("-0d2").unwrap(), 0xfffe);
    assert_eq!(numf_parser_str::<u16>("-0d1").unwrap(), 0xffff);
    assert_eq!(numf_parser_str::<u16>("2").unwrap(), 2);
}

#[test]
fn test_parser_generics() {
    assert_eq!(numf_parser_str::<u8>("55").unwrap(), 55);
    assert_eq!(numf_parser_str::<u16>("55").unwrap(), 55);
    assert_eq!(numf_parser_str::<u32>("55").unwrap(), 55);
    assert_eq!(numf_parser_str::<u64>("55").unwrap(), 55);
    assert_eq!(numf_parser_str::<u128>("55").unwrap(), 55);
}

#[test]
fn test_parser_underscores() {
    assert_eq!(numf_parser_str::<u16>("5_500").unwrap(), 5_500);
    assert_eq!(
        numf_parser_str::<u64>("0xffffffff_00110011").unwrap(),
        0xffffffff_00110011
    );
}
