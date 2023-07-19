mod message;
pub use message::Message;

use crate::{Readable, Writeable};

pub mod gov;
pub mod portal;

/// Trait to capture common payload behavior.
pub trait Payload: crate::Readable + crate::Writeable + Clone + std::fmt::Debug {}

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

    fn read<R>(reader: &mut R) -> std::io::Result<Self>
    where
        Self: Sized,
        R: std::io::Read,
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

    fn write<W>(&self, writer: &mut W) -> std::io::Result<()>
    where
        W: std::io::Write,
    {
        #[allow(unreachable_patterns)]
        match self {
            Self::Binary(buf) => writer.write_all(buf),
            _ => Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Tried to write a JSON payload",
            )),
        }
    }
}
