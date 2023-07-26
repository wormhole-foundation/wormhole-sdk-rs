use std::borrow::Cow;

use crate::Payload;

/// A token bridge payload, with type flag
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct TokenBridgePayload<'a> {
    span: &'a [u8],

    message: TokenBridgeMessage<'a>,
}

impl AsRef<[u8]> for TokenBridgePayload<'_> {
    fn as_ref(&self) -> &[u8] {
        self.span
    }
}

impl<'a> TryFrom<Payload<'a>> for TokenBridgePayload<'a> {
    type Error = &'static str;

    fn try_from(payload: Payload<'a>) -> Result<TokenBridgePayload<'a>, &'static str> {
        TokenBridgePayload::parse(payload.span)
    }
}

impl<'a> TokenBridgePayload<'a> {
    pub fn span(&self) -> &[u8] {
        self.span
    }

    pub fn message(&self) -> &TokenBridgeMessage<'a> {
        &self.message
    }

    pub fn parse(span: &'a [u8]) -> Result<TokenBridgePayload<'a>, &'static str> {
        if span.is_empty() {
            return Err("TokenBridgePayload span too short. Need at least 1 byte");
        }

        let message = TokenBridgeMessage::parse(span)?;

        Ok(TokenBridgePayload { span, message })
    }
}

/// The non-type-flag contents
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum TokenBridgeMessage<'a> {
    Transfer(Transfer<'a>),
    Attestation(Attestation<'a>),
    TransferWithMessage(TransferWithMessage<'a>),
}

impl<'a> TryFrom<Payload<'a>> for TokenBridgeMessage<'a> {
    type Error = &'static str;

    fn try_from(payload: Payload<'a>) -> Result<TokenBridgeMessage<'a>, &'static str> {
        TokenBridgeMessage::parse(payload.span)
    }
}

impl AsRef<[u8]> for TokenBridgeMessage<'_> {
    fn as_ref(&self) -> &[u8] {
        match self {
            TokenBridgeMessage::Transfer(inner) => inner.as_ref(),
            TokenBridgeMessage::Attestation(inner) => inner.as_ref(),
            TokenBridgeMessage::TransferWithMessage(inner) => inner.as_ref(),
        }
    }
}

impl<'a> TokenBridgeMessage<'a> {
    pub fn span(&self) -> &[u8] {
        self.as_ref()
    }

    pub fn transfer(&self) -> Option<&Transfer> {
        match self {
            TokenBridgeMessage::Transfer(inner) => Some(inner),
            _ => None,
        }
    }

    pub fn attestation(&self) -> Option<&Attestation> {
        match self {
            TokenBridgeMessage::Attestation(inner) => Some(inner),
            _ => None,
        }
    }

    pub fn transfer_with_message(&self) -> Option<&TransferWithMessage> {
        match self {
            TokenBridgeMessage::TransferWithMessage(inner) => Some(inner),
            _ => None,
        }
    }

    pub fn parse(span: &'a [u8]) -> Result<Self, &'static str> {
        if span.is_empty() {
            return Err("TokenBridgeMessage span too short. Need at least 1 byte");
        }

        match span[0] {
            1 => Ok(TokenBridgeMessage::Transfer(Transfer::parse(&span[1..])?)),
            2 => Ok(TokenBridgeMessage::Attestation(Attestation::parse(
                &span[1..],
            )?)),
            3 => Ok(TokenBridgeMessage::TransferWithMessage(
                TransferWithMessage::parse(&span[1..])?,
            )),
            _ => Err("Unknown TokenBridgeMessage type"),
        }
    }
}

/// A token bridge transfer
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Transfer<'a> {
    span: &'a [u8],
}

impl AsRef<[u8]> for Transfer<'_> {
    fn as_ref(&self) -> &[u8] {
        self.span
    }
}

impl<'a> Transfer<'a> {
    pub fn amount(&self) -> [u8; 32] {
        self.span[..32].try_into().unwrap()
    }

    pub fn token_address(&self) -> [u8; 32] {
        self.span[32..64].try_into().unwrap()
    }

