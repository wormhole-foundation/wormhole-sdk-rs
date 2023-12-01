use std::io;

pub trait Readable {
    const SIZE: Option<usize>;

    fn read<R>(reader: &mut R) -> io::Result<Self>
    where
        Self: Sized,
        R: io::Read;
}

pub trait Writeable {
    fn write<W>(&self, writer: &mut W) -> io::Result<()>
    where
        W: io::Write;

    fn written_size(&self) -> usize;

    fn to_vec(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(self.written_size());
        self.write(&mut buf).expect("no alloc failure");
        buf
    }
}

impl Readable for u8 {
    const SIZE: Option<usize> = Some(1);

    fn read<R>(reader: &mut R) -> io::Result<Self>
    where
        R: io::Read,
    {
        let mut buf = [0u8; 1];
        reader.read_exact(&mut buf)?;
        Ok(buf[0])
    }
}

impl Writeable for u8 {
    fn written_size(&self) -> usize {
        <Self as Readable>::SIZE.unwrap()
    }

    fn write<W>(&self, writer: &mut W) -> io::Result<()>
    where
        W: io::Write,
    {
        writer.write_all(&[*self])
    }
}

impl Readable for bool {
    const SIZE: Option<usize> = <u8 as Readable>::SIZE;

    fn read<R>(reader: &mut R) -> io::Result<Self>
    where
        R: io::Read,
    {
        match u8::read(reader)? {
            0 => Ok(false),
            1 => Ok(true),
            _ => {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "invalid bool value",
                ))
            }
        }
    }
}

impl Writeable for bool {
    fn written_size(&self) -> usize {
        <Self as Readable>::SIZE.unwrap()
    }

    fn write<W>(&self, writer: &mut W) -> io::Result<()>
    where
        W: io::Write,
    {
        writer.write_all(&[u8::from(*self)])
    }
}

macro_rules! impl_for_int {
    ($type:ty) => {
        impl Readable for $type {
            const SIZE: Option<usize> = Some(std::mem::size_of::<$type>());

            fn read<R>(reader: &mut R) -> io::Result<Self>
            where
                R: io::Read,
            {
                let mut buf = [0u8; std::mem::size_of::<$type>()];
                reader.read_exact(&mut buf)?;
                Ok(Self::from_be_bytes(buf))
            }
        }

        impl Writeable for $type {
            fn written_size(&self) -> usize {
                <Self as Readable>::SIZE.unwrap()
            }

            fn write<W>(&self, writer: &mut W) -> io::Result<()>
            where
                W: io::Write,
            {
                writer.write_all(&self.to_be_bytes())
            }
        }
    };
}

impl_for_int!(u16);
impl_for_int!(u32);
impl_for_int!(u64);
impl_for_int!(u128);

impl_for_int!(i8);
impl_for_int!(i16);
impl_for_int!(i32);
impl_for_int!(i64);
impl_for_int!(i128);

impl<const N: usize> Readable for [u8; N] {
    const SIZE: Option<usize> = Some(N);

    fn read<R>(reader: &mut R) -> io::Result<Self>
    where
        Self: Sized,
        R: io::Read,
    {
        let mut buf = [0u8; N];
        reader.read_exact(&mut buf)?;
        Ok(buf)
    }
}

impl<const N: usize> Writeable for [u8; N] {
    fn written_size(&self) -> usize {
        <Self as Readable>::SIZE.unwrap()
    }

    fn write<W>(&self, writer: &mut W) -> io::Result<()>
    where
        W: io::Write,
    {
        writer.write_all(self)
    }
}

