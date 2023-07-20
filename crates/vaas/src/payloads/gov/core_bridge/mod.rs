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

impl Writeable for Decree {
    fn write<W>(&self, writer: &mut W) -> std::io::Result<()>
    where
        W: std::io::Write,
    {
        match self {
            Decree::ContractUpgrade(inner) => {
                u8::write(&1, writer)?;
                inner.write(writer)
            }
            Decree::GuardianSetUpdate(inner) => {
                u8::write(&2, writer)?;
                inner.write(writer)
            }
            Decree::SetMessageFee(inner) => {
                u8::write(&3, writer)?;
                inner.write(writer)
            }
            Decree::TransferFees(inner) => {
                u8::write(&4, writer)?;
                inner.write(writer)
            }
            Decree::RecoverChainId(inner) => {
                u8::write(&5, writer)?;
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
            1 => Decree::ContractUpgrade(Readable::read(reader)?),
            2 => Decree::GuardianSetUpdate(Readable::read(reader)?),
            3 => Decree::SetMessageFee(Readable::read(reader)?),
            4 => Decree::TransferFees(Readable::read(reader)?),
            5 => Decree::RecoverChainId(Readable::read(reader)?),
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
