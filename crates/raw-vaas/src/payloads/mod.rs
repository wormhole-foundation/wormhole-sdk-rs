pub mod core;
pub mod token_bridge;

use crate::Payload;

/// A governance Message with header and type flag.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct GovernanceMessage<'a> {
    span: &'a [u8],

    header: GovernanceHeader<'a>,
    decree: Payload<'a>,
}

impl AsRef<[u8]> for GovernanceMessage<'_> {
    fn as_ref(&self) -> &[u8] {
        self.span
    }
}

impl<'a> TryFrom<&'a [u8]> for GovernanceMessage<'a> {
    type Error = &'static str;

    fn try_from(value: &'a [u8]) -> Result<Self, Self::Error> {
        Self::parse(value)
    }
}

impl<'a> GovernanceMessage<'a> {
    pub fn span(&self) -> &[u8] {
        self.span
    }

    pub fn header(&self) -> GovernanceHeader<'a> {
        self.header
    }

    pub fn decree(&self) -> Payload<'a> {
        self.decree
    }

    pub fn parse(span: &'a [u8]) -> Result<GovernanceMessage<'a>, &'static str> {
        if span.len() < 1 {
            return Err("GovernanceMessage span too short. Need at least 1 byte");
        }

        let header = GovernanceHeader::parse(span)?;
        let decree = Payload::parse(&span[32..]);

        Ok(GovernanceMessage {
            span,
            header,
            decree,
        })
    }
}

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
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct GovernanceHeader<'a> {
    span: &'a [u8],
}

impl AsRef<[u8]> for GovernanceHeader<'_> {
    fn as_ref(&self) -> &[u8] {
        self.span
    }
}

impl<'a> TryFrom<&'a [u8]> for GovernanceHeader<'a> {
    type Error = &'static str;

    fn try_from(span: &'a [u8]) -> Result<GovernanceHeader<'a>, &'static str> {
        GovernanceHeader::parse(span)
    }
}

impl<'a> GovernanceHeader<'a> {
    pub fn module(&self) -> [u8; 32] {
        self.span[0..32].try_into().unwrap()
    }

    pub fn parse(span: &'a [u8]) -> Result<GovernanceHeader<'a>, &'static str> {
        if span.len() < 32 {
            return Err("GovernanceHeader span too short. Need at least 32 bytes");
        }

        Ok(GovernanceHeader { span: &span[..32] })
    }
}
