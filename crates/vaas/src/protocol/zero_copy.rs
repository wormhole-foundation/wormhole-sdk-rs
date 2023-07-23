#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct ZcGuardianSetSig<'a>(&'a [u8]);

impl<'a> TryFrom<&'a [u8]> for ZcGuardianSetSig<'a> {
    type Error = ();

    fn try_from(value: &'a [u8]) -> Result<Self, Self::Error> {
        Self::parse(value)
    }
}

impl ZcGuardianSetSig<'_> {
    pub fn guardian_set_index(&self) -> u8 {
        self.0[0]
    }

    pub fn r(&self) -> &[u8] {
        &self.0[1..33]
    }

    pub fn s(&self) -> &[u8] {
        &self.0[33..65]
    }

    pub fn rs(&self) -> &[u8] {
        &self.0[1..65]
    }

    pub fn v(&self) -> u8 {
        self.0[65]
    }

    pub fn parse<'a>(value: &'a [u8]) -> Result<ZcGuardianSetSig<'a>, ()> {
        // exact size
        if value.len() != 66 {
            return Err(());
        }

        Ok(ZcGuardianSetSig(value))
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct ZeroCopyVaaHeader<'a>(&'a [u8]);

impl<'a> TryFrom<&'a [u8]> for ZeroCopyVaaHeader<'a> {
    type Error = ();

    fn try_from(value: &'a [u8]) -> Result<Self, Self::Error> {
        Self::parse(value)
    }
}

impl ZeroCopyVaaHeader<'_> {
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

    pub fn signatures(&self) -> impl Iterator<Item = ZcGuardianSetSig<'_>> {
        self.raw_signatures().chunks(66).map(ZcGuardianSetSig)
    }

    pub fn parse<'a>(value: &'a [u8]) -> Result<ZeroCopyVaaHeader<'a>, ()> {
        // too short
        if value.len() < 6 {
            return Err(());
        }

        let expected_len = 6 + value[5] as usize * 66;

        // slice not long enough to contain all signatures
        if value.len() < expected_len {
            return Err(());
        }

        Ok(ZeroCopyVaaHeader(&value[..expected_len]))
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct ZeroCopyVaa<'a> {
    header: ZeroCopyVaaHeader<'a>,
    body: &'a [u8],
}

impl<'a> TryFrom<&'a [u8]> for ZeroCopyVaa<'a> {
    type Error = ();

    fn try_from(value: &'a [u8]) -> Result<Self, Self::Error> {
        Self::parse(value)
    }
}

impl ZeroCopyVaa<'_> {
    pub fn header(&self) -> ZeroCopyVaaHeader<'_> {
        self.header
    }

    pub fn payload(&self) -> &[u8] {
        self.payload()
    }

    pub fn parse<'a>(value: &'a [u8]) -> Result<ZeroCopyVaa<'a>, ()> {
        let header = ZeroCopyVaaHeader::parse(value)?;

        // too short
        if value.len() < header.0.len() {
            return Err(());
        }

        Ok(ZeroCopyVaa {
            header,
            body: &value[header.0.len()..],
        })
    }
}