macro_rules! impl_for_int_array {
    ($type:ty) => {
        impl<const N: usize> Readable for [$type; N] {
            const SIZE: Option<usize> = Some(N * std::mem::size_of::<$type>());

            fn read<R>(reader: &mut R) -> io::Result<Self>
            where
                R: io::Read,
            {
                let mut buf = [Default::default(); N];
                for i in 0..N {
                    buf[i] = <$type>::read(reader)?;
                }
                Ok(buf)
            }
        }

        impl<const N: usize> Writeable for [$type; N] {
            fn written_size(&self) -> usize {
                <Self as Readable>::SIZE.unwrap()
            }

            fn write<W>(&self, writer: &mut W) -> io::Result<()>
            where
                W: io::Write,
            {
                for i in 0..N {
                    self[i].write(writer)?;
                }
                Ok(())
            }
        }
    };
}

impl_for_int_array!(u16);
impl_for_int_array!(u32);
impl_for_int_array!(u64);
impl_for_int_array!(u128);

impl_for_int_array!(i8);
impl_for_int_array!(i16);
impl_for_int_array!(i32);
impl_for_int_array!(i64);
impl_for_int_array!(i128);

/// Wrapper for Vec<u8>. Encoding is similar to Borsh, where the length is encoded as u32 (but in
/// this case, it's big endian).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VariableBytes(pub Vec<u8>);

impl From<Vec<u8>> for VariableBytes {
    fn from(vec: Vec<u8>) -> Self {
        Self(vec)
    }
}

impl From<VariableBytes> for Vec<u8> {
    fn from(bytes: VariableBytes) -> Self {
        bytes.0
    }
}

