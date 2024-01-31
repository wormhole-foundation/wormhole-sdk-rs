#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Vaa<'a> {
    pub(crate) span: &'a [u8],
    header: Header<'a>,
    body: Body<'a>,
}

impl<'a> AsRef<[u8]> for Vaa<'a> {
    fn as_ref(&self) -> &[u8] {
        self.span
    }
}

impl<'a> TryFrom<&'a [u8]> for Vaa<'a> {
    type Error = &'static str;

    fn try_from(value: &'a [u8]) -> Result<Self, Self::Error> {
        Self::parse(value)
    }
}

impl<'a> Vaa<'a> {
    pub fn version(&self) -> u8 {
        self.header.version()
    }

    pub fn guardian_set_index(&self) -> u32 {
        self.header.guardian_set_index()
    }

    pub fn signature_count(&self) -> u8 {
        self.header.signature_count()
    }

    pub fn signatures(&self) -> impl Iterator<Item = GuardianSetSig<'_>> {
        self.header.signatures()
    }

    pub fn body(&self) -> Body<'a> {
        self.body
    }

    pub fn payload(&self) -> Payload<'a> {
        self.body.payload()
    }

    pub fn parse(span: &'a [u8]) -> Result<Self, &'static str> {
        let header = Header::parse(span)?;
        let body = Body::parse(&span[header.0.len()..])?;

        Ok(Self { span, header, body })
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Header<'a>(pub(crate) &'a [u8]);

impl<'a> AsRef<[u8]> for Header<'a> {
    fn as_ref(&self) -> &[u8] {
        self.0
    }
}

impl<'a> TryFrom<&'a [u8]> for Header<'a> {
    type Error = &'static str;

    fn try_from(value: &'a [u8]) -> Result<Self, Self::Error> {
        Self::parse(value)
    }
}

impl<'a> Header<'a> {
    pub fn version(&self) -> u8 {
        self.0[0]
    }

    pub fn guardian_set_index(&self) -> u32 {
        u32::from_be_bytes(self.0[1..5].try_into().unwrap())
    }

    pub fn signature_count(&self) -> u8 {
        self.0[5]
    }

    pub fn raw_signatures(&self) -> &[u8] {
        &self.0[6..]
    }

    pub fn signatures(&self) -> impl Iterator<Item = GuardianSetSig<'_>> {
        self.raw_signatures()
            .chunks(66)
            .map(GuardianSetSig::unchecked_from)
    }

    pub fn parse(span: &'a [u8]) -> Result<Self, &'static str> {
        if span.len() < 6 {
            return Err("Header: invalid length. Expected at least 6 bytes.");
        }

        let expected_len = 6 + span[5] as usize * 66;

        // slice not long enough to contain all signatures
        if span.len() < expected_len {
            return Err("Header: Insufficient bytes to parse all signatures");
        }

        Ok(Self(&span[..expected_len]))
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Body<'a>(pub(crate) &'a [u8]);

impl<'a> AsRef<[u8]> for Body<'a> {
    fn as_ref(&self) -> &[u8] {
        self.0
    }
}

impl<'a> TryFrom<&'a [u8]> for Body<'a> {
    type Error = &'static str;

    fn try_from(value: &'a [u8]) -> Result<Self, Self::Error> {
        Self::parse(value)
    }
}

impl<'a> Body<'a> {
    pub fn timestamp(&self) -> u32 {
        u32::from_be_bytes(self.0[0..4].try_into().unwrap())
    }

    pub fn nonce(&self) -> u32 {
        u32::from_be_bytes(self.0[4..8].try_into().unwrap())
    }

    pub fn emitter_chain(&self) -> u16 {
        u16::from_be_bytes(self.0[8..10].try_into().unwrap())
    }

    pub fn emitter_address(&self) -> [u8; 32] {
        self.0[10..42].try_into().unwrap()
    }

    pub fn sequence(&self) -> u64 {
        u64::from_be_bytes(self.0[42..50].try_into().unwrap())
    }

    pub fn consistency_level(&self) -> u8 {
        self.0[50]
    }

    pub fn payload(&self) -> Payload<'a> {
        Payload::parse(if self.0.len() < 51 {
            &[]
        } else {
            &self.0[51..]
        })
    }

    pub fn parse(span: &'a [u8]) -> Result<Self, &'static str> {
        if span.len() < 51 {
            return Err("Body: invalid length. Expected at least 51 bytes.");
        }

        Ok(Self(span))
    }

    // available when `off-chain` feature is enabled
    #[inline]
    #[cfg(feature = "off-chain")]
    pub fn digest(&self) -> [u8; 32] {
        crate::utils::keccak256(self)
    }

    // available when `off-chain` feature is enabled
    #[inline]
    #[cfg(feature = "off-chain")]
    pub fn double_digest(&self) -> [u8; 32] {
        crate::utils::keccak256(self.digest())
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Payload<'a>(pub(crate) &'a [u8]);

impl<'a> AsRef<[u8]> for Payload<'a> {
    fn as_ref(&self) -> &[u8] {
        self.0
    }
}

impl<'a> Payload<'a> {
    pub fn parse(span: &'a [u8]) -> Self {
        Self(span)
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn to_vec(&self) -> Vec<u8> {
        self.0.to_vec()
    }
}

impl<'a> From<&'a [u8]> for Payload<'a> {
    fn from(value: &'a [u8]) -> Self {
        Self::parse(value)
    }
}

impl<'a> From<Payload<'a>> for &'a [u8] {
    fn from(value: Payload<'a>) -> Self {
        value.0
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct GuardianSetSig<'a>(pub(crate) &'a [u8]);

impl<'a> AsRef<[u8]> for GuardianSetSig<'a> {
    fn as_ref(&self) -> &[u8] {
        self.0
    }
}

impl<'a> GuardianSetSig<'a> {
    pub fn guardian_index(&self) -> u8 {
        self.0[0]
    }

    pub fn r(&self) -> [u8; 32] {
        self.0[1..33].try_into().unwrap()
    }

    pub fn s(&self) -> [u8; 32] {
        self.0[33..65].try_into().unwrap()
    }

    pub fn rs(&self) -> [u8; 64] {
        self.0[1..65].try_into().unwrap()
    }

    pub fn v(&self) -> u8 {
        self.0[65]
    }

    pub fn signature(&self) -> [u8; 65] {
        self.0[1..].try_into().unwrap()
    }

    pub fn recovery_id(&self) -> u8 {
        self.0[65]
    }

    pub fn parse(span: &'a [u8]) -> Result<GuardianSetSig<'a>, &'static str> {
        if span.len() != 66 {
            return Err("expected exactly 66 bytes");
        }

        Ok(Self(span))
    }

    fn unchecked_from(span: &'a [u8]) -> GuardianSetSig<'a> {
        Self(span)
    }
}

impl<'a> TryFrom<&'a [u8]> for GuardianSetSig<'a> {
    type Error = &'static str;

    fn try_from(value: &'a [u8]) -> Result<Self, Self::Error> {
        Self::parse(value)
    }
}
