use crate::Payload;

pub(crate) const GOV_MODULE: &[u8; 32] = b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00TokenBridge";

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

        if &span[..32] != GOV_MODULE {
            return Err("Invalid Token Bridge governance message");
        }

        let decree = TokenBridgeDecree::parse(&span[32..])?;

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
    pub fn span(&self) -> &[u8] {
        self.as_ref()
    }

    pub fn register_chain(&self) -> Option<&RegisterChain> {
        match self {
            TokenBridgeDecree::RegisterChain(inner) => Some(inner),
            _ => None,
        }
    }

    pub fn contract_upgrade(&self) -> Option<&ContractUpgrade> {
        match self {
            TokenBridgeDecree::ContractUpgrade(inner) => Some(inner),
            _ => None,
        }
    }

    pub fn recover_chain_id(&self) -> Option<&RecoverChainId> {
        match self {
            TokenBridgeDecree::RecoverChainId(inner) => Some(inner),
            _ => None,
        }
    }

    pub fn parse(span: &'a [u8]) -> Result<Self, &'static str> {
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
        if span.len() != 36 {
            return Err("RegisterChain span too short. Need exactly 36 bytes");
        }

        if span[..2] != [0, 0] {
            return Err("RegisterChain target chain must be 0");
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
        u16::from_be_bytes(self.span[..2].try_into().unwrap())
    }

    pub fn implementation(&self) -> [u8; 32] {
        self.span[2..34].try_into().unwrap()
    }

    pub fn parse(span: &'a [u8]) -> Result<ContractUpgrade<'a>, &'static str> {
        if span.len() != 34 {
            return Err("ContractUpgrade span too short. Need exactly 34 bytes");
        }

        Ok(ContractUpgrade { span: &span[..34] })
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
        u16::from_be_bytes(self.span[..2].try_into().unwrap())
    }

    pub fn evm_chain_id(&self) -> [u8; 32] {
        self.span[2..34].try_into().unwrap()
    }

    pub fn new_chain(&self) -> u16 {
        u16::from_be_bytes(self.span[34..36].try_into().unwrap())
    }

    pub fn parse(span: &'a [u8]) -> Result<RecoverChainId<'a>, &'static str> {
        if span.len() != 36 {
            return Err("RecoverChainId span too short. Need exactly 36 bytes");
        }

        Ok(RecoverChainId { span: &span[..36] })
    }
}

#[cfg(test)]
mod test {
    use crate::{token_bridge::TokenBridgeGovPayload, Vaa};
    use hex_literal::hex;

    #[test]
    fn register_chain() {
        let vaa = hex!("010000000201002424a14044fa5538a5572c519e3b969a716fdf09d9129db2139ba1c3dca9767a53474fb37928e0a0d71c075d8e430d606347a95d4296bade3f6c52e64c4bf7d30100bc614e000000000001000000000000000000000000000000000000000000000000000000000000000400000000001eab9001000000000000000000000000000000000000000000546f6b656e42726964676501000000020000000000000000000000003ee18b2214aff97000d974cf647e7c347e8fa585");

        let raw_vaa = Vaa::parse(vaa.as_slice()).unwrap();
        assert_eq!(raw_vaa.version(), 1);
        assert_eq!(raw_vaa.guardian_set_index(), 2);
        assert_eq!(raw_vaa.signature_count(), 1);

        let body = raw_vaa.body();
        assert_eq!(body.timestamp(), 12345678);
        assert_eq!(body.nonce(), 0);
        assert_eq!(body.emitter_chain(), 1);

        let payload = TokenBridgeGovPayload::try_from(raw_vaa.payload())
            .unwrap()
            .decree();

        let register_chain = payload.register_chain().unwrap();

        assert_eq!(register_chain.foreign_chain(), 2);
        assert_eq!(
            register_chain.foreign_emitter(),
            hex!("0000000000000000000000003ee18b2214aff97000d974cf647e7c347e8fa585")
        );
    }

    #[test]
    fn invalid_register_chain() {
        let vaa = hex!("010000000201002424a14044fa5538a5572c519e3b969a716fdf09d9129db2139ba1c3dca9767a53474fb37928e0a0d71c075d8e430d606347a95d4296bade3f6c52e64c4bf7d30100bc614e000000000001000000000000000000000000000000000000000000000000000000000000000400000000001eab9001000000000000000000000000000000000000000000546f6b656e42726964676501000000020000000000000000000000003ee18b2214aff97000d974cf647e7c347e8fa58569");

        let raw_vaa = Vaa::parse(vaa.as_slice()).unwrap();
        assert_eq!(raw_vaa.version(), 1);
        assert_eq!(raw_vaa.guardian_set_index(), 2);
        assert_eq!(raw_vaa.signature_count(), 1);

        let body = raw_vaa.body();
        assert_eq!(body.timestamp(), 12345678);
        assert_eq!(body.nonce(), 0);
        assert_eq!(body.emitter_chain(), 1);

        let err = TokenBridgeGovPayload::try_from(raw_vaa.payload())
            .err()
            .unwrap();
        assert_eq!(err, "RegisterChain span too short. Need exactly 36 bytes");
    }

    #[test]
    fn contract_upgrade() {
        let vaa = hex!("01000000020100b57c401c985d2e4301685d42a86d1372117a27de8b0c12532d869a7d879599c675f11dae5c6b47e429c9802516fbf88f51bcb857c1a233ae24763f6a03df80410100bc614e0000000000010000000000000000000000000000000000000000000000000000000000000004000000000020035001000000000000000000000000000000000000000000546f6b656e427269646765020001485edcc94dd21decbbac52610a008c1bc5c8e4859c4504fff7433ad876cb1263");

        let raw_vaa = Vaa::parse(vaa.as_slice()).unwrap();
        assert_eq!(raw_vaa.version(), 1);
        assert_eq!(raw_vaa.guardian_set_index(), 2);
        assert_eq!(raw_vaa.signature_count(), 1);

        let body = raw_vaa.body();
        assert_eq!(body.timestamp(), 12345678);
        assert_eq!(body.nonce(), 0);
        assert_eq!(body.emitter_chain(), 1);

        let payload = TokenBridgeGovPayload::try_from(raw_vaa.payload())
            .unwrap()
            .decree();

        let contract_upgrade = payload.contract_upgrade().unwrap();

        assert_eq!(contract_upgrade.chain(), 1);
        assert_eq!(
            contract_upgrade.implementation(),
            hex!("485edcc94dd21decbbac52610a008c1bc5c8e4859c4504fff7433ad876cb1263")
        );
    }

    #[test]
    fn invalid_contract_upgrade() {
        let vaa = hex!("01000000020100b57c401c985d2e4301685d42a86d1372117a27de8b0c12532d869a7d879599c675f11dae5c6b47e429c9802516fbf88f51bcb857c1a233ae24763f6a03df80410100bc614e0000000000010000000000000000000000000000000000000000000000000000000000000004000000000020035001000000000000000000000000000000000000000000546f6b656e427269646765020001485edcc94dd21decbbac52610a008c1bc5c8e4859c4504fff7433ad876cb126369");

        let raw_vaa = Vaa::parse(vaa.as_slice()).unwrap();
        assert_eq!(raw_vaa.version(), 1);
        assert_eq!(raw_vaa.guardian_set_index(), 2);
        assert_eq!(raw_vaa.signature_count(), 1);

        let body = raw_vaa.body();
        assert_eq!(body.timestamp(), 12345678);
        assert_eq!(body.nonce(), 0);
        assert_eq!(body.emitter_chain(), 1);

        let err = TokenBridgeGovPayload::try_from(raw_vaa.payload())
            .err()
            .unwrap();
        assert_eq!(err, "ContractUpgrade span too short. Need exactly 34 bytes");
    }

    #[test]
    fn invalid_token_bridge_gov() {
        let vaa = hex!("0100000002010005e1bb5901b0a78951ecec430994383f9ad0e0f767c21a67a826078bf11ece0c39381fa81bfa20a6ed2ea2362a6c0d9459778f5e8bf8e949a58e23b59718f5690000bc614e0000000000010000000000000000000000000000000000000000000000000000000000000004000000000010c1110100000000000000000000000000000000000000000000000000000000436f7265010001dd33db6e624f8354d2168a9b3e04a6e04602d2f658edaa11403dc1b61b46efc5");

        let raw_vaa = Vaa::parse(vaa.as_slice()).unwrap();
        assert_eq!(raw_vaa.version(), 1);
        assert_eq!(raw_vaa.guardian_set_index(), 2);
        assert_eq!(raw_vaa.signature_count(), 1);

        let body = raw_vaa.body();
        assert_eq!(body.timestamp(), 12345678);
        assert_eq!(body.nonce(), 0);
        assert_eq!(body.emitter_chain(), 1);

        let payload = raw_vaa.payload();
        let module = &payload.as_ref()[..32];
        assert_ne!(module, super::GOV_MODULE);

        let err = TokenBridgeGovPayload::try_from(raw_vaa.payload())
            .err()
            .unwrap();
        assert_eq!(err, "Invalid Token Bridge governance message");
    }
}
