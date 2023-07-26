mod register_chain;
pub use register_chain::RegisterChain;

mod recover_chain_id;
pub use recover_chain_id::RecoverChainId;

mod contract_upgrade;
pub use contract_upgrade::ContractUpgrade;

use crate::aliases::FixedBytes;
use crate::{Readable, TypePrefixedPayload, Writeable};
use hex_literal::hex;

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

impl TypePrefixedPayload for Decree {
    const TYPE: Option<u8> = None;
}

impl Writeable for Decree {
    fn write<W>(&self, writer: &mut W) -> std::io::Result<()>
    where
        W: std::io::Write,
    {
        match self {
            Decree::RegisterChain(inner) => inner.write_payload(writer),
            Decree::ContractUpgrade(inner) => inner.write_payload(writer),
            Decree::RecoverChainId(inner) => inner.write_payload(writer),
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

impl Readable for Decree {
    const SIZE: Option<usize> = None;

    fn read<R>(reader: &mut R) -> std::io::Result<Self>
    where
        R: std::io::Read,
    {
        let decree = match Some(u8::read(reader)?) {
            <RegisterChain as TypePrefixedPayload>::TYPE => {
                Decree::RegisterChain(Readable::read(reader)?)
            }
            <ContractUpgrade as TypePrefixedPayload>::TYPE => {
                Decree::ContractUpgrade(Readable::read(reader)?)
            }
            <RecoverChainId as TypePrefixedPayload>::TYPE => {
                Decree::RecoverChainId(Readable::read(reader)?)
            }
            _ => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "Invalid Token Bridge decree",
                ))
            }
        };

        Ok(decree)
    }
}
