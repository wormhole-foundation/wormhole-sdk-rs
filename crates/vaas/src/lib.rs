mod read_write;
pub use read_write::{Readable, Writeable};

pub mod utils;
pub use utils::{keccak256, quorum};

pub mod payloads;
pub use payloads::{PayloadKind, TypePrefixedPayload};

mod protocol;
pub use protocol::{
    encoded_types::EncodedAmount,
    signature::GuardianSetSig,
    vaa::{Vaa, VaaBody, VaaHeader},
};

mod support;

#[cfg(not(feature = "anchor"))]
mod aliases {
    pub use alloy_primitives::{ruint::UintTryFrom, Address, FixedBytes, Uint, U256, U64, U8};
}

#[cfg(feature = "anchor")]
mod aliases {
    pub use ruint::{Uint, UintTryFrom};
    pub type U256 = Uint<256, 4>;
    pub type U64 = Uint<64, 1>;
    pub type U8 = Uint<8, 1>;

    use derive_more::{Deref, DerefMut, From, Index, IndexMut, IntoIterator};

    #[derive(
        Debug,
        Clone,
        Copy,
        PartialEq,
        Eq,
        PartialOrd,
        Ord,
        Hash,
        Deref,
        DerefMut,
        From,
        Index,
        IndexMut,
        IntoIterator,
    )]
    pub struct FixedBytes<const N: usize>(#[into_iterator(owned, ref, ref_mut)] pub [u8; N]);

    impl<const N: usize> FixedBytes<N> {
        /// Array of Zero bytes.
        pub const ZERO: Self = Self([0u8; N]);
    }

    impl<const N: usize> AsRef<[u8]> for FixedBytes<N> {
        #[inline]
        fn as_ref(&self) -> &[u8] {
            &self.0
        }
    }

    pub type Address = FixedBytes<20>;
}

pub use aliases::*;
