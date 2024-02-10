#[cfg(feature = "experimental")]
mod encoded_vaa;
#[cfg(feature = "experimental")]
pub use encoded_vaa::*;

mod posted_vaa_v1;
pub use posted_vaa_v1::*;

use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{account_info::AccountInfo, program_error::ProgramError};
use wormhole_raw_vaas::Payload;

#[cfg(feature = "experimental")]
use super::VaaVersion;

pub const VAA_VERSION: u8 = 1;

#[derive(Debug)]
#[non_exhaustive]
pub enum VaaAccount<'a> {
    PostedVaaV1(PostedVaaV1<'a>),
    #[cfg(feature = "experimental")]
    EncodedVaa(EncodedVaa<'a>),
}

#[derive(Debug, BorshDeserialize, BorshSerialize, Copy, Clone, PartialEq, Eq, Default)]
pub struct EmitterInfo {
    pub chain: u16,
    pub address: [u8; 32],
    pub sequence: u64,
}

impl<'a> VaaAccount<'a> {
    pub fn version(&'a self) -> u8 {
        match self {
            Self::PostedVaaV1(_) => VAA_VERSION,
            #[cfg(feature = "experimental")]
            Self::EncodedVaa(inner) => inner.version(),
        }
    }

    pub fn emitter_info(&self) -> EmitterInfo {
        match self {
            Self::PostedVaaV1(inner) => EmitterInfo {
                chain: inner.emitter_chain(),
                address: inner.emitter_address(),
                sequence: inner.sequence(),
            },
            #[cfg(feature = "experimental")]
            Self::EncodedVaa(inner) => match inner.as_vaa() {
                VaaVersion::V1(vaa) => EmitterInfo {
                    chain: vaa.body().emitter_chain(),
                    address: vaa.body().emitter_address(),
                    sequence: vaa.body().sequence(),
                },
            },
        }
    }

    pub fn emitter_chain(&self) -> u16 {
        match self {
            Self::PostedVaaV1(inner) => inner.emitter_chain(),
            #[cfg(feature = "experimental")]
            Self::EncodedVaa(inner) => match inner.as_vaa() {
                VaaVersion::V1(vaa) => vaa.body().emitter_chain(),
            },
        }
    }

    pub fn emitter_address(&self) -> [u8; 32] {
        match self {
            Self::PostedVaaV1(inner) => inner.emitter_address(),
            #[cfg(feature = "experimental")]
            Self::EncodedVaa(inner) => match inner.as_vaa() {
                VaaVersion::V1(vaa) => vaa.body().emitter_address(),
            },
        }
    }

    pub fn sequence(&self) -> u64 {
        match self {
            Self::PostedVaaV1(inner) => inner.sequence(),
            #[cfg(feature = "experimental")]
            Self::EncodedVaa(inner) => match inner.as_vaa() {
                VaaVersion::V1(vaa) => vaa.body().sequence(),
            },
        }
    }

    pub fn consistency_level(&self) -> u8 {
        match self {
            Self::PostedVaaV1(inner) => inner.consistency_level(),
            #[cfg(feature = "experimental")]
            Self::EncodedVaa(inner) => match inner.as_vaa() {
                VaaVersion::V1(vaa) => vaa.body().consistency_level(),
            },
        }
    }

    pub fn timestamp(&self) -> u32 {
        match self {
            Self::PostedVaaV1(inner) => inner.timestamp(),
            #[cfg(feature = "experimental")]
            Self::EncodedVaa(inner) => match inner.as_vaa() {
                VaaVersion::V1(vaa) => vaa.body().timestamp(),
            },
        }
    }

    pub fn nonce(&self) -> u32 {
        match self {
            Self::PostedVaaV1(inner) => inner.nonce(),
            #[cfg(feature = "experimental")]
            Self::EncodedVaa(inner) => match inner.as_vaa() {
                VaaVersion::V1(vaa) => vaa.body().nonce(),
            },
        }
    }

    pub fn payload(&'a self) -> Payload<'a> {
        match self {
            Self::PostedVaaV1(inner) => inner.payload(),
            #[cfg(feature = "experimental")]
            Self::EncodedVaa(inner) => match inner.as_vaa() {
                VaaVersion::V1(vaa) => vaa.body().payload(),
            },
        }
    }

    pub fn digest(&self) -> solana_program::keccak::Hash {
        match self {
            Self::PostedVaaV1(inner) => inner.digest(),
            #[cfg(feature = "experimental")]
            Self::EncodedVaa(inner) => inner.digest(),
        }
    }

    #[cfg(feature = "experimental")]
    pub fn encoded_vaa(&'a self) -> Option<&'a EncodedVaa<'a>> {
        match self {
            Self::EncodedVaa(inner) => Some(inner),
            _ => None,
        }
    }

    pub fn posted_vaa_v1(&'a self) -> Option<&'a PostedVaaV1<'a>> {
        match self {
            Self::PostedVaaV1(inner) => Some(inner),
            #[cfg(feature = "experimental")]
            _ => None,
        }
    }

    pub fn load(acc_info: &'a AccountInfo) -> Result<Self, ProgramError> {
        let data = acc_info.try_borrow_data()?;

        if data.len() <= 8 {
            Err(ProgramError::InvalidAccountData)
        } else {
            match <[u8; 8]>::try_from(&data[..8]).unwrap() {
                [118, 97, 97, 1, _, _, _, _] => Ok(Self::PostedVaaV1(PostedVaaV1::new(acc_info)?)),
                #[cfg(feature = "experimental")]
                EncodedVaa::DISCRIMINATOR => Ok(Self::EncodedVaa(EncodedVaa::new(acc_info)?)),
                _ => Err(ProgramError::InvalidAccountData),
            }
        }
    }
}
