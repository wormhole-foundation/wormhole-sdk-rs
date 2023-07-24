use alloy_primitives::{ruint::UintTryFrom, FixedBytes, U256, U8};

use crate::{Readable, Writeable};

use std::io;

const MAX_DECIMALS: u8 = 8;
const TEN: U256 = U256::from_limbs([0, 0, 0, 10]);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct EncodedAmount(pub U256);

impl EncodedAmount {
    pub const ZERO: Self = Self(U256::ZERO);

    pub fn from_raw(amount: U256, decimals: u8) -> Self {
        if decimals <= MAX_DECIMALS {
            Self(amount)
        } else {
            Self(amount.wrapping_div(pow10(decimals - MAX_DECIMALS)))
        }
    }

    pub fn to_raw(self, decimals: u8) -> U256 {
        if decimals <= MAX_DECIMALS {
            self.0
        } else {
            self.0.wrapping_mul(pow10(decimals - MAX_DECIMALS))
        }
    }

    pub fn checked_to_raw(self, decimals: u8) -> Option<U256> {
        if decimals <= MAX_DECIMALS {
            Some(self.0)
        } else {
            self.0.checked_mul(pow10(decimals - MAX_DECIMALS))
        }
    }
}

impl<T> From<T> for EncodedAmount
where
    U256: UintTryFrom<T>,
{
    fn from(amount: T) -> Self {
        Self(U256::from(amount))
    }
}

impl Readable for EncodedAmount {
    const SIZE: Option<usize> = Some(32);

    fn read<R>(reader: &mut R) -> io::Result<Self>
    where
        Self: Sized,
        R: io::Read,
    {
        Ok(Self(Readable::read(reader)?))
    }
}

impl Writeable for EncodedAmount {
    fn written_size(&self) -> usize {
        <Self as Readable>::SIZE.unwrap()
    }

    fn write<W>(&self, writer: &mut W) -> io::Result<()>
    where
        Self: Sized,
        W: io::Write,
    {
        self.0.write(writer)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EncodedString(pub FixedBytes<32>);

impl From<String> for EncodedString {
    fn from(value: String) -> Self {
        let mut bytes = FixedBytes::<32>::default();
        if value.len() > 32 {
            bytes.copy_from_slice(&value.as_bytes()[..32]);
        } else {
            bytes[..value.len()].copy_from_slice(value.as_bytes());
        }
        Self(bytes)
    }
}

impl From<EncodedString> for String {
    fn from(value: EncodedString) -> Self {
        use bstr::ByteSlice;
        value
            .0
            .chars()
            .filter(|&c| c != '\u{FFFD}')
            .collect::<Vec<char>>()
            .iter()
            .collect::<String>()
            .trim_end_matches(char::from(0))
            .to_string()
    }
}

impl Readable for EncodedString {
    const SIZE: Option<usize> = Some(32);

    fn read<R>(reader: &mut R) -> io::Result<Self>
    where
        Self: Sized,
        R: io::Read,
    {
        Ok(Self(Readable::read(reader)?))
    }
}

impl Writeable for EncodedString {
    fn written_size(&self) -> usize {
        <Self as Readable>::SIZE.unwrap()
    }

    fn write<W>(&self, writer: &mut W) -> io::Result<()>
    where
        Self: Sized,
        W: io::Write,
    {
        self.0.write(writer)
    }
}

fn pow10(power: u8) -> U256 {
    TEN.pow(U256::from(U8::from(power)))
}

#[cfg(test)]
mod test {
    use super::*;

    const TRIAL_AMOUNTS: [U256; 3] = [
        U256::ZERO,
        U256::from_limbs([0, 0, 0, 0xffffffffffffffff]),
        U256::MAX,
    ];

    #[test]
    fn normalized_amount_7_decimals() {
        const DECIMALS: u8 = 7;

        for &amount in TRIAL_AMOUNTS.iter() {
            let normalized = EncodedAmount::from_raw(amount, DECIMALS);
            assert_eq!(normalized.0, amount);
            let recovered = normalized.to_raw(DECIMALS);
            assert_eq!(recovered, amount);
        }
    }

    #[test]
    fn normalized_amount_8_decimals() {
        const DECIMALS: u8 = 8;

        for &amount in TRIAL_AMOUNTS.iter() {
            let normalized = EncodedAmount::from_raw(amount, DECIMALS);
            assert_eq!(normalized.0, amount);
            let recovered = normalized.to_raw(DECIMALS);
            assert_eq!(recovered, amount);
        }
    }

    #[test]
    fn normalized_amount_9_decimals() {
        const DECIMALS: u8 = 9;

        for &amount in TRIAL_AMOUNTS.iter() {
            let normalized = EncodedAmount::from_raw(amount, DECIMALS);
            assert_eq!(normalized.0, amount.wrapping_div(TEN));

            // Recovered amount will be truncated.
            let recovered = normalized.to_raw(DECIMALS);
            assert_eq!(recovered, amount.wrapping_div(TEN).wrapping_mul(TEN));
        }
    }

    #[test]
    fn normalized_amount_too_large() {
        let recovered = EncodedAmount::from(U256::MAX).checked_to_raw(9);
        assert_eq!(recovered, None);
    }

    #[test]
    fn unicode_truncation_empty() {
        let input = String::new();
        let converted = EncodedString::from(input);
        let recovered = String::from(converted);
        assert_eq!(recovered, String::new());
    }

    #[test]
    fn unicode_truncation_small() {
        let input = String::from("🔥");
        let converted = EncodedString::from(input);
        let recovered = String::from(converted);
        assert_eq!(recovered, String::from("🔥"));
    }

    #[test]
    fn unicode_truncation_exact() {
        let input = String::from("🔥🔥🔥🔥🔥🔥🔥🔥");
        let converted = EncodedString::from(input);
        let recovered = String::from(converted);
        assert_eq!(recovered, String::from("🔥🔥🔥🔥🔥🔥🔥🔥"));
    }

    #[test]
    fn unicode_truncation_large() {
        let input = String::from("🔥🔥🔥🔥🔥🔥🔥🔥🔥");
        let converted = EncodedString::from(input);
        let recovered = String::from(converted);
        assert_eq!(recovered, String::from("🔥🔥🔥🔥🔥🔥🔥🔥"));
    }

    #[test]
    fn unicode_truncation_partial_overflow() {
        let input = String::from("0000000000000000000000000000000🔥");
        let converted = EncodedString::from(input);
        let recovered = String::from(converted);
        assert_eq!(recovered, String::from("0000000000000000000000000000000"));
    }
}
