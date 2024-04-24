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

    /// Read the payload as a slice. Under the hood, this uses
    /// [read_payload](TypePrefixedPayload::read_payload).
    ///
    /// NOTE: This method will check that the slice is empty after reading the
    /// payload.
    fn read_slice(buf: &[u8]) -> Result<Self, io::Error> {
        let buf = &mut &buf[..];
        let out = Self::read_payload(buf)?;

        if buf.is_empty() {
            Ok(out)
        } else {
            Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Invalid payload length",
            ))
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

    fn to_vec_payload(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(self.payload_written_size());
        self.write_payload(&mut buf).expect("no alloc failure");
        buf
    }
}

#[cfg(test)]
mod test {
    use crate::{Readable, TypePrefixedPayload, Writeable, WriteableBytes};
    use hex_literal::hex;

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct NineteenBytes([u8; 19]);

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct Message {
        pub a: u32,
        pub b: NineteenBytes,
        pub c: WriteableBytes<u32>,
        pub d: [u64; 4],
        pub e: bool,
    }

    impl TypePrefixedPayload for Message {
        const TYPE: Option<u8> = Some(69);
    }

    impl Readable for Message {
        const SIZE: Option<usize> = Some(88);

        fn read<R>(reader: &mut R) -> std::io::Result<Self>
        where
            Self: Sized,
            R: std::io::Read,
        {
            Ok(Self {
                a: Readable::read(reader)?,
                b: NineteenBytes(Readable::read(reader)?),
                c: Readable::read(reader)?,
                d: Readable::read(reader)?,
                e: Readable::read(reader)?,
            })
        }
    }

    impl Writeable for Message {
        fn written_size(&self) -> usize {
            <Message as Readable>::SIZE.unwrap()
        }

        fn write<W>(&self, writer: &mut W) -> std::io::Result<()>
        where
            W: std::io::Write,
        {
            self.a.write(writer)?;
            self.b.0.write(writer)?;
            self.c.write(writer)?;
            self.d.write(writer)?;
            self.e.write(writer)?;
            Ok(())
        }
    }

    #[test]
    fn to_vec_payload() {
        let msg = Message {
            a: 420,
            b: NineteenBytes(hex!("ba5edba5edba5edba5edba5edba5edba5edba5")),
            c: b"Somebody set us up the bomb.".to_vec().try_into().unwrap(),
            d: [0x45; 4],
            e: true,
        };

        let mut encoded = msg.to_vec_payload();
        assert_eq!(encoded, hex!("45000001a4ba5edba5edba5edba5edba5edba5edba5edba50000001c536f6d65626f6479207365742075732075702074686520626f6d622e000000000000004500000000000000450000000000000045000000000000004501"));
        assert_eq!(encoded.capacity(), msg.payload_written_size());
        assert_eq!(encoded.capacity(), encoded.len());

        let mut cursor = std::io::Cursor::new(&mut encoded);
        let decoded = Message::read_typed(&mut cursor).unwrap();
        assert_eq!(msg, decoded);
    }

    #[test]
    fn invalid_length() {
        let encoded = hex!("45000001a4ba5edba5edba5edba5edba5edba5edba5edba50000001c536f6d65626f6479207365742075732075702074686520626f6d622e00000000000000450000000000000045000000000000004500000000000000450169");

        assert!(matches!(
            Message::read_slice(&encoded).unwrap_err().kind(),
            std::io::ErrorKind::InvalidData,
        ));
    }

    #[test]
    fn read_slice() {
        let encoded = hex!("45000001a4ba5edba5edba5edba5edba5edba5edba5edba50000001c536f6d65626f6479207365742075732075702074686520626f6d622e000000000000004500000000000000450000000000000045000000000000004501");

        let expected = Message {
            a: 420,
            b: NineteenBytes(hex!("ba5edba5edba5edba5edba5edba5edba5edba5")),
            c: b"Somebody set us up the bomb.".to_vec().try_into().unwrap(),
            d: [0x45; 4],
            e: true,
        };

        let decoded = Message::read_slice(&encoded).unwrap();
        assert_eq!(decoded, expected);
    }
}
