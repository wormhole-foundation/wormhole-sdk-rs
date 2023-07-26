use crate::Payload;

/// Token Bridge Governance payload, including type
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct TokenBridgeGovPayload<'a> {
    pub(crate) span: &'a [u8],

    decree: TokenBridgeDecree<'a>,
}

impl AsRef<[u8]> for TokenBridgeGovPayload<'_> {
    fn as_ref(&self) -> &[u8] {
        self.span
    }
}

impl<'a> TryFrom<Payload<'a>> for TokenBridgeGovPayload<'a> {
    type Error = &'static str;

    fn try_from(payload: Payload<'a>) -> Result<TokenBridgeGovPayload<'a>, &'static str> {
        TokenBridgeGovPayload::parse(payload.span)
    }
}

impl<'a> TokenBridgeGovPayload<'a> {
    pub fn span(&self) -> &[u8] {
        self.span
    }

    pub fn decree(&self) -> TokenBridgeDecree<'a> {
        self.decree
    }

    pub fn parse(span: &[u8]) -> Result<TokenBridgeGovPayload, &'static str> {
        if span.is_empty() {
            return Err("TokenBridgeGovPayload span too short. Need at least 1 byte");
        }

        let decree = TokenBridgeDecree::parse(span)?;

        Ok(TokenBridgeGovPayload { span, decree })
    }
}

/// The non-type-flag contents
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum TokenBridgeDecree<'a> {
    RegisterChain(RegisterChain<'a>),
    ContractUpgrade(ContractUpgrade<'a>),
    RecoverChainId(RecoverChainId<'a>),
}

impl AsRef<[u8]> for TokenBridgeDecree<'_> {
    fn as_ref(&self) -> &[u8] {
        match self {
            TokenBridgeDecree::RegisterChain(inner) => inner.as_ref(),
            TokenBridgeDecree::ContractUpgrade(inner) => inner.as_ref(),
            TokenBridgeDecree::RecoverChainId(inner) => inner.as_ref(),
        }
    }
}

impl<'a> TryFrom<&'a [u8]> for TokenBridgeDecree<'a> {
    type Error = &'static str;

    fn try_from(span: &'a [u8]) -> Result<TokenBridgeDecree<'a>, &'static str> {
        TokenBridgeDecree::parse(span)
    }
}

impl<'a> TokenBridgeDecree<'a> {
    pub fn parse(span: &'a [u8]) -> Result<TokenBridgeDecree<'a>, &'static str> {
        if span.is_empty() {
            return Err("TokenBridgeDecree span too short. Need at least 1 byte");
        }

        let decree = match span[0] {
            1 => TokenBridgeDecree::RegisterChain(TryFrom::try_from(&span[1..])?),
            2 => TokenBridgeDecree::ContractUpgrade(TryFrom::try_from(&span[1..])?),
            3 => TokenBridgeDecree::RecoverChainId(TryFrom::try_from(&span[1..])?),
            _ => {
                return Err("Invalid Token Bridge decree");
            }
        };

        Ok(decree)
    }
}

/// Register a new chain
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct RegisterChain<'a> {
    pub(crate) span: &'a [u8],
}

impl AsRef<[u8]> for RegisterChain<'_> {
    fn as_ref(&self) -> &[u8] {
        self.span
    }
}

impl<'a> TryFrom<&'a [u8]> for RegisterChain<'a> {
    type Error = &'static str;

    fn try_from(span: &'a [u8]) -> Result<RegisterChain<'a>, &'static str> {
        RegisterChain::parse(span)
    }
}

impl<'a> RegisterChain<'a> {
    pub fn foreign_chain(&self) -> u16 {
        u16::from_be_bytes(self.span[2..4].try_into().unwrap())
    }

    pub fn foreign_emitter(&self) -> [u8; 32] {
        self.span[4..36].try_into().unwrap()
    }

    pub fn parse(span: &'a [u8]) -> Result<RegisterChain<'a>, &'static str> {
        if span.len() < 36 {
            return Err("RegisterChain span too short. Need exactly 36 bytes");
        }

        if span.len() > 36 {
            return Err("RegisterChain span too long. Need exactly 36 bytes");
        }

        Ok(RegisterChain { span: &span[..36] })
    }
}

/// Upgrade a contract
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct ContractUpgrade<'a> {
    span: &'a [u8],
}

impl AsRef<[u8]> for ContractUpgrade<'_> {
    fn as_ref(&self) -> &[u8] {
        self.span
    }
}

impl<'a> TryFrom<&'a [u8]> for ContractUpgrade<'a> {
    type Error = &'static str;

    fn try_from(span: &'a [u8]) -> Result<ContractUpgrade<'a>, &'static str> {
        ContractUpgrade::parse(span)
    }
}

impl<'a> ContractUpgrade<'a> {
    pub fn chain(&self) -> u16 {
        u16::from_be_bytes(self.span[2..4].try_into().unwrap())
    }

    pub fn implementation(&self) -> [u8; 32] {
        self.span[4..36].try_into().unwrap()
    }

    pub fn parse(span: &'a [u8]) -> Result<ContractUpgrade<'a>, &'static str> {
        if span.len() < 36 {
            return Err("ContractUpgrade span too short. Need exactly 36 bytes");
        }

        if span.len() > 36 {
            return Err("ContractUpgrade span too long. Need exactly 36 bytes");
        }

        Ok(ContractUpgrade { span: &span[..36] })
    }
}

/// Recover a chain ID
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct RecoverChainId<'a> {
    span: &'a [u8],
}

impl AsRef<[u8]> for RecoverChainId<'_> {
    fn as_ref(&self) -> &[u8] {
        self.span
    }
}

impl<'a> TryFrom<&'a [u8]> for RecoverChainId<'a> {
    type Error = &'static str;

    fn try_from(span: &'a [u8]) -> Result<RecoverChainId<'a>, &'static str> {
        RecoverChainId::parse(span)
    }
}

impl<'a> RecoverChainId<'a> {
    pub fn recovered_chain(&self) -> u16 {
        u16::from_be_bytes(self.span[2..4].try_into().unwrap())
    }

    pub fn evm_chain_id(&self) -> [u8; 32] {
        self.span[4..36].try_into().unwrap()
    }

    pub fn new_chain(&self) -> u16 {
        u16::from_be_bytes(self.span[36..38].try_into().unwrap())
    }

    pub fn parse(span: &'a [u8]) -> Result<RecoverChainId<'a>, &'static str> {
        if span.len() < 38 {
            return Err("RecoverChainId span too short. Need exactly 38 bytes");
        }

        if span.len() > 38 {
            return Err("RecoverChainId span too long. Need exactly 38 bytes");
        }

        Ok(RecoverChainId { span: &span[..38] })
    }
}
