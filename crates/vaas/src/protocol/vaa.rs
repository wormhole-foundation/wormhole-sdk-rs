use alloy_primitives::{FixedBytes, U64};

use crate::{payloads, utils, Payload};
pub use crate::{GuardianSetSig, Readable, Writeable};

use std::io;

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Vaa {
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub header: VaaHeader,
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub body: VaaBody,
}

impl Readable for Vaa {
    fn read<R>(reader: &mut R) -> io::Result<Self>
    where
        Self: Sized,
        R: io::Read,
    {
        let header = VaaHeader::read(reader)?;
        let body = VaaBody::read(reader)?;
        Ok(Self { header, body })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct VaaHeader {
    pub version: u8,
    pub guardian_set_index: u32,
    pub signatures: Vec<GuardianSetSig>,
}

impl Writeable for VaaHeader {
    fn write<W>(&self, writer: &mut W) -> io::Result<()>
    where
        W: io::Write,
    {
        writer.write_all(&[self.version])?;
        writer.write_all(&self.guardian_set_index.to_be_bytes())?;
        writer.write_all(&[self.signatures.len() as u8])?;
        self.signatures
            .iter()
            .try_for_each(|sig| sig.write(writer))?;
        Ok(())
    }
}

impl Readable for VaaHeader {
    fn read<R>(reader: &mut R) -> io::Result<Self>
    where
        R: io::Read,
    {
        let mut buf = [0u8; 1 + 4 + 1];
        reader.read_exact(&mut buf)?;

        let version = buf[0];
        let guardian_set_index = u32::from_be_bytes(buf[1..5].try_into().unwrap());
        let sig_count = buf[5] as usize;

        let mut signatures: Vec<_> = Vec::with_capacity(sig_count);
        for _ in 0..sig_count {
            signatures.push(GuardianSetSig::read(reader)?);
        }

        Ok(Self {
            version,
            guardian_set_index,
            signatures,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct VaaBody {
    pub timestamp: u32,
    pub nonce: u32,
    pub emitter_chain: u16,
    pub emitter_address: FixedBytes<32>,
    pub sequence: U64,
    pub consistency_level: u8,
    #[cfg_attr(feature = "serde", serde(rename = "serializedPayload"))]
    pub payload: alloy_primitives::Bytes,
}

impl VaaBody {
    #[inline]
    pub fn digest(&self) -> FixedBytes<32> {
        utils::keccak256(self.to_vec())
    }

    #[inline]
    pub fn double_digest(&self) -> FixedBytes<32> {
        utils::keccak256(self.digest())
    }
}

impl Writeable for VaaBody {
    fn write<W>(&self, writer: &mut W) -> io::Result<()>
    where
        W: io::Write,
    {
        self.timestamp.write(writer)?;
        self.nonce.write(writer)?;
        self.emitter_chain.write(writer)?;
        self.emitter_address.write(writer)?;
        self.sequence.into_limbs()[0].write(writer)?;
        self.consistency_level.write(writer)?;
        writer.write_all(&self.payload)?;
        Ok(())
    }
}

impl Readable for VaaBody {
    fn read<R>(reader: &mut R) -> io::Result<Self>
    where
        R: io::Read,
    {
        Ok(Self {
            timestamp: u32::read(reader)?,
            nonce: u32::read(reader)?,
            emitter_chain: u16::read(reader)?,
            emitter_address: <FixedBytes<32>>::read(reader)?,
            sequence: U64::from_limbs([u64::read(reader)?]),
            consistency_level: u8::read(reader)?,
            payload: {
                let mut buf = Vec::new();
                reader.read_to_end(&mut buf)?;
                buf.into()
            },
        })
    }
}

impl VaaBody {
    pub fn read_payload<P: Payload>(&self) -> Option<P> {
        let deser = P::read(&mut self.payload.as_ref()).ok()?;

        let mut reser = Vec::with_capacity(self.payload.len());
        P::write(&deser, &mut reser).expect("no alloc issue");

        (reser == self.payload).then_some(deser)
    }

    pub fn payload_as_message(&self) -> Option<payloads::Message> {
        self.read_payload()
    }
}
