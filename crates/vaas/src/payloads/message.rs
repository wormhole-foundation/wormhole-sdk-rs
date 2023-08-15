use crate::{Readable, Writeable};

use std::io;

impl crate::payloads::TypePrefixedPayload for Message {
    const TYPE: Option<u8> = Some(0xbb);
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Message {
    pub version: u8,
    pub message_ty: u8,
    pub index: u64,
    pub target_chain: u16,
    pub target: Vec<u8>,
    pub sender: Vec<u8>,
    pub body: Vec<u8>,
}

impl Readable for Message {
    const SIZE: Option<usize> = None;

    fn read<R>(reader: &mut R) -> io::Result<Self>
    where
        R: io::Read,
    {
        let version = u8::read(reader)?;
        let message_ty = u8::read(reader)?;
        let index = u64::read(reader)?;
        let target_chain = u16::read(reader)?;

        dbg!("a");

        let target_len = u16::read(reader)?;
        let mut target = vec![0u8; target_len as usize];
        reader.read_exact(&mut target)?;

        let sender_len = u16::read(reader)?;
        let mut sender = vec![0u8; sender_len as usize];
        reader.read_exact(sender.as_mut_slice())?;

        let body_len = u16::read(reader)?;
        let mut body = vec![0u8; body_len as usize];
        reader.read_exact(&mut body)?;

        Ok(Self {
            version,
            message_ty,
            index,
            target_chain,
            sender,
            target,
            body,
        })
    }
}

impl Writeable for Message {
    fn write<W>(&self, writer: &mut W) -> io::Result<()>
    where
        W: io::Write,
    {
        self.version.write(writer)?;
        self.message_ty.write(writer)?;
        self.index.write(writer)?;
        self.target_chain.write(writer)?;
        (self.target.len() as u16).write(writer)?;
        writer.write_all(&self.target)?;
        (self.sender.len() as u16).write(writer)?;
        writer.write_all(&self.sender)?;
        (self.body.len() as u16).write(writer)?;
        writer.write_all(&self.body)?;
        Ok(())
    }

    fn written_size(&self) -> usize {
        1 + 8 + 2 + 2 + self.sender.len() + 2 + self.target.len() + 2 + self.body.len()
    }
}

#[cfg(test)]
mod test {
    use hex_literal::hex;

    use super::*;
    #[test]
    fn it_roundtrips() {
        let message = hex!(
            "0000000000000000000012340002567800147fa9385be102ac3eac297483dd6233d62b3e149600029abc"
        );

        let message = Message::read(&mut &message[..]).unwrap();
        dbg!(&message);

        assert_eq!(message.index, 0);
        assert_eq!(message.target_chain, 0x1234);
        assert_eq!(message.target, hex!("5678"));
        assert_eq!(
            message.sender,
            hex!("7fa9385be102ac3eac297483dd6233d62b3e1496")
        );
        assert_eq!(message.body, hex!("9abc"));
    }
}
