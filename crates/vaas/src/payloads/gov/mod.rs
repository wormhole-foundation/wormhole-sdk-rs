pub mod core_bridge;
pub use core_bridge::{
    ContractUpgrade, GuardianSetUpdate, RecoverChainId, SetMessageFee, TransferFees,
};

pub mod token_bridge;
pub use token_bridge::RegisterChain;

use alloy_primitives::FixedBytes;
use hex_literal::hex;

use crate::{Payload, Readable, Writeable};

pub const GOVERNANCE_CHAIN: u16 = 1;
pub const GOVERNANCE_EMITTER: FixedBytes<32> = FixedBytes(hex!(
    "0000000000000000000000000000000000000000000000000000000000000004"
));

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct GovernanceHeader {
    pub module: FixedBytes<32>,
}

impl Readable for GovernanceHeader {
    const SIZE: Option<usize> = Some(32);

    fn read<R>(reader: &mut R) -> std::io::Result<Self>
    where
        R: std::io::Read,
    {
        Ok(Self {
            module: FixedBytes::read(reader)?,
        })
    }
}

impl Writeable for GovernanceHeader {
    fn write<W>(&self, writer: &mut W) -> std::io::Result<()>
    where
        W: std::io::Write,
    {
        self.module.write(writer)
    }

    fn written_size(&self) -> usize {
        <Self as Readable>::SIZE.unwrap()
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct GovernanceMessage<D: Writeable> {
    pub header: GovernanceHeader,
    pub decree: D,
}

impl Payload for GovernanceMessage<core_bridge::Decree> {}

impl Payload for GovernanceMessage<token_bridge::Decree> {}

impl<D: Writeable> Writeable for GovernanceMessage<D> {
    fn write<W>(&self, writer: &mut W) -> std::io::Result<()>
    where
        W: std::io::Write,
    {
        self.header.write(writer)?;
        self.decree.write(writer)
    }

    fn written_size(&self) -> usize {
        self.header.written_size() + self.decree.written_size()
    }
}