    pub fn token_chain(&self) -> u16 {
        u16::from_be_bytes(self.span[64..66].try_into().unwrap())
    }

    pub fn recipient(&self) -> [u8; 32] {
        self.span[66..98].try_into().unwrap()
    }

    pub fn recipient_chain(&self) -> u16 {
        u16::from_be_bytes(self.span[98..100].try_into().unwrap())
    }

    pub fn relayer_fee(&self) -> [u8; 32] {
        self.span[100..132].try_into().unwrap()
    }

    pub fn parse(span: &'a [u8]) -> Result<Transfer<'a>, &'static str> {
        if span.len() < 132 {
            return Err("Transfer span too short. Need exactly 132 bytes");
        }

        Ok(Transfer { span: &span[..132] })
    }
}

/// A token bridge attestation
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Attestation<'a> {
    span: &'a [u8],
}

impl AsRef<[u8]> for Attestation<'_> {
    fn as_ref(&self) -> &[u8] {
        self.span
    }
}

impl<'a> TryFrom<&'a [u8]> for Attestation<'a> {
    type Error = &'static str;

    fn try_from(span: &'a [u8]) -> Result<Attestation<'a>, &'static str> {
        Attestation::parse(span)
    }
}

impl<'a> Attestation<'a> {
    pub fn token_address(&self) -> [u8; 32] {
        self.span[..32].try_into().unwrap()
    }

    pub fn token_chain(&self) -> u16 {
        u16::from_be_bytes(self.span[32..34].try_into().unwrap())
    }

    pub fn decimals(&self) -> u8 {
        self.span[34]
    }

    pub fn symbol(&self) -> Cow<'a, str> {
        let idx = &self.span[35..67]
            .iter()
            .rposition(|x| *x != 0)
            .map(|i| i + 1)
            .unwrap_or_default();
        String::from_utf8_lossy(&self.span[35..35 + idx])
    }

    pub fn name(&self) -> Cow<'a, str> {
        let idx = &self.span[67..99]
            .iter()
            .rposition(|x| *x != 0)
            .map(|i| i + 1)
            .unwrap_or_default();
        String::from_utf8_lossy(&self.span[67..67 + idx])
    }

    pub fn parse(span: &'a [u8]) -> Result<Attestation<'a>, &'static str> {
        if span.len() < 99 {
            return Err("Attestation span too short. Need exactly 99 bytes");
        }

        Ok(Attestation { span: &span[..99] })
    }
}

/// A token bridge transfer with message
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct TransferWithMessage<'a> {
    span: &'a [u8],
}

impl AsRef<[u8]> for TransferWithMessage<'_> {
    fn as_ref(&self) -> &[u8] {
        self.span
    }
}

impl<'a> TryFrom<&'a [u8]> for TransferWithMessage<'a> {
    type Error = &'static str;

    fn try_from(span: &'a [u8]) -> Result<TransferWithMessage<'a>, &'static str> {
        TransferWithMessage::parse(span)
    }
}

impl<'a> TransferWithMessage<'a> {
    pub fn amount(&self) -> [u8; 32] {
        self.span[..32].try_into().unwrap()
    }

    pub fn token_address(&self) -> [u8; 32] {
        self.span[32..64].try_into().unwrap()
    }

    pub fn token_chain(&self) -> u16 {
        u16::from_be_bytes(self.span[64..66].try_into().unwrap())
    }

    pub fn redeemer(&self) -> &[u8] {
        &self.span[66..98]
    }

    pub fn redeemer_chain(&self) -> u16 {
        u16::from_be_bytes(self.span[98..100].try_into().unwrap())
    }

    pub fn sender(&self) -> [u8; 32] {
        self.span[100..132].try_into().unwrap()
    }

    pub fn payload(&self) -> &[u8] {
        &self.span[132..]
    }

    pub fn parse(span: &'a [u8]) -> Result<TransferWithMessage<'a>, &'static str> {
        if span.len() < 132 {
            return Err("TransferWithMessage span too short. Need at least 132 bytes");
        }

        Ok(TransferWithMessage { span })
    }
}

