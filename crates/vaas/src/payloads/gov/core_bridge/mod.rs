mod contract_upgrade;
pub use contract_upgrade::ContractUpgrade;

mod guardian_set_update;
pub use guardian_set_update::GuardianSetUpdate;

mod recover_chain_id;
pub use recover_chain_id::RecoverChainId;

mod set_message_fee;
pub use set_message_fee::SetMessageFee;

mod transfer_fees;
pub use transfer_fees::TransferFees;

use crate::{Readable, Writeable};
use alloy_primitives::FixedBytes;
use hex_literal::hex;

use super::{GovernanceHeader, GovernanceMessage};

/// A.K.A. "Core".
pub const GOVERNANCE_MODULE: FixedBytes<32> = FixedBytes(hex!(
    "00000000000000000000000000000000000000000000000000000000436f7265"
));

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Decree {
    ContractUpgrade(ContractUpgrade),
    GuardianSetUpdate(GuardianSetUpdate),
    SetMessageFee(SetMessageFee),
    TransferFees(TransferFees),
    RecoverChainId(RecoverChainId),
}

impl Decree {
    const CONTRACT_UPGRADE: u8 = 1;
    const GUARDIAN_SET_UPDATE: u8 = 2;
    const SET_MESSAGE_FEE: u8 = 3;
    const TRANSFER_FEES: u8 = 4;
    const RECOVER_CHAIN_ID: u8 = 5;
}

impl Writeable for Decree {
    fn write<W>(&self, writer: &mut W) -> std::io::Result<()>
    where
        W: std::io::Write,
    {
        match self {
            Decree::ContractUpgrade(inner) => {
                Decree::CONTRACT_UPGRADE.write(writer)?;
                inner.write(writer)
            }
            Decree::GuardianSetUpdate(inner) => {
                Decree::GUARDIAN_SET_UPDATE.write(writer)?;
                inner.write(writer)
            }
            Decree::SetMessageFee(inner) => {
                Decree::SET_MESSAGE_FEE.write(writer)?;
                inner.write(writer)
            }
            Decree::TransferFees(inner) => {
                Decree::TRANSFER_FEES.write(writer)?;
                inner.write(writer)
            }
            Decree::RecoverChainId(inner) => {
                Decree::RECOVER_CHAIN_ID.write(writer)?;
                inner.write(writer)
            }
        }
    }

    fn written_size(&self) -> usize {
        1 + match self {
            Decree::ContractUpgrade(inner) => inner.written_size(),
            Decree::GuardianSetUpdate(inner) => inner.written_size(),
            Decree::SetMessageFee(inner) => inner.written_size(),
            Decree::TransferFees(inner) => inner.written_size(),
            Decree::RecoverChainId(inner) => inner.written_size(),
        }
    }
}

impl Readable for GovernanceMessage<Decree> {
    const SIZE: Option<usize> = None;

    fn read<R>(reader: &mut R) -> std::io::Result<Self>
    where
        R: std::io::Read,
    {
        let header = GovernanceHeader::read(reader)?;
        if header.module != GOVERNANCE_MODULE {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Invalid module",
            ));
        }

        let decree = match u8::read(reader)? {
            Decree::CONTRACT_UPGRADE => Decree::ContractUpgrade(Readable::read(reader)?),
            Decree::GUARDIAN_SET_UPDATE => Decree::GuardianSetUpdate(Readable::read(reader)?),
            Decree::SET_MESSAGE_FEE => Decree::SetMessageFee(Readable::read(reader)?),
            Decree::TRANSFER_FEES => Decree::TransferFees(Readable::read(reader)?),
            Decree::RECOVER_CHAIN_ID => Decree::RecoverChainId(Readable::read(reader)?),
            _ => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "Invalid Core Bridge decree",
                ))
            }
        };

        Ok(Self { header, decree })
    }
}
