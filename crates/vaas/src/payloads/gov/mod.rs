use hex_literal::hex;

use alloy_primitives::FixedBytes;

use crate::{Payload, Readable, Writeable};

pub const GOVERNANCE_CHAIN: u16 = 1;
pub const GOVERNANCE_EMITTER: FixedBytes<32> = FixedBytes(hex!(
    "0000000000000000000000000000000000000000000000000000000000000004"
));

/// A.K.A. "Core".
pub const CORE_BRIDGE_GOVERNANCE_MODULE: FixedBytes<32> = FixedBytes(hex!(
    "00000000000000000000000000000000000000000000000000000000436f7265"
));

/// A.K.A. "TokenBridge".
pub const TOKEN_BRIDGE_GOVERNANCE_MODULE: FixedBytes<32> = FixedBytes(hex!(
    "000000000000000000000000000000000000000000546f6b656e427269646765"
));

impl Payload for GovernanceMessage {}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct GovernanceHeader {
    pub module: FixedBytes<32>,
    pub action: u8,
    pub target: u16,
}

impl Readable for GovernanceHeader {
    const SIZE: Option<usize> = Some(32 + 1 + 2);

    fn read<R>(reader: &mut R) -> std::io::Result<Self>
    where
        R: std::io::Read,
    {
        Ok(Self {
            module: FixedBytes::read(reader)?,
            action: u8::read(reader)?,
            target: u16::read(reader)?,
        })
    }
}

impl Writeable for GovernanceHeader {
    fn write<W>(&self, writer: &mut W) -> std::io::Result<()>
    where
        W: std::io::Write,
    {
        self.module.write(writer)?;
        self.action.write(writer)?;
        self.target.write(writer)?;
        Ok(())
    }

    fn written_size(&self) -> usize {
        <Self as Readable>::SIZE.unwrap()
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct GovernanceMessage {
    pub header: GovernanceHeader,
    pub decree: Vec<u8>,
}

impl Readable for GovernanceMessage {
    const SIZE: Option<usize> = None;

    fn read<R>(reader: &mut R) -> std::io::Result<Self>
    where
        R: std::io::Read,
    {
        Ok(Self {
            header: GovernanceHeader::read(reader)?,
            decree: {
                let mut buf = Vec::new();
                reader.read_to_end(&mut buf)?;
                buf
            },
        })
    }
}

impl Writeable for GovernanceMessage {
    fn write<W>(&self, writer: &mut W) -> std::io::Result<()>
    where
        W: std::io::Write,
    {
        self.header.write(writer)?;
        writer.write_all(&self.decree)?;
        Ok(())
    }

    fn written_size(&self) -> usize {
        self.header.written_size() + self.decree.len()
    }
}

impl GovernanceMessage {
    pub fn read_decree<R: Readable>(&self) -> Option<R> {
        R::read(&mut self.decree.as_slice()).ok()
    }
}