impl std::ops::Deref for VariableBytes {
    type Target = Vec<u8>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for VariableBytes {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Readable for VariableBytes {
    const SIZE: Option<usize> = None;

    fn read<R>(reader: &mut R) -> io::Result<Self>
    where
        Self: Sized,
        R: io::Read,
    {
        let len = usize::try_from(u32::read(reader)?).expect("usize overflow");
        let mut buf = vec![0u8; len];
        reader.read_exact(&mut buf)?;
        Ok(Self(buf))
    }
}

impl Writeable for VariableBytes {
    fn written_size(&self) -> usize {
        4 + self.0.len()
    }

    fn write<W>(&self, writer: &mut W) -> io::Result<()>
    where
        W: io::Write,
    {
        (u32::try_from(self.0.len()).expect("u32 overflow")).write(writer)?;
        writer.write_all(&self.0)
    }
}

#[cfg(feature = "alloy")]
impl<const N: usize> Readable for alloy_primitives::FixedBytes<N> {
    const SIZE: Option<usize> = Some(N);

    fn read<R>(reader: &mut R) -> io::Result<Self>
    where
        Self: Sized,
        R: io::Read,
    {
        <[u8; N]>::read(reader).map(Self)
    }
}

#[cfg(feature = "alloy")]
impl<const N: usize> Writeable for alloy_primitives::FixedBytes<N> {
    fn written_size(&self) -> usize {
        <Self as Readable>::SIZE.unwrap()
    }

    fn write<W>(&self, writer: &mut W) -> io::Result<()>
    where
        W: io::Write,
    {
        self.0.write(writer)
    }
}

#[cfg(feature = "alloy")]
impl<const BITS: usize, const LIMBS: usize> Readable for alloy_primitives::Uint<BITS, LIMBS> {
    const SIZE: Option<usize> = { Some(BITS * 8) };

    fn read<R>(reader: &mut R) -> io::Result<Self>
    where
        Self: Sized,
        R: io::Read,
    {
        let mut buf = alloy_primitives::Uint::<BITS, LIMBS>::default().to_be_bytes_vec();
        reader.read_exact(buf.as_mut_slice())?;

        Ok(alloy_primitives::Uint::try_from_be_slice(buf.as_slice()).unwrap())
    }
}

#[cfg(feature = "alloy")]
impl<const BITS: usize, const LIMBS: usize> Writeable for alloy_primitives::Uint<BITS, LIMBS> {
    fn written_size(&self) -> usize {
        <Self as Readable>::SIZE.unwrap()
    }

    fn write<W>(&self, writer: &mut W) -> io::Result<()>
    where
        W: io::Write,
    {
        writer.write_all(self.to_be_bytes_vec().as_slice())
    }
}

#[cfg(feature = "alloy")]
impl Readable for alloy_primitives::Address {
    const SIZE: Option<usize> = Some(20);

    fn read<R>(reader: &mut R) -> io::Result<Self>
    where
        Self: Sized,
        R: io::Read,
    {
        alloy_primitives::FixedBytes::<20>::read(reader).map(Self)
    }
}

#[cfg(feature = "alloy")]
impl Writeable for alloy_primitives::Address {
    fn written_size(&self) -> usize {
        <Self as Readable>::SIZE.unwrap()
    }

    fn write<W>(&self, writer: &mut W) -> io::Result<()>
    where
        W: io::Write,
    {
        self.0.write(writer)
    }
}

#[cfg(test)]
pub mod test {
    use super::*;
    use hex_literal::hex;

    #[test]
    fn u8_read_write() {
        const EXPECTED_SIZE: usize = 1;
        assert_eq!(u8::SIZE, Some(EXPECTED_SIZE));

        let value = 69u8;
        assert_eq!(value.written_size(), EXPECTED_SIZE);

        let mut encoded = Vec::<u8>::with_capacity(value.written_size());
        let mut writer = std::io::Cursor::new(&mut encoded);
        value.write(&mut writer).unwrap();

        let expected = hex!("45");
        assert_eq!(encoded, expected);
        assert_eq!(value.to_vec(), expected.to_vec());
    }

    #[test]
    fn u64_read_write() {
        const EXPECTED_SIZE: usize = 8;
        assert_eq!(u64::SIZE, Some(EXPECTED_SIZE));

        let value = 69u64;
        assert_eq!(value.written_size(), EXPECTED_SIZE);

        let mut encoded = Vec::<u8>::with_capacity(value.written_size());
        let mut writer = std::io::Cursor::new(&mut encoded);
        value.write(&mut writer).unwrap();

        let expected = hex!("0000000000000045");
        assert_eq!(encoded, expected);
        assert_eq!(value.to_vec(), expected.to_vec());
    }

    #[test]
    fn u8_array_read_write() {
        let data = [1, 2, 8, 16, 32, 64, 69u8];
        assert_eq!(<[u8; 7]>::SIZE, Some(data.len()));
        assert_eq!(data.written_size(), data.len());

        let mut encoded = Vec::<u8>::with_capacity(data.written_size());
        let mut writer = std::io::Cursor::new(&mut encoded);
        data.write(&mut writer).unwrap();

        let expected = hex!("01020810204045");
        assert_eq!(encoded, expected);
        assert_eq!(data.to_vec(), expected.to_vec());
    }

    #[test]
    fn u64_array_read_write() {
        let data = [1, 2, 8, 16, 32, 64, 69u64];
        const EXPECTED_SIZE: usize = 56;
        assert_eq!(<[u64; 7]>::SIZE, Some(EXPECTED_SIZE));
        assert_eq!(data.written_size(), EXPECTED_SIZE);

        let mut encoded = Vec::<u8>::with_capacity(data.written_size());
        let mut writer = std::io::Cursor::new(&mut encoded);
        data.write(&mut writer).unwrap();

        let expected = hex!("0000000000000001000000000000000200000000000000080000000000000010000000000000002000000000000000400000000000000045");
        assert_eq!(encoded, expected);
        assert_eq!(data.to_vec(), expected.to_vec());
    }

    #[test]
    fn variable_bytes_read_write() {
        let data = b"All your base are belong to us.";
        let bytes = VariableBytes(data.to_vec());

        let mut encoded = Vec::<u8>::with_capacity(bytes.written_size());
        let mut writer = std::io::Cursor::new(&mut encoded);
        bytes.write(&mut writer).unwrap();

        let expected =
            hex!("0000001f416c6c20796f75722062617365206172652062656c6f6e6720746f2075732e");
        assert_eq!(encoded, expected);
        assert_eq!(bytes.to_vec(), expected.to_vec());
    }
}
