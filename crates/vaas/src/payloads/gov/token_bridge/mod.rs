mod register_chain;
pub use register_chain::RegisterChain;

pub use super::core_bridge::{ContractUpgrade, RecoverChainId};

use crate::{Readable, Writeable};
use alloy_primitives::FixedBytes;
use hex_literal::hex;

use super::{GovernanceHeader, GovernanceMessage};

/// A.K.A. "TokenBridge".
pub const GOVERNANCE_MODULE: FixedBytes<32> = FixedBytes(hex!(
    "000000000000000000000000000000000000000000546f6b656e427269646765"
));

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Decree {
    RegisterChain(RegisterChain),
    ContractUpgrade(ContractUpgrade),
    RecoverChainId(RecoverChainId),
}

impl Decree {
    const REGISTER_CHAIN: u8 = 1;
    const CONTRACT_UPGRADE: u8 = 2;
    const RECOVER_CHAIN_ID: u8 = 3;
}

impl Writeable for Decree {
    fn write<W>(&self, writer: &mut W) -> std::io::Result<()>
    where
        W: std::io::Write,
    {
        match self {
            Decree::RegisterChain(inner) => {
                Decree::REGISTER_CHAIN.write(writer)?;
                inner.write(writer)
            }
            Decree::ContractUpgrade(inner) => {
                Decree::CONTRACT_UPGRADE.write(writer)?;
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
            Decree::RegisterChain(inner) => inner.written_size(),
            Decree::ContractUpgrade(inner) => inner.written_size(),
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
            Decree::REGISTER_CHAIN => Decree::RegisterChain(Readable::read(reader)?),
            Decree::CONTRACT_UPGRADE => Decree::ContractUpgrade(Readable::read(reader)?),
            Decree::RECOVER_CHAIN_ID => Decree::RecoverChainId(Readable::read(reader)?),
            _ => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "Invalid Token Bridge decree",
                ))
            }
        };

        Ok(Self { header, decree })
    }
}
