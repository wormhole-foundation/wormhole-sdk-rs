use std::io;

use crate::{Readable, Writeable};

/// Trait to capture common payload behavior. We do not recommend overwriting
/// any trait methods. Simply set the type constant and implement [`Readable`]
/// and [`Writeable`].
pub trait TypePrefixedPayload: Readable + Writeable + Clone + std::fmt::Debug {
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
