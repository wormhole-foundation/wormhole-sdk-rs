mod attestation;
pub use attestation::Attestation;

mod transfer;
pub use transfer::Transfer;

mod transfer_with_message;
pub use transfer_with_message::TransferWithMessage;

use crate::payloads::{Readable, TypePrefixedPayload, Writeable};

// TODO: make normalizer struct for norm amount/relayer_fee.

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenBridgeMessage {
    Transfer(Transfer),
    Attestation(Attestation),
    TransferWithMessage(TransferWithMessage),
}

impl Readable for TokenBridgeMessage {
    const SIZE: Option<usize> = None;

    fn read<R>(reader: &mut R) -> std::io::Result<Self>
    where
        Self: Sized,
        R: std::io::Read,
    {
        match Some(u8::read(reader)?) {
            <Transfer as TypePrefixedPayload>::TYPE => {
                Ok(TokenBridgeMessage::Transfer(Readable::read(reader)?))
            }
            <Attestation as TypePrefixedPayload>::TYPE => {
                Ok(TokenBridgeMessage::Attestation(Readable::read(reader)?))
            }
            <TransferWithMessage as TypePrefixedPayload>::TYPE => Ok(
                TokenBridgeMessage::TransferWithMessage(Readable::read(reader)?),
            ),
            _ => Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Invalid token bridge message type",
            )),
        }
    }
}

impl Writeable for TokenBridgeMessage {
    fn write<W>(&self, writer: &mut W) -> std::io::Result<()>
    where
        W: std::io::Write,
    {
        match self {
            TokenBridgeMessage::Transfer(inner) => inner.write_payload(writer),
            TokenBridgeMessage::Attestation(inner) => inner.write_payload(writer),
            TokenBridgeMessage::TransferWithMessage(inner) => inner.write_payload(writer),
        }
    }

    fn written_size(&self) -> usize {
        1 + match self {
            TokenBridgeMessage::Transfer(inner) => inner.written_size(),
            TokenBridgeMessage::Attestation(inner) => inner.written_size(),
            TokenBridgeMessage::TransferWithMessage(inner) => inner.written_size(),
        }
    }
}

impl TypePrefixedPayload for TokenBridgeMessage {
    const TYPE: Option<u8> = None;
}
