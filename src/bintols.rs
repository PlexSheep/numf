use anyhow::anyhow;
use log::{debug, error, trace};

use crate::macros::UseManyTraitsForGenericUnsignedInt;

UseManyTraitsForGenericUnsignedInt!(
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
        T: SO_MANY_TRAITS_FROM_MACRO,
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
);

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

UseManyTraitsForGenericUnsignedInt!(
    // this can probably be implemented less complicated, but the limiting thing is that we support
    // an arbitrary bit size for the signed integer. u128 is just the data container. Not all data in
    // u128 fits into a signed `bl` bit integer, so it needs to return a result.
    pub fn signed_to_abs<T>(mut num: T, bl: u8) -> anyhow::Result<T>
    where
        T: SO_MANY_TRAITS_FROM_MACRO,
    {
        let mut num: u128 = num.into();
        if num >= (1 << (bl)) {
            let err =
                anyhow!("input number {num} is too large to fit into a {bl} bit signed integer");
            return Err(err);
        }

        if num >> (bl - 1) == 0 {
            // sign bit is not set
            T::try_from(num).map_err(anyhow::Error::from)
        } else {
            let mask = (1 << (bl)) - 1;
            debug!("num (unsigned): {num:#x}");
            debug!("mask: {mask:#x}");
            // sign bit is set
            num ^= mask;
            trace!("num (flip): {num:#x}");
            num += 1;
            debug!("num (signed): {num:#x} = {num:#b} = {num}");
            let limit = 2u128.pow(bl as u32 - 1);
            trace!("limit: {limit}");
            assert!(num <= limit);
            T::try_from(num).map_err(anyhow::Error::from)
        }
    }
);

UseManyTraitsForGenericUnsignedInt!(
    pub fn absolute_to_negative_signed_abs<T>(num: T, bl: u8) -> anyhow::Result<T>
    where
        T: SO_MANY_TRAITS_FROM_MACRO,
    {
        let mut num: u128 = num.into();
        let mask = (1 << (bl)) - 1;

        if num >= (1 << (bl - 1)) {
            let err = anyhow!("input number {num} is too large to fit into the negative area of a {bl} bit signed integer");
            return Err(err);
        }

        dbg!(format!("{num:05b}"));
        num ^= mask;
        dbg!(format!("{num:05b}"));
        num += 1;
        dbg!(format!("{num:05b}"));
        dbg!(format!("{num:02x}"));
        T::try_from(num).map_err(anyhow::Error::from)
    }
);

#[cfg(test)]
mod test {
    use assert_hex::assert_eq_hex;

    use super::*;

    #[test]
    fn test_signed_to_absolute() {
        signed_to_abs(0b10_0000u32, 5).unwrap_err();

        assert_eq_hex!(signed_to_abs(0b1111_1111u32, 8).unwrap(), 1);
        assert_eq_hex!(signed_to_abs(0b1111_1110u32, 8).unwrap(), 2);
        assert_eq_hex!(signed_to_abs(0b1_1111u16, 5).unwrap(), 1);
        assert_eq_hex!(signed_to_abs(0b1_0000u64, 5).unwrap(), 16);
        assert_eq_hex!(signed_to_abs(0b0_0000u8, 5).unwrap(), 0);
        assert_eq_hex!(signed_to_abs(0b0_1111u128, 5).unwrap(), 15);

        assert_eq_hex!(signed_to_abs(-18i32 as u32 as u128, 32).unwrap(), 18);
        assert_eq_hex!(
            signed_to_abs(-181001i32 as u32 as u128, 32).unwrap(),
            181001
        );
    }
    #[test]
    fn test_absolute_to_negative_signed() {
        absolute_to_negative_signed_abs(0b1_1111u128, 5).unwrap_err();
        absolute_to_negative_signed_abs(0b1_0000u64, 5).unwrap_err();

        assert_eq_hex!(absolute_to_negative_signed_abs(1u8, 5).unwrap(), 0b1_1111);
        assert_eq_hex!(
            absolute_to_negative_signed_abs(0b0_0110u16, 5).unwrap(),
            0b1_1010
        );
        assert_eq_hex!(
            absolute_to_negative_signed_abs(0b0_0111u32, 5).unwrap(),
            0b1_1001
        );
        assert_eq_hex!(
            absolute_to_negative_signed_abs(18, 32).unwrap(),
            -18i32 as u32 as u128
        );
        assert_eq_hex!(
            absolute_to_negative_signed_abs(13277, 32).unwrap(),
            -13277i32 as u32 as u128
        );
    }
}
