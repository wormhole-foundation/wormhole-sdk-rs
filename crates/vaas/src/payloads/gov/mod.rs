pub mod core_bridge;
pub use core_bridge::{
    ContractUpgrade, GuardianSetUpdate, RecoverChainId, SetMessageFee, TransferFees,
};

pub mod token_bridge;
pub use token_bridge::RegisterChain;

use crate::aliases::FixedBytes;
use hex_literal::hex;

use crate::{Readable, TypePrefixedPayload, Writeable};

pub const GOVERNANCE_CHAIN: u16 = 1;
pub const GOVERNANCE_EMITTER: FixedBytes<32> = FixedBytes(hex!(
    "0000000000000000000000000000000000000000000000000000000000000004"
));

/// The [specification] for Governance messages is the following:
/// - module (32 bytes)
/// - action (1 byte)
/// - target chain (2 bytes)
/// - decree (message payload encoding governance instruction).
///
/// The structs in this module deviate from the specification where the header only specifies the
/// module for which smart contract the governance is relevant. What this SDK calls the payload
/// starts with an action discriminator (1 byte) and the remaining bytes is the governance decree,
/// which for all of these governance decrees will start with two bytes. Either these two bytes will
/// be zeroed out (for global governance actions) or it will encode the chain ID relevant for the
/// governance action.
///
/// [specification]: https://docs.wormhole.com/wormhole/explore-wormhole/vaa#governance
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
pub struct GovernanceMessage<P> {
    pub header: GovernanceHeader,
    pub decree: P,
}

impl<P: TypePrefixedPayload> TypePrefixedPayload for GovernanceMessage<P> {
    const TYPE: Option<u8> = None;
}

impl<P: TypePrefixedPayload> Writeable for GovernanceMessage<P> {
    fn write<W>(&self, writer: &mut W) -> std::io::Result<()>
    where
        W: std::io::Write,
    {
        self.header.write(writer)?;
        self.decree.write_payload(writer)
    }

    fn written_size(&self) -> usize {
        self.header.written_size() + self.decree.payload_written_size()
    }
}

impl<P: TypePrefixedPayload> Readable for GovernanceMessage<P> {
    const SIZE: Option<usize> = None;

    fn read<R>(reader: &mut R) -> std::io::Result<Self>
    where
        Self: Sized,
        R: std::io::Read,
    {
        Ok(Self {
            header: Readable::read(reader)?,
            decree: TypePrefixedPayload::read_payload(reader)?,
        })
    }
}