#[cfg(test)]
mod test {
    use crate::{token_bridge::TokenBridgePayload, Vaa};

    #[test]
    fn basic_test() {
        //01000000000100ff7edcd3facb7dd6e06e0bd3e178cfddd775208f3e09f0b68bba981b812258716e6e5cd42c0ba413586df1e4066e29a1a41f9a49ae05a58f5fa93590d165abf100000000007ce2ea3f000195f83a27e90c622a98c037353f271fd8f5f57b4dc18ebf5ff75a934724bd0491a43a1c0020f88a3e2002000000000000000000000000c02aaa39b223fe8d0a0e5c4f27ead9083c756cc200021257455448000000000000000000000000000000000000000000000000000000005772617070656420657468657200000000000000000000000000000000000000
        let vaa = [
            0x1, 0x0, 0x0, 0x0, 0x0, 0x1, 0x0, 0xff, 0x7e, 0xdc, 0xd3, 0xfa, 0xcb, 0x7d, 0xd6,
            0xe0, 0x6e, 0xb, 0xd3, 0xe1, 0x78, 0xcf, 0xdd, 0xd7, 0x75, 0x20, 0x8f, 0x3e, 0x9, 0xf0,
            0xb6, 0x8b, 0xba, 0x98, 0x1b, 0x81, 0x22, 0x58, 0x71, 0x6e, 0x6e, 0x5c, 0xd4, 0x2c,
            0xb, 0xa4, 0x13, 0x58, 0x6d, 0xf1, 0xe4, 0x6, 0x6e, 0x29, 0xa1, 0xa4, 0x1f, 0x9a, 0x49,
            0xae, 0x5, 0xa5, 0x8f, 0x5f, 0xa9, 0x35, 0x90, 0xd1, 0x65, 0xab, 0xf1, 0x0, 0x0, 0x0,
            0x0, 0x0, 0x7c, 0xe2, 0xea, 0x3f, 0x0, 0x1, 0x95, 0xf8, 0x3a, 0x27, 0xe9, 0xc, 0x62,
            0x2a, 0x98, 0xc0, 0x37, 0x35, 0x3f, 0x27, 0x1f, 0xd8, 0xf5, 0xf5, 0x7b, 0x4d, 0xc1,
            0x8e, 0xbf, 0x5f, 0xf7, 0x5a, 0x93, 0x47, 0x24, 0xbd, 0x4, 0x91, 0xa4, 0x3a, 0x1c, 0x0,
            0x20, 0xf8, 0x8a, 0x3e, 0x20, 0x2, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
            0x0, 0x0, 0xc0, 0x2a, 0xaa, 0x39, 0xb2, 0x23, 0xfe, 0x8d, 0xa, 0xe, 0x5c, 0x4f, 0x27,
            0xea, 0xd9, 0x8, 0x3c, 0x75, 0x6c, 0xc2, 0x0, 0x2, 0x12, 0x57, 0x45, 0x54, 0x48, 0x0,
            0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
            0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x57, 0x72, 0x61, 0x70, 0x70, 0x65,
            0x64, 0x20, 0x65, 0x74, 0x68, 0x65, 0x72, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
            0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
        ];

        let raw_vaa = Vaa::parse(vaa.as_slice()).unwrap();
        assert_eq!(raw_vaa.version(), 1);
        assert_eq!(raw_vaa.guardian_set_index(), 0);
        assert_eq!(raw_vaa.signature_count(), 1);

        let body = raw_vaa.body();
        assert_eq!(body.timestamp(), 0);
        assert_eq!(body.nonce(), 2095245887);
        assert_eq!(body.emitter_chain(), 1);

        let payload = TokenBridgePayload::try_from(raw_vaa.payload()).unwrap();

        let attestation = payload.message().attestation().unwrap();

        assert_eq!(attestation.token_chain(), 2);
        assert_eq!(attestation.decimals(), 18);
        assert_eq!(attestation.symbol(), "WETH");
        assert_eq!(attestation.name(), "Wrapped ether");
    }
}
