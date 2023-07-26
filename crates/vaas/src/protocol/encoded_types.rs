use crate::aliases::{UintTryFrom, U256, U8};

use crate::{Readable, Writeable};

use std::io;

const MAX_DECIMALS: u8 = 8;
const TEN: U256 = U256::from_limbs([10, 0, 0, 0]);

/// This amount reflects the token transfer amount encoded in a Token Bridge
/// message. These amounts are capped at 8 decimals. This means that any amount
/// of a coin whose metadata defines its decimals as some value greater than 8,
/// the encoded amount will be normalized to eight decimals (which will lead to
/// some residual amount after the transfer). For inbound transfers, this amount
/// will be denormalized (scaled by the same decimal difference).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct EncodedAmount(pub U256);

impl EncodedAmount {
    pub const ZERO: Self = Self(U256::ZERO);

    /// Create a new encoded amount by normalizing a raw amount sdjusted by an
    /// asset's decimals only if the decimals exceed the maximum allowed (8)
    /// for encoding.
    pub fn norm(amount: U256, decimals: u8) -> Self {
        if decimals <= MAX_DECIMALS {
            Self(amount)
        } else {
            Self(amount.wrapping_div(pow10(decimals - MAX_DECIMALS)))
        }
    }

    /// Convert an encoded amount back to a raw amount by scaling it by its
    /// decimals if the decimals eceed the maximum allowed (8) from encoding.
    pub fn denorm(self, decimals: u8) -> U256 {
        if decimals <= MAX_DECIMALS {
            self.0
        } else {
            self.0.wrapping_mul(pow10(decimals - MAX_DECIMALS))
        }
    }

    /// Convert an encoded amount back to a raw amount by scaling it by its
    /// decimals if the decimals eceed the maximum allowed (8) from encoding.
    /// This method will return `None` if the raw amount overflows 32 bytes.
    pub fn checked_denorm(self, decimals: u8) -> Option<U256> {
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

fn pow10(power: u8) -> U256 {
    TEN.pow(U256::from(U8::from(power)))
}

#[cfg(test)]
mod test {
    use super::*;

    const TRIAL_AMOUNTS: [U256; 3] = [
        U256::ZERO,
        U256::from_limbs([0xffffffffffffffff, 0, 0, 0]),
        U256::MAX,
    ];

    #[test]
    fn normalized_amount_7_decimals() {
        const DECIMALS: u8 = 7;

        for &amount in TRIAL_AMOUNTS.iter() {
            let normalized = EncodedAmount::norm(amount, DECIMALS);
            assert_eq!(normalized.0, amount);
            let recovered = normalized.denorm(DECIMALS);
            assert_eq!(recovered, amount);
        }
    }

    #[test]
    fn normalized_amount_8_decimals() {
        const DECIMALS: u8 = 8;

        for &amount in TRIAL_AMOUNTS.iter() {
            let normalized = EncodedAmount::norm(amount, DECIMALS);
            assert_eq!(normalized.0, amount);
            let recovered = normalized.denorm(DECIMALS);
            assert_eq!(recovered, amount);
        }
    }

    #[test]
    fn normalized_amount_9_decimals() {
        const DECIMALS: u8 = 9;

        for &amount in TRIAL_AMOUNTS.iter() {
            let normalized = EncodedAmount::norm(amount, DECIMALS);
            assert_eq!(normalized.0, amount.wrapping_div(TEN));

            // Recovered amount will be truncated.
            let recovered = normalized.denorm(DECIMALS);
            assert_eq!(recovered, amount.wrapping_div(TEN).wrapping_mul(TEN));
        }
    }

    #[test]
    fn normalized_amount_too_large() {
        let recovered = EncodedAmount::from(U256::MAX).checked_denorm(9);
        assert_eq!(recovered, None);
    }
}
