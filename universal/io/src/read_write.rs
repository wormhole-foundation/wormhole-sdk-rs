use std::{io, marker::PhantomData};

pub trait Readable {
    fn read<R>(reader: &mut R) -> io::Result<Self>
    where
        Self: Sized,
        R: io::Read;
}

pub trait Writeable {
    fn write<W>(&self, writer: &mut W) -> io::Result<()>
    where
        W: io::Write;
}

impl Readable for u8 {
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
    fn write<W>(&self, writer: &mut W) -> io::Result<()>
    where
        W: io::Write,
    {
        writer.write_all(&[*self])
    }
}

impl Readable for bool {
    fn read<R>(reader: &mut R) -> io::Result<Self>
    where
        R: io::Read,
    {
        match u8::read(reader)? {
            0 => Ok(false),
            1 => Ok(true),
            _ => Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "invalid bool value",
            )),
        }
    }
}

impl Writeable for bool {
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

/// Wrapper for `Vec<u8>`. Encoding is similar to Borsh, where the length is encoded as u32 (but in
/// this case, it's big endian).
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct WriteableBytes<L>
where
    u32: From<L>,
    L: Sized + Readable + Writeable + TryFrom<usize>,
{
    phantom: PhantomData<L>,
    inner: Vec<u8>,
}

impl<L> WriteableBytes<L>
where
    u32: From<L>,
    L: Sized + Readable + Writeable + TryFrom<usize>,
{
    pub fn new(inner: Vec<u8>) -> Self {
        Self {
            phantom: PhantomData,
            inner,
        }
    }

    pub fn try_encoded_len(&self) -> io::Result<L> {
        match L::try_from(self.inner.len()) {
            Ok(len) => Ok(len),
            Err(_) => Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "L overflow when converting from usize",
            )),
        }
    }

    pub fn written_size(&self) -> usize {
        std::mem::size_of::<L>() + self.inner.len()
    }
}

impl<L> TryFrom<Vec<u8>> for WriteableBytes<L>
where
    u32: From<L>,
    L: Sized + Readable + Writeable + TryFrom<usize>,
{
    type Error = <L as TryFrom<usize>>::Error;

    fn try_from(vec: Vec<u8>) -> Result<Self, Self::Error> {
        match L::try_from(vec.len()) {
            Ok(_) => Ok(Self {
                phantom: PhantomData,
                inner: vec,
            }),
            Err(e) => Err(e),
        }
    }
}

impl<L> From<WriteableBytes<L>> for Vec<u8>
where
    u32: From<L>,
    L: Sized + Readable + Writeable + TryFrom<usize>,
{
    fn from(bytes: WriteableBytes<L>) -> Self {
        bytes.inner
    }
}

impl<L> std::ops::Deref for WriteableBytes<L>
where
    L: Sized + Readable + Writeable,
    u32: From<L>,
    L: TryFrom<usize>,
{
    type Target = Vec<u8>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<L> std::ops::DerefMut for WriteableBytes<L>
where
    u32: From<L>,
    L: Sized + Readable + Writeable + TryFrom<usize>,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl Readable for WriteableBytes<u8> {
    fn read<R>(reader: &mut R) -> io::Result<Self>
    where
        Self: Sized,
        R: io::Read,
    {
        let len = u8::read(reader)?;
        let mut inner: Vec<u8> = vec![0u8; len.into()];
        reader.read_exact(&mut inner)?;
        Ok(Self {
            phantom: PhantomData,
            inner,
        })
    }
}

impl Readable for WriteableBytes<u16> {
    fn read<R>(reader: &mut R) -> io::Result<Self>
    where
        Self: Sized,
        R: io::Read,
    {
        let len = u16::read(reader)?;
        let mut inner = vec![0u8; len.into()];
        reader.read_exact(&mut inner)?;
        Ok(Self {
            phantom: PhantomData,
            inner,
        })
    }
}

impl Readable for WriteableBytes<u32> {
    fn read<R>(reader: &mut R) -> io::Result<Self>
    where
        Self: Sized,
        R: io::Read,
    {
        let len = u32::read(reader)?;
        match len.try_into() {
            Ok(len) => {
                let mut inner = vec![0u8; len];
                reader.read_exact(&mut inner)?;
                Ok(Self {
                    phantom: PhantomData,
                    inner,
                })
            }
            Err(_) => Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "u32 overflow when converting to usize",
            )),
        }
    }
}

