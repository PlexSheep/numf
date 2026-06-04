//! Format numbers
//!
//! This crate contains several utility functions for formatting numbers
//! into other systems, such as converting decimal numbers to hexadecimal, and back.
//!
//! See [format::Format] for supported formats.
//!
//! The [bintols] module contains functionality about manipulating binary data, which may be useful
//! to users of this library too.
//!
//! Note that this crate is primarily intended to be used as a executable.
//!
//! Highlights:
//! * [format::numf_parser]
//! * [format::numf_parser_str]
//! * [format::Format::format]
//! * [format::Format::format_str]

pub mod bintols;
pub mod format;

mod macros {

    // this macro is used to easier declare functions that have a shit ton of where conditions for
    // generics
    macro_rules!  UseManyTraitsForGenericUnsignedInt{
    (
        $(#[doc = $doc:expr])*
        $v:vis fn $fname:ident<$T:tt>($($args:tt : $args_type:ty)*)
        -> $res:path where $void:tt: SO_MANY_TRAITS_FROM_MACRO,
            $body:block

     ) => {

$v fn $fname<$T>($($args: $args_type)*) -> $res
where
$T: std::str::FromStr + std::convert::TryFrom<u128>,
<$T as std::str::FromStr>::Err: std::fmt::Display,
$T: num::Num,
<$T as num::Num>::FromStrRadixErr: std::fmt::Display,
<$T as std::str::FromStr>::Err: std::fmt::Debug,
u128: std::convert::From<$T>,
<$T as std::str::FromStr>::Err: std::error::Error,
<$T as std::convert::TryFrom<u128>>::Error: std::error::Error,
<$T as std::convert::TryFrom<u128>>::Error: std::marker::Send,
<$T as std::convert::TryFrom<u128>>::Error: std::marker::Sync,
<$T as std::convert::TryFrom<u128>>::Error: 'static,
{$body}
    };
}

    pub(crate) use UseManyTraitsForGenericUnsignedInt;
}
