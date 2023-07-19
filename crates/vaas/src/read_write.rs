use std::io;

use alloy_primitives::{FixedBytes, Uint};

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

    fn to_vec(&self) -> Vec<u8> {
        let mut buf = Vec::new();
        self.write(&mut buf).expect("no alloc failure");
        buf
    }
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

impl Readable for u16 {
    fn read<R>(reader: &mut R) -> io::Result<Self>
    where
        R: io::Read,
    {
        let mut buf = [0u8; 2];
        reader.read_exact(&mut buf)?;
        Ok(u16::from_be_bytes(buf))
    }
}

impl Writeable for u16 {
    fn write<W>(&self, writer: &mut W) -> io::Result<()>
    where
        W: io::Write,
    {
        writer.write_all(&self.to_be_bytes())
    }
}

impl Readable for u32 {
    fn read<R>(reader: &mut R) -> io::Result<Self>
    where
        R: io::Read,
    {
        let mut buf = [0u8; 4];
        reader.read_exact(&mut buf)?;
        Ok(u32::from_be_bytes(buf))
    }
}

impl Writeable for u32 {
    fn write<W>(&self, writer: &mut W) -> io::Result<()>
    where
        W: io::Write,
    {
        writer.write_all(&self.to_be_bytes())
    }
}

impl Readable for u64 {
    fn read<R>(reader: &mut R) -> io::Result<Self>
    where
        R: io::Read,
    {
        let mut buf = [0u8; 8];
        reader.read_exact(&mut buf)?;
        Ok(u64::from_be_bytes(buf))
    }
}

impl Writeable for u64 {
    fn write<W>(&self, writer: &mut W) -> io::Result<()>
    where
        W: io::Write,
    {
        writer.write_all(&self.to_be_bytes())
    }
}

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

impl<const N: usize> Readable for FixedBytes<N> {
    fn read<R>(reader: &mut R) -> io::Result<Self>
    where
        Self: Sized,
        R: io::Read,
    {
        <[u8; N]>::read(reader).map(Self)
    }
}

impl<const N: usize> Writeable for FixedBytes<N> {
    fn write<W>(&self, writer: &mut W) -> io::Result<()>
    where
        W: io::Write,
    {
        self.0.write(writer)
    }
}

impl<const BITS: usize, const LIMBS: usize> Readable for Uint<BITS, LIMBS> {
    fn read<R>(reader: &mut R) -> io::Result<Self>
    where
        Self: Sized,
        R: io::Read,
    {
        let mut buf = Uint::<BITS, LIMBS>::default().to_be_bytes_vec();
        reader.read_exact(buf.as_mut_slice())?;

        Ok(Uint::try_from_be_slice(buf.as_slice()).unwrap())
    }
}

impl<const BITS: usize, const LIMBS: usize> Writeable for Uint<BITS, LIMBS> {
    fn write<W>(&self, writer: &mut W) -> io::Result<()>
    where
        W: io::Write,
    {
        writer.write_all(self.to_be_bytes_vec().as_slice())
    }
}
