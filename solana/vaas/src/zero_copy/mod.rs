#![allow(clippy::result_large_err)]

#[cfg(feature = "encoded-vaa")]
mod encoded_vaa;
#[cfg(feature = "encoded-vaa")]
pub use encoded_vaa::*;

mod posted_vaa_v1;
pub use posted_vaa_v1::*;

use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::account_info::AccountInfo;
use wormhole_raw_vaas::Payload;

#[cfg(feature = "anchor")]
type FeatureResult<T> = anchor_lang::Result<T>;
#[cfg(not(feature = "anchor"))]
type FeatureResult<T> = Result<T, solana_program::program_error::ProgramError>;

#[cfg(feature = "encoded-vaa")]
use super::VaaVersion;

pub const VAA_VERSION: u8 = 1;

#[derive(Debug)]
#[non_exhaustive]
pub enum VaaAccount<'a> {
    PostedVaaV1(PostedVaaV1<'a>),
    #[cfg(feature = "encoded-vaa")]
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
            #[cfg(feature = "encoded-vaa")]
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
            #[cfg(feature = "encoded-vaa")]
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
            #[cfg(feature = "encoded-vaa")]
            Self::EncodedVaa(inner) => match inner.as_vaa() {
                VaaVersion::V1(vaa) => vaa.body().emitter_chain(),
            },
        }
    }

    pub fn emitter_address(&self) -> [u8; 32] {
        match self {
            Self::PostedVaaV1(inner) => inner.emitter_address(),
            #[cfg(feature = "encoded-vaa")]
            Self::EncodedVaa(inner) => match inner.as_vaa() {
                VaaVersion::V1(vaa) => vaa.body().emitter_address(),
            },
        }
    }

    pub fn sequence(&self) -> u64 {
        match self {
            Self::PostedVaaV1(inner) => inner.sequence(),
            #[cfg(feature = "encoded-vaa")]
            Self::EncodedVaa(inner) => match inner.as_vaa() {
                VaaVersion::V1(vaa) => vaa.body().sequence(),
            },
        }
    }

    pub fn consistency_level(&self) -> u8 {
        match self {
            Self::PostedVaaV1(inner) => inner.consistency_level(),
            #[cfg(feature = "encoded-vaa")]
            Self::EncodedVaa(inner) => match inner.as_vaa() {
                VaaVersion::V1(vaa) => vaa.body().consistency_level(),
            },
        }
    }

    pub fn timestamp(&self) -> u32 {
        match self {
            Self::PostedVaaV1(inner) => inner.timestamp(),
            #[cfg(feature = "encoded-vaa")]
            Self::EncodedVaa(inner) => match inner.as_vaa() {
                VaaVersion::V1(vaa) => vaa.body().timestamp(),
            },
        }
    }

    pub fn nonce(&self) -> u32 {
        match self {
            Self::PostedVaaV1(inner) => inner.nonce(),
            #[cfg(feature = "encoded-vaa")]
            Self::EncodedVaa(inner) => match inner.as_vaa() {
                VaaVersion::V1(vaa) => vaa.body().nonce(),
            },
        }
    }

    pub fn payload(&'a self) -> Payload<'a> {
        match self {
            Self::PostedVaaV1(inner) => inner.payload(),
            #[cfg(feature = "encoded-vaa")]
            Self::EncodedVaa(inner) => match inner.as_vaa() {
                VaaVersion::V1(vaa) => vaa.body().payload(),
            },
        }
    }

    pub fn digest(&self) -> solana_program::keccak::Hash {
        match self {
            Self::PostedVaaV1(inner) => inner.digest(),
            #[cfg(feature = "encoded-vaa")]
            Self::EncodedVaa(inner) => inner.digest(),
        }
    }

    pub fn guardian_set_index(&self) -> u32 {
        match self {
            VaaAccount::PostedVaaV1(inner) => inner.guardian_set_index(),
            #[cfg(feature = "encoded-vaa")]
            VaaAccount::EncodedVaa(inner) => match inner.as_vaa() {
                VaaVersion::V1(vaa) => vaa.guardian_set_index(),
            },
        }
    }

    #[cfg(feature = "encoded-vaa")]
    pub fn encoded_vaa(&'a self) -> Option<&'a EncodedVaa<'a>> {
        match self {
            Self::EncodedVaa(inner) => Some(inner),
            _ => None,
        }
    }

    pub fn posted_vaa_v1(&'a self) -> Option<&'a PostedVaaV1<'a>> {
        match self {
            Self::PostedVaaV1(inner) => Some(inner),
            #[cfg(feature = "encoded-vaa")]
            _ => None,
        }
    }

    pub fn load(acc_info: &'a AccountInfo) -> FeatureResult<VaaAccount<'a>> {
        // First check owner. TODO: Change this to InvalidAccountOwner when we bump to solana 1.17
        if *acc_info.owner != wormhole_solana_consts::CORE_BRIDGE_PROGRAM_ID {
            #[cfg(feature = "anchor")]
            return Err(anchor_lang::error::ErrorCode::ConstraintOwner.into());
            #[cfg(not(feature = "anchor"))]
            return Err(solana_program::program_error::ProgramError::IllegalOwner);
        } else {
            let data = acc_info.try_borrow_data()?;
            if data.len() <= 8 {
                #[cfg(feature = "anchor")]
                return Err(anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into());
                #[cfg(not(feature = "anchor"))]
                return Err(solana_program::program_error::ProgramError::InvalidAccountData);
            } else {
                match <[u8; 8]>::try_from(&data[..8]).unwrap() {
                    [118, 97, 97, 1, _, _, _, _] => {
                        Ok(Self::PostedVaaV1(PostedVaaV1::new(acc_info)?))
                    }
                    #[cfg(feature = "encoded-vaa")]
                    EncodedVaa::DISCRIMINATOR => Ok(Self::EncodedVaa(EncodedVaa::new(acc_info)?)),
                    _ => {
                        #[cfg(feature = "anchor")]
                        return Err(anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into());
                        #[cfg(not(feature = "anchor"))]
                        return Err(
                            solana_program::program_error::ProgramError::InvalidAccountData,
                        );
                    }
                }
            }
        }
    }

    pub fn load_unchecked(acc_info: &'a AccountInfo) -> Self {
        let data = acc_info.data.borrow();

        match <[u8; 8]>::try_from(&data[..8]).unwrap() {
            [118, 97, 97, 1, _, _, _, _] => Self::PostedVaaV1(PostedVaaV1::new_unchecked(acc_info)),
            #[cfg(feature = "encoded-vaa")]
            EncodedVaa::DISCRIMINATOR => Self::EncodedVaa(EncodedVaa::new_unchecked(acc_info)),
            _ => panic!("Invalid account data"),
        }
    }
}
