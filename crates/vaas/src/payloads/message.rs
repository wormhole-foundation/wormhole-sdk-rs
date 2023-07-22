use crate::{Readable, Writeable};

use std::io;

impl crate::payloads::TypePrefixedPayload for Message {
    const TYPE: Option<u8> = None;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Message {
    pub(crate) version: u8,
    pub(crate) index: u64,
    pub(crate) target_chain: u16,
    pub(crate) sender: Vec<u8>,
    pub(crate) target: Vec<u8>,
    pub(crate) message: Vec<u8>,
}

impl Readable for Message {
    const SIZE: Option<usize> = None;

    fn read<R>(reader: &mut R) -> io::Result<Self>
    where
        R: io::Read,
    {
        let version = u8::read(reader)?;
        let index = u64::read(reader)?;
        let target_chain = u16::read(reader)?;

        let sender_len = u16::read(reader)?;
        let mut sender = vec![0u8; sender_len as usize];
        reader.read_exact(sender.as_mut_slice())?;

        let target_len = u16::read(reader)?;
        let mut target = vec![0u8; target_len as usize];
        reader.read_exact(&mut target)?;

        let message_len = u16::read(reader)?;
        let mut message = vec![0u8; message_len as usize];
        reader.read_exact(&mut message)?;

        Ok(Self {
            version,
            index,
            target_chain,
            sender,
            target,
            message,
        })
    }
}

impl Writeable for Message {
    fn write<W>(&self, writer: &mut W) -> io::Result<()>
    where
        W: io::Write,
    {
        self.version.write(writer)?;
        self.index.write(writer)?;
        self.target_chain.write(writer)?;
        (self.sender.len() as u16).write(writer)?;
        writer.write_all(&self.sender)?;
        (self.target.len() as u16).write(writer)?;
        writer.write_all(&self.target)?;
        (self.message.len() as u16).write(writer)?;
        writer.write_all(&self.message)?;
        Ok(())
    }

    fn written_size(&self) -> usize {
        1 + 8 + 2 + 2 + self.sender.len() + 2 + self.target.len() + 2 + self.message.len()
    }
}
