#![cfg(any(feature = "on-chain", feature = "off-chain"))]

//! Provides support for EITHER ruint@1.9.0 OR alloy_primitives.

#[cfg(feature = "ruint")]
use ruint::Uint;

#[cfg(not(feature = "ruint"))]
use alloy_primitives::Uint;

use crate::payloads::token_bridge::{Transfer, TransferWithMessage};

type U256 = Uint<256, 4>;

const MAX_DECIMALS: u8 = 8;
const TEN: U256 = Uint::from_limbs([10, 0, 0, 0]);

pub struct EncodedAmount(pub U256);

impl From<[u8; 32]> for EncodedAmount {
    fn from(value: [u8; 32]) -> Self {
        Self(Uint::from_be_bytes(value))
    }
}

fn wrapping_pow10(power: u8) -> U256 {
    TEN.wrapping_pow(U256::from(Uint::<8, 1>::from(power)))
}

fn checked_pow10(power: u8) -> Option<U256> {
    TEN.checked_pow(U256::from(Uint::<8, 1>::from(power)))
}

impl EncodedAmount {
    pub const ZERO: Self = Self(Uint::ZERO);

    /// Create a new encoded amount by normalizing a raw amount sdjusted by an
    /// asset's decimals only if the decimals exceed the maximum allowed (8)
    /// for encoding.
    pub fn norm(amount: U256, decimals: u8) -> Self {
        if decimals <= MAX_DECIMALS {
            Self(amount)
        } else {
            Self(amount.wrapping_div(wrapping_pow10(decimals - MAX_DECIMALS)))
        }
    }

    /// Convert an encoded amount back to a raw amount by scaling it by its
    /// decimals if the decimals eceed the maximum allowed (8) from encoding.
    pub fn denorm(self, decimals: u8) -> U256 {
        if decimals <= MAX_DECIMALS {
            self.0
        } else {
            self.0.wrapping_mul(wrapping_pow10(decimals - MAX_DECIMALS))
        }
    }

    /// Convert an encoded amount back to a raw amount by scaling it by its
    /// decimals if the decimals eceed the maximum allowed (8) from encoding.
    /// This method will return `None` if the raw amount overflows 32 bytes.
    pub fn checked_denorm(self, decimals: u8) -> Option<U256> {
        if decimals <= MAX_DECIMALS {
            Some(self.0)
        } else {
            checked_pow10(decimals - MAX_DECIMALS).and_then(|scale| self.0.checked_mul(scale))
        }
    }
}

#[cfg(any(feature = "on-chain", feature = "off-chain"))]
impl Transfer<'_> {
    pub fn encoded_amount(&self) -> EncodedAmount {
        self.amount().into()
    }

    pub fn encoded_relayer_fee(&self) -> EncodedAmount {
        self.relayer_fee().into()
    }
}

#[cfg(any(feature = "on-chain", feature = "off-chain"))]
impl TransferWithMessage<'_> {
    pub fn encoded_amount(&self) -> EncodedAmount {
        self.amount().into()
    }
}
