mod message;
use std::io;

pub use message::Message;

use crate::{Readable, Writeable};

pub mod gov;
pub mod token_bridge;

/// Trait to capture common payload behavior. We do not recommend overwriting
/// any trait methods. Simply set the type constant and implement [`Readable`]
/// and [`Writeable`].
pub trait TypePrefixedPayload:
    crate::Readable + crate::Writeable + Clone + std::fmt::Debug
{
    const TYPE: Option<u8>;

    /// Read the payload, including the type prefix.
    fn read_typed<R: io::Read>(reader: &mut R) -> Result<Self, io::Error> {
        let payload_type = u8::read(reader)?;
        if payload_type == Self::TYPE.expect("Called write_typed on untyped payload") {
            Self::read(reader)
        } else {
            Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Invalid payload type",
            ))
        }
    }

    /// Write the payload, including the type prefix.
    fn write_typed<W: io::Write>(&self, writer: &mut W) -> Result<(), io::Error> {
        Self::TYPE
            .expect("Called write_typed on untyped payload")
            .write(writer)?;
        Writeable::write(self, writer)
    }

    /// Read the payload, including the type prefix if applicable.
    fn read_payload<R: io::Read>(reader: &mut R) -> Result<Self, io::Error> {
        match Self::TYPE {
            Some(_) => Self::read_typed(reader),
            None => Readable::read(reader),
        }
    }

    /// Write the payload, including the type prefix if applicable.
    fn write_payload<W: io::Write>(&self, writer: &mut W) -> Result<(), io::Error> {
        match Self::TYPE {
            Some(_) => self.write_typed(writer),
            None => Writeable::write(self, writer),
        }
    }

    /// Returns the size of the payload, including the type prefix.
    fn payload_written_size(&self) -> usize {
        match Self::TYPE {
            Some(_) => self.written_size() + 1,
            None => self.written_size(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(untagged)
)]
#[non_exhaustive]
pub enum PayloadKind {
    Binary(Vec<u8>),
    #[cfg(feature = "serde")]
    Json(serde_json::Value),
}

impl Readable for PayloadKind {
    const SIZE: Option<usize> = None;

    fn read<R>(reader: &mut R) -> io::Result<Self>
    where
        Self: Sized,
        R: io::Read,
    {
        let mut buf = vec![];
        reader.read_to_end(&mut buf)?;
        Ok(Self::Binary(buf))
    }
}

impl Writeable for PayloadKind {
    fn written_size(&self) -> usize {
        #[allow(unreachable_patterns)]
        match self {
            PayloadKind::Binary(buf) => buf.len(),
            _ => 0,
        }
    }

    fn write<W>(&self, writer: &mut W) -> io::Result<()>
    where
        W: io::Write,
    {
        #[allow(unreachable_patterns)]
        match self {
            Self::Binary(buf) => writer.write_all(buf),
            _ => Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Tried to write a JSON payload",
            )),
        }
    }
}

impl TypePrefixedPayload for PayloadKind {
    const TYPE: Option<u8> = None;
}