impl<L> Writeable for WriteableBytes<L>
where
    u32: From<L>,
    L: Sized + Readable + Writeable + TryFrom<usize>,
{
    fn write<W>(&self, writer: &mut W) -> io::Result<()>
    where
        W: io::Write,
    {
        match self.try_encoded_len() {
            Ok(len) => {
                len.write(writer)?;
                writer.write_all(&self.inner)
            }
            Err(e) => Err(e),
        }
    }
}

#[cfg(feature = "alloy")]
impl<const N: usize> Readable for alloy_primitives::FixedBytes<N> {
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
    fn write<W>(&self, writer: &mut W) -> io::Result<()>
    where
        W: io::Write,
    {
        self.0.write(writer)
    }
}

#[cfg(feature = "alloy")]
impl<const BITS: usize, const LIMBS: usize> Readable for alloy_primitives::Uint<BITS, LIMBS> {
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
    fn write<W>(&self, writer: &mut W) -> io::Result<()>
    where
        W: io::Write,
    {
        writer.write_all(self.to_be_bytes_vec().as_slice())
    }
}

#[cfg(feature = "alloy")]
impl Readable for alloy_primitives::Address {
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

        let value = 69u8;

        let mut encoded = Vec::<u8>::with_capacity(EXPECTED_SIZE);
        let mut writer = std::io::Cursor::new(&mut encoded);
        value.write(&mut writer).unwrap();

        let expected = hex!("45");
        assert_eq!(encoded, expected);
    }

    #[test]
    fn u64_read_write() {
        const EXPECTED_SIZE: usize = 8;

        let value = 69u64;
        let mut encoded = Vec::<u8>::with_capacity(EXPECTED_SIZE);
        let mut writer = std::io::Cursor::new(&mut encoded);
        value.write(&mut writer).unwrap();

        let expected = hex!("0000000000000045");
        assert_eq!(encoded, expected);
    }

    #[test]
    fn u8_array_read_write() {
        let data = [1, 2, 8, 16, 32, 64, 69u8];

        let mut encoded = Vec::<u8>::with_capacity(data.len());
        let mut writer = std::io::Cursor::new(&mut encoded);
        data.write(&mut writer).unwrap();

        let expected = hex!("01020810204045");
        assert_eq!(encoded, expected);
    }

    #[test]
    fn u64_array_read_write() {
        let data = [1, 2, 8, 16, 32, 64, 69u64];
        const EXPECTED_SIZE: usize = 56;

        let mut encoded = Vec::<u8>::with_capacity(EXPECTED_SIZE);
        let mut writer = std::io::Cursor::new(&mut encoded);
        data.write(&mut writer).unwrap();

        let expected = hex!("0000000000000001000000000000000200000000000000080000000000000010000000000000002000000000000000400000000000000045");
        assert_eq!(encoded, expected);
    }

    #[test]
    fn variable_bytes_read_write_u8() {
        let data = b"All your base are belong to us.";
        let bytes = WriteableBytes::<u8>::new(data.to_vec());

        let mut encoded = Vec::<u8>::with_capacity(1 + data.len());
        let mut writer = std::io::Cursor::new(&mut encoded);
        bytes.write(&mut writer).unwrap();

        let expected = hex!("1f416c6c20796f75722062617365206172652062656c6f6e6720746f2075732e");
        assert_eq!(encoded, expected);
    }

    #[test]
    fn variable_bytes_read_write_u16() {
        let data = b"All your base are belong to us.";
        let bytes = WriteableBytes::<u16>::new(data.to_vec());

        let mut encoded = Vec::<u8>::with_capacity(2 + data.len());
        let mut writer = std::io::Cursor::new(&mut encoded);
        bytes.write(&mut writer).unwrap();

        let expected = hex!("001f416c6c20796f75722062617365206172652062656c6f6e6720746f2075732e");
        assert_eq!(encoded, expected);
    }

    #[test]
    fn variable_bytes_read_write_u32() {
        let data = b"All your base are belong to us.";
        let bytes = WriteableBytes::<u32>::new(data.to_vec());

        let mut encoded = Vec::<u8>::with_capacity(4 + data.len());
        let mut writer = std::io::Cursor::new(&mut encoded);
        bytes.write(&mut writer).unwrap();

        let expected =
            hex!("0000001f416c6c20796f75722062617365206172652062656c6f6e6720746f2075732e");
        assert_eq!(encoded, expected);
    }

    #[test]
    fn mem_take() {
        let data = b"All your base are belong to us.";
        let mut bytes = WriteableBytes::<u16>::new(data.to_vec());

        let taken = std::mem::take(&mut bytes);
        assert_eq!(taken.as_slice(), data);
    }
}
