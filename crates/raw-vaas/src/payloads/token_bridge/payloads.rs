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
        if span.len() < 1 {
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
        if span.len() < 1 {
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

/// A token transfer payload
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

/// An attestation payload
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

/// A token transfer payload with a message
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
