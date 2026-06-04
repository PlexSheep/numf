// this macro is used to easier declare functions that have a shit ton of where conditions for
// generics
macro_rules!  UseManyTraitsForGenericUnsignedInt{
(
    $(#[doc = $doc:expr])*
    $v:vis fn $fname:ident<$T:tt>($($arg:tt)*)
    -> $res:path where $void:tt: SO_MANY_TRAITS_FROM_MACRO,
        $body:block

 ) => {

$v fn $fname<$T>($($arg)*) -> $res
where
$T: std::fmt::Display,
$T: std::fmt::Binary,
$T: std::fmt::LowerHex,
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
