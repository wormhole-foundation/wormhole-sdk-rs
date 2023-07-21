mod attestation;
pub use attestation::Attestation;

mod transfer;
pub use transfer::Transfer;

mod transfer_with_message;
pub use transfer_with_message::TransferWithMessage;

use crate::payloads::{Payload, Readable, Writeable};

// TODO: make normalizer struct for norm amount/relayer_fee.

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenBridgeMessage {
    Transfer(Transfer),
    Attestation(Attestation),
    TransferWithMessage(TransferWithMessage),
}

impl TokenBridgeMessage {
    const TRANSFER: u8 = 1;
    const ATTESTATION: u8 = 2;
    const TRANSFER_WITH_MESSAGE: u8 = 3;
}

impl Readable for TokenBridgeMessage {
    const SIZE: Option<usize> = None;

    fn read<R>(reader: &mut R) -> std::io::Result<Self>
    where
        Self: Sized,
        R: std::io::Read,
    {
        match u8::read(reader)? {
            TokenBridgeMessage::TRANSFER => {
                Ok(TokenBridgeMessage::Transfer(Readable::read(reader)?))
            }
            TokenBridgeMessage::ATTESTATION => {
                Ok(TokenBridgeMessage::Attestation(Readable::read(reader)?))
            }
            TokenBridgeMessage::TRANSFER_WITH_MESSAGE => Ok(
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
            TokenBridgeMessage::Transfer(inner) => {
                TokenBridgeMessage::TRANSFER.write(writer)?;
                inner.write(writer)
            }
            TokenBridgeMessage::Attestation(inner) => {
                TokenBridgeMessage::ATTESTATION.write(writer)?;
                inner.write(writer)
            }
            TokenBridgeMessage::TransferWithMessage(inner) => {
                TokenBridgeMessage::TRANSFER_WITH_MESSAGE.write(writer)?;
                inner.write(writer)
            }
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

impl Payload for TokenBridgeMessage {}
