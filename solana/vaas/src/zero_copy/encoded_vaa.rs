use std::cell::Ref;

use solana_program::{
    account_info::AccountInfo, keccak, program_error::ProgramError, pubkey::Pubkey,
};
use wormhole_raw_vaas::Vaa;

use crate::VaaVersion;

#[derive(Debug)]
/// Account used to warehouse VAA buffer.
pub struct EncodedVaa<'a>(Ref<'a, &'a mut [u8]>);

impl<'a> EncodedVaa<'a> {
    pub const DISCRIMINATOR: [u8; 8] = [226, 101, 163, 4, 133, 160, 84, 245];
    pub const VAA_START: usize = 46;

    pub const PROCESSING_STATUS_UNSET: u8 = 0;
    pub const PROCESSING_STATUS_WRITING: u8 = 1;
    pub const PROCESSING_STATUS_VERIFIED: u8 = 2;

    /// Processing status. **This encoded VAA is only considered usable when this status is set
    /// to [Verified](state::ProcessingStatus::Verified).**
    pub fn status(&self) -> u8 {
        self.0[8]
    }

    /// The authority that has write privilege to this account.
    pub fn write_authority(&self) -> Pubkey {
        Pubkey::try_from(&self.0[9..41]).unwrap()
    }

    /// VAA version. Only when the VAA is verified is this version set to something that is not
    /// [Unset](VaaVersion::Unset).
    pub fn version(&self) -> u8 {
        self.0[41]
    }

    pub fn vaa_size(&self) -> usize {
        u32::from_le_bytes(self.0[42..Self::VAA_START].try_into().unwrap())
            .try_into()
            .unwrap()
    }

    pub fn buf(&self) -> &[u8] {
        &self.0[Self::VAA_START..]
    }

    pub fn as_vaa(&self) -> VaaVersion {
        match self.version() {
            super::VAA_VERSION => VaaVersion::V1(Vaa::parse(&self.0[Self::VAA_START..]).unwrap()),
            _ => unreachable!(),
        }
    }

    /// Recompute the message hash.
    pub fn message_hash(&self) -> keccak::Hash {
        match self.as_vaa() {
            VaaVersion::V1(vaa) => keccak::hash(vaa.body().as_ref()),
        }
    }

    /// Compute digest (hash of [message_hash](Self::message_hash)).
    pub fn digest(&self) -> keccak::Hash {
        keccak::hash(self.message_hash().as_ref())
    }

    pub(super) fn new(acc_info: &'a AccountInfo) -> Result<Self, ProgramError> {
        let parsed = Self(acc_info.try_borrow_data()?);

        // We only allow verified VAAs to be read.
        if parsed.version() != 1 || parsed.status() != Self::PROCESSING_STATUS_VERIFIED {
            Err(ProgramError::InvalidAccountData)
        } else {
            Ok(parsed)
        }
    }
}
