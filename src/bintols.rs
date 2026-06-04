use anyhow::anyhow;
use log::{debug, error, trace};

/// Join a [Vec] of [u8]s into an unsigned integer
///
/// Say you have the array `[0b00000110, 0b10110101]` and want to use it as a [u32].
/// This function sets it together to a integer type of your choosing:
/// 1717 (binary: `00000000 00000000 00000110 10110101`).
///
/// If the array is not long enough, the number will be padded with null bytes.
///
/// # Examples
///
/// ```
/// # use numf::bintols::join::*;
///
/// let x: [u8; 2] = [0b00000110, 0b10110101];
///
/// assert_eq!(array_to_unsigned::<u32>(&x).unwrap(), 1717);
/// ```
pub fn array_to_unsigned<T>(parts: &[u8]) -> anyhow::Result<T>
where
    u128: std::convert::From<T>,
    T: std::str::FromStr,
    T: std::convert::TryFrom<u128>,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
    <T as std::str::FromStr>::Err: std::error::Error,
    <T as std::convert::TryFrom<u128>>::Error: std::error::Error,
    <T as std::convert::TryFrom<u128>>::Error: std::marker::Send,
    <T as std::convert::TryFrom<u128>>::Error: std::marker::Sync,
    <T as std::convert::TryFrom<u128>>::Error: 'static,
{
    trace!("amount of parts: {}", parts.len());
    if parts.len() > (u128::BITS / 8) as usize {
        return Err(anyhow!(
            "the list is too long to fit into the specified integer type: {}",
            std::any::type_name::<T>()
        ));
    }
    let mut ri: u128 = 0;
    for (i, e) in parts.iter().rev().enumerate() {
        ri += (*e as u128) * 256u128.pow(i as u32);
    }
    T::try_from(ri).map_err(anyhow::Error::from)
}

/// Split unsigned integers into a [Vec] of [u8]s
///
/// Say you have the [u32] 1717 (binary: `00000000 00000000 00000110 10110101 `). This number would
/// be splitted to `vec![0b00000110, 0b10110101]`.
///
/// The 0 bytes of the numbers will be discarded (unless the number is 0, then the Vec contains a
/// single Null byte.) and the remaining parts of the numbers are inserted into a Vec as [u8].
///
/// # Examples
///
/// ```
/// # use numf::bintols::split::*;
///
/// let x: u32 = 1717;
///
/// assert_eq!(unsigned_to_vec(x), vec![0b00000110, 0b10110101]);
/// ```
pub fn unsigned_to_vec<T>(num: T) -> Vec<u8>
where
    u128: std::convert::From<T>,
{
    let mut num: u128 = num.into();
    if num == 0 {
        return vec![0];
    }
    let mut buf: Vec<u8> = Vec::new();
    while num > 0 {
        buf.push(num as u8);
        num >>= 8;
    }
    buf.reverse();
    buf
}

pub fn highest_set_bit_pos(mut num: u128) -> u8 {
    let mut b = 0;
    while num > 1 {
        num >>= 1;
        b += 1;
    }
    b
}

pub fn from_twos_complement_to_asbolute(mut num: u128, bl: u8) -> anyhow::Result<u128> {
    if num >> (bl - 1) > 1 {
        let err = anyhow!("input number {num} is too large to fit into an {bl} bit signed integer");
        return Err(err);
    }

    debug!("num (unsigned): {num:#x}");
    trace!("bl: {bl}");
    let mask = (1 << (bl)) - 1;
    trace!("mask: {mask:#x}");
    num &= mask;
    trace!("num (trunc): {num:#x}");
    num ^= mask;
    trace!("num (flip): {num:#x}");
    num += 1;
    debug!("num (signed): {num:#x} = {num:#b} = {num}");
    let limit = 2u128.pow(bl as u32 - 1);
    trace!("limit: {limit}");
    assert!(num <= limit);
    Ok(num)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_bintols_highest_set_bit_pos() {
        assert_eq!(highest_set_bit_pos(0b1_0000), 4);
        assert_eq!(highest_set_bit_pos(0b10_0000), 5);
        assert_eq!(highest_set_bit_pos(0b11_1111), 5);
        assert_eq!(highest_set_bit_pos(0b1_1111), 4);
    }

    #[test]
    fn test_to_negative_twos_complement() {
        assert_eq!(from_twos_complement_to_asbolute(0b1111_1111, 8).unwrap(), 1);
        assert_eq!(from_twos_complement_to_asbolute(0b1111_1110, 8).unwrap(), 2);
        assert_eq!(from_twos_complement_to_asbolute(0b1_1111, 5).unwrap(), 1);
        assert_eq!(from_twos_complement_to_asbolute(0b1_0000, 5).unwrap(), 16);
    }
}
