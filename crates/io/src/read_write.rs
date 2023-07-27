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

impl Readable for u16 {
    fn read<R>(reader: &mut R) -> io::Result<Self>
    where
        R: io::Read,
    {
        let mut buf = [0u8; 2];
        reader.read_exact(&mut buf)?;
        Ok(u16::from_be_bytes(buf))
    }

    const SIZE: Option<usize> = Some(2);
}

impl Writeable for u16 {
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

impl Readable for u32 {
    const SIZE: Option<usize> = Some(4);

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

impl Readable for u64 {
    const SIZE: Option<usize> = Some(8);

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
