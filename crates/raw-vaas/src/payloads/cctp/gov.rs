use crate::Payload;

pub(crate) const GOV_MODULE: &[u8; 32] =
    b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00CircleIntegration";

/// Wormhole CCTP Governance payload, including type
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct WormholeCctpGovPayload<'a> {
    pub(crate) span: &'a [u8],

    decree: CircleIntegrationDecree<'a>,
}

impl<'a> AsRef<[u8]> for WormholeCctpGovPayload<'a> {
    fn as_ref(&self) -> &[u8] {
        self.span
    }
}

impl<'a> TryFrom<Payload<'a>> for WormholeCctpGovPayload<'a> {
    type Error = &'static str;

    fn try_from(payload: Payload<'a>) -> Result<Self, &'static str> {
        Self::parse(payload.0)
    }
}

impl<'a> WormholeCctpGovPayload<'a> {
    pub fn span(&self) -> &[u8] {
        self.span
    }

    pub fn decree(&self) -> CircleIntegrationDecree<'a> {
        self.decree
    }

    pub fn parse(span: &'a [u8]) -> Result<Self, &'static str> {
        if span.is_empty() {
            return Err("WormholeCctpGovPayload span too short. Need at least 1 byte");
        }

        if &span[..32] != GOV_MODULE {
            return Err("Invalid Wormhole CCTP governance message");
        }

        let decree = CircleIntegrationDecree::parse(&span[32..])?;

        Ok(Self { span, decree })
    }
}

/// The non-type-flag contents
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum CircleIntegrationDecree<'a> {
    UpdateWormholeFinality(UpdateWormholeFinality<'a>),
    RegisterEmitterAndDomain(RegisterEmitterAndDomain<'a>),
    ContractUpgrade(ContractUpgrade<'a>),
}

impl<'a> AsRef<[u8]> for CircleIntegrationDecree<'a> {
    fn as_ref(&self) -> &[u8] {
        match self {
            Self::UpdateWormholeFinality(inner) => inner.as_ref(),
            Self::RegisterEmitterAndDomain(inner) => inner.as_ref(),
            Self::ContractUpgrade(inner) => inner.as_ref(),
        }
    }
}

impl<'a> TryFrom<&'a [u8]> for CircleIntegrationDecree<'a> {
    type Error = &'static str;

    fn try_from(span: &'a [u8]) -> Result<Self, &'static str> {
        Self::parse(span)
    }
}

impl<'a> CircleIntegrationDecree<'a> {
    pub fn span(&self) -> &[u8] {
        self.as_ref()
    }

    pub fn update_wormhole_finality(&self) -> Option<&UpdateWormholeFinality> {
        match self {
            Self::UpdateWormholeFinality(inner) => Some(inner),
            _ => None,
        }
    }

    pub fn to_update_wormhole_finality_unchecked(self) -> UpdateWormholeFinality<'a> {
        match self {
            Self::UpdateWormholeFinality(inner) => inner,
            _ => panic!("CircleIntegrationDecree is not UpdateWormholeFinality"),
        }
    }

    pub fn register_emitter_and_domain(&self) -> Option<&RegisterEmitterAndDomain> {
        match self {
            Self::RegisterEmitterAndDomain(inner) => Some(inner),
            _ => None,
        }
    }

    pub fn to_register_emitter_and_domain_unchecked(self) -> RegisterEmitterAndDomain<'a> {
        match self {
            Self::RegisterEmitterAndDomain(inner) => inner,
            _ => panic!("CircleIntegrationDecree is not RegisterEmitterAndDomain"),
        }
    }

    pub fn contract_upgrade(&self) -> Option<&ContractUpgrade> {
        match self {
            Self::ContractUpgrade(inner) => Some(inner),
            _ => None,
        }
    }

    pub fn to_contract_upgrade_unchecked(self) -> ContractUpgrade<'a> {
        match self {
            Self::ContractUpgrade(inner) => inner,
            _ => panic!("CircleIntegrationDecree is not ContractUpgrade"),
        }
    }

    pub fn parse(span: &'a [u8]) -> Result<Self, &'static str> {
        if span.is_empty() {
            return Err("CircleIntegrationDecree span too short. Need at least 1 byte");
        }

        let decree = match span[0] {
            1 => Self::UpdateWormholeFinality(TryFrom::try_from(&span[1..])?),
            2 => Self::RegisterEmitterAndDomain(TryFrom::try_from(&span[1..])?),
            3 => Self::ContractUpgrade(TryFrom::try_from(&span[1..])?),
            _ => {
                return Err("Invalid Wormhole CCTP decree");
            }
        };

        Ok(decree)
    }
}

/// Update Wormhole finality
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct UpdateWormholeFinality<'a>(&'a [u8]);

impl AsRef<[u8]> for UpdateWormholeFinality<'_> {
    fn as_ref(&self) -> &[u8] {
        self.0
    }
}

impl<'a> TryFrom<&'a [u8]> for UpdateWormholeFinality<'a> {
    type Error = &'static str;

    fn try_from(span: &'a [u8]) -> Result<Self, &'static str> {
        Self::parse(span)
    }
}

impl<'a> UpdateWormholeFinality<'a> {
    pub fn chain(&self) -> u16 {
        u16::from_be_bytes(self.0[..2].try_into().unwrap())
    }

    pub fn finality(&self) -> u8 {
        self.0[2]
    }

    pub fn parse(span: &'a [u8]) -> Result<Self, &'static str> {
        if span.len() != 3 {
            return Err("UpdateWormholeFinality span too short. Need exactly 3 bytes");
        }

        Ok(Self(&span[..3]))
    }
}

/// Register emitter and CCTP domain
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct RegisterEmitterAndDomain<'a>(&'a [u8]);

impl AsRef<[u8]> for RegisterEmitterAndDomain<'_> {
    fn as_ref(&self) -> &[u8] {
        self.0
    }
}

impl<'a> TryFrom<&'a [u8]> for RegisterEmitterAndDomain<'a> {
    type Error = &'static str;

    fn try_from(span: &'a [u8]) -> Result<Self, &'static str> {
        Self::parse(span)
    }
}

impl<'a> RegisterEmitterAndDomain<'a> {
    pub fn chain(&self) -> u16 {
        u16::from_be_bytes(self.0[..2].try_into().unwrap())
    }

    pub fn foreign_chain(&self) -> u16 {
        u16::from_be_bytes(self.0[2..4].try_into().unwrap())
    }

    pub fn foreign_emitter(&self) -> [u8; 32] {
        self.0[4..36].try_into().unwrap()
    }

    pub fn cctp_domain(&self) -> u32 {
        u32::from_be_bytes(self.0[36..40].try_into().unwrap())
    }

    pub fn parse(span: &'a [u8]) -> Result<Self, &'static str> {
        if span.len() != 40 {
            return Err("RegisterEmitterAndDomain span too short. Need exactly 40 bytes");
        }

        Ok(Self(&span[..40]))
    }
}

/// Upgrade a contract
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct ContractUpgrade<'a>(&'a [u8]);

impl AsRef<[u8]> for ContractUpgrade<'_> {
    fn as_ref(&self) -> &[u8] {
        self.0
    }
}

impl<'a> TryFrom<&'a [u8]> for ContractUpgrade<'a> {
    type Error = &'static str;

    fn try_from(span: &'a [u8]) -> Result<Self, &'static str> {
        Self::parse(span)
    }
}

impl<'a> ContractUpgrade<'a> {
    pub fn chain(&self) -> u16 {
        u16::from_be_bytes(self.0[..2].try_into().unwrap())
    }

    pub fn implementation(&self) -> [u8; 32] {
        self.0[2..34].try_into().unwrap()
    }

    pub fn parse(span: &'a [u8]) -> Result<Self, &'static str> {
        if span.len() != 34 {
            return Err("ContractUpgrade span too short. Need exactly 34 bytes");
        }

        Ok(Self(&span[..34]))
    }
}

#[cfg(test)]
mod test {
    use crate::{cctp::WormholeCctpGovPayload, Vaa};
    use hex_literal::hex;

    #[test]
    fn register_emitter_and_domain() {
        let vaa = hex!("01000000030d021e9c5ce4c1bdc92c336ecbbecf89a0fb131929d27b9a33c09f8e178d6c5d76283041f1fc85c55e32928a7eb50d686755ec4ef9a82dc2791c273e7f377b2d3a9601039e1a105193623d01e576940c95731d1d0f7eaee22a8ca19558592678e6f1384867dc143bae9c568bfb64a1ec1f56bcec216249b554aa0348cb4ee377c451c09401069f5d7139cc4c7357b515665599e8ac1f5d575a0b2226dc8e67626cf083d435cf6d75a085d4edd3b8719f59347dd7b6c108699db92491a5445fcff7b3b4b7c51c010735f667168caaf9476ae37b248138b101bd71345915e59dc828b2cc3e24700089103e09f81d219842d1a2c44ca55022d159ecc5a93b5827d00f1cd89af4a90a7001080117d9b7979af3708c699b509de9da6448b36b1fee4ddea4d1adb3b1c695555f18aaace1144a0e476cf91bc653d6f3b01818c3fbb3252206ab58f664f9c7574900095bb7e00d0768644cbb94e024e3fdc6e5ec95eb50d128b8dc032dc3347048ddc32c5e3eeb711ff918b4e0db7e4919103d2c61aa01334ae0364961fc02d181bc60000a92158ddee2460eb5ee53788f5d0dd55faf1fb852ffbc19d559fc5e593d1c37f1186ba23802313b76b5ab11f2ddf89a6e3a86a24848ca7852e127b05ee09263fb010b1232c516ec1fc027eb3cbeb7d6ba03cc82aa1d821a4f52fc1766754ce9fd378c602cf8181ad87d13cd0b852720d47bc6cc0105442bfe72058f28789cc0a92050000cc762cf4aa57afbd54d402d9c50bdc7b79770957719ee10185ed5b32836527865457eac5da26e520ea86ebfc8c03791b06ac67c67d2b987ac70983b5d34d1ee31000d784892c6d46aaf856911758df845ce30f39cbea766891bc5216dfc53184f488e382232db90baff9d77cc0bf50f8804d11957849432bf2fcddd75622f6ca09936010e9abe122cb483bf8149062c0c4298b2ea18bca543417ebc13e8af7403305e87c158b618e33d9e2fb676ac39b56bda44d0d42e59795e35263629fbb132bfd8782c0110382756a859a73eac5c6b20bde2770142f35633a18edf3d3ed034198a5612b8286a8f58ad19eca8dd84d0701f041a9f36f82f773f65510ca4e592a7579c6e56ac0011f2dc9467887e0b09f7bca9c3ea5aac533f75176d769ef150a43fb13b0db9544d3787e88d4d9e8fd3eddca612d09d73c87c88abbca4054bad0ccf2c6d510f23ef0100000000260e63c900010000000000000000000000000000000000000000000000000000000000000004a6c87e6395f7e98a20000000000000000000000000000000436972636c65496e746567726174696f6e02000600170000000000000000000000002703483b1a5a7c577e8680de9df8be03c6f30e3c00000003");

        let raw_vaa = Vaa::parse(vaa.as_slice()).unwrap();
        assert_eq!(raw_vaa.version(), 1);
        assert_eq!(raw_vaa.guardian_set_index(), 3);
        assert_eq!(raw_vaa.signature_count(), 13);

        let body = raw_vaa.body();
        assert_eq!(body.timestamp(), 0);
        assert_eq!(body.nonce(), 638477257);
        assert_eq!(body.emitter_chain(), 1);

        let payload = WormholeCctpGovPayload::try_from(raw_vaa.payload())
            .unwrap()
            .decree();

        let registration = payload.register_emitter_and_domain().unwrap();

        assert_eq!(registration.foreign_chain(), 23);
        assert_eq!(
            registration.foreign_emitter(),
            hex!("0000000000000000000000002703483b1a5a7c577e8680de9df8be03c6f30e3c")
        );
        assert_eq!(registration.cctp_domain(), 3);
    }

    #[test]
    fn invalid_register_emitter_and_domain() {
        let vaa = hex!("01000000030d021e9c5ce4c1bdc92c336ecbbecf89a0fb131929d27b9a33c09f8e178d6c5d76283041f1fc85c55e32928a7eb50d686755ec4ef9a82dc2791c273e7f377b2d3a9601039e1a105193623d01e576940c95731d1d0f7eaee22a8ca19558592678e6f1384867dc143bae9c568bfb64a1ec1f56bcec216249b554aa0348cb4ee377c451c09401069f5d7139cc4c7357b515665599e8ac1f5d575a0b2226dc8e67626cf083d435cf6d75a085d4edd3b8719f59347dd7b6c108699db92491a5445fcff7b3b4b7c51c010735f667168caaf9476ae37b248138b101bd71345915e59dc828b2cc3e24700089103e09f81d219842d1a2c44ca55022d159ecc5a93b5827d00f1cd89af4a90a7001080117d9b7979af3708c699b509de9da6448b36b1fee4ddea4d1adb3b1c695555f18aaace1144a0e476cf91bc653d6f3b01818c3fbb3252206ab58f664f9c7574900095bb7e00d0768644cbb94e024e3fdc6e5ec95eb50d128b8dc032dc3347048ddc32c5e3eeb711ff918b4e0db7e4919103d2c61aa01334ae0364961fc02d181bc60000a92158ddee2460eb5ee53788f5d0dd55faf1fb852ffbc19d559fc5e593d1c37f1186ba23802313b76b5ab11f2ddf89a6e3a86a24848ca7852e127b05ee09263fb010b1232c516ec1fc027eb3cbeb7d6ba03cc82aa1d821a4f52fc1766754ce9fd378c602cf8181ad87d13cd0b852720d47bc6cc0105442bfe72058f28789cc0a92050000cc762cf4aa57afbd54d402d9c50bdc7b79770957719ee10185ed5b32836527865457eac5da26e520ea86ebfc8c03791b06ac67c67d2b987ac70983b5d34d1ee31000d784892c6d46aaf856911758df845ce30f39cbea766891bc5216dfc53184f488e382232db90baff9d77cc0bf50f8804d11957849432bf2fcddd75622f6ca09936010e9abe122cb483bf8149062c0c4298b2ea18bca543417ebc13e8af7403305e87c158b618e33d9e2fb676ac39b56bda44d0d42e59795e35263629fbb132bfd8782c0110382756a859a73eac5c6b20bde2770142f35633a18edf3d3ed034198a5612b8286a8f58ad19eca8dd84d0701f041a9f36f82f773f65510ca4e592a7579c6e56ac0011f2dc9467887e0b09f7bca9c3ea5aac533f75176d769ef150a43fb13b0db9544d3787e88d4d9e8fd3eddca612d09d73c87c88abbca4054bad0ccf2c6d510f23ef0100000000260e63c900010000000000000000000000000000000000000000000000000000000000000004a6c87e6395f7e98a20000000000000000000000000000000436972636c65496e746567726174696f6e02000600170000000000000000000000002703483b1a5a7c577e8680de9df8be03c6f30e3c0000000369");

        let raw_vaa = Vaa::parse(vaa.as_slice()).unwrap();
        assert_eq!(raw_vaa.version(), 1);
        assert_eq!(raw_vaa.guardian_set_index(), 3);
        assert_eq!(raw_vaa.signature_count(), 13);

        let body = raw_vaa.body();
        assert_eq!(body.timestamp(), 0);
        assert_eq!(body.nonce(), 638477257);
        assert_eq!(body.emitter_chain(), 1);

        let err = WormholeCctpGovPayload::try_from(raw_vaa.payload())
            .err()
            .unwrap();
        assert_eq!(
            err,
            "RegisterEmitterAndDomain span too short. Need exactly 40 bytes"
        );
    }

    // #[test]
    // fn contract_upgrade() {
    //     let vaa = hex!("01000000020100b57c401c985d2e4301685d42a86d1372117a27de8b0c12532d869a7d879599c675f11dae5c6b47e429c9802516fbf88f51bcb857c1a233ae24763f6a03df80410100bc614e0000000000010000000000000000000000000000000000000000000000000000000000000004000000000020035001000000000000000000000000000000000000000000546f6b656e427269646765020001485edcc94dd21decbbac52610a008c1bc5c8e4859c4504fff7433ad876cb1263");

    //     let raw_vaa = Vaa::parse(vaa.as_slice()).unwrap();
    //     assert_eq!(raw_vaa.version(), 1);
    //     assert_eq!(raw_vaa.guardian_set_index(), 2);
    //     assert_eq!(raw_vaa.signature_count(), 1);

    //     let body = raw_vaa.body();
    //     assert_eq!(body.timestamp(), 12345678);
    //     assert_eq!(body.nonce(), 0);
    //     assert_eq!(body.emitter_chain(), 1);

    //     let payload = WormholeCctpGovPayload::try_from(raw_vaa.payload())
    //         .unwrap()
    //         .decree();

    //     let contract_upgrade = payload.contract_upgrade().unwrap();

    //     assert_eq!(contract_upgrade.chain(), 1);
    //     assert_eq!(
    //         contract_upgrade.implementation(),
    //         hex!("485edcc94dd21decbbac52610a008c1bc5c8e4859c4504fff7433ad876cb1263")
    //     );
    // }

    // #[test]
    // fn invalid_contract_upgrade() {
    //     let vaa = hex!("01000000020100b57c401c985d2e4301685d42a86d1372117a27de8b0c12532d869a7d879599c675f11dae5c6b47e429c9802516fbf88f51bcb857c1a233ae24763f6a03df80410100bc614e0000000000010000000000000000000000000000000000000000000000000000000000000004000000000020035001000000000000000000000000000000000000000000546f6b656e427269646765020001485edcc94dd21decbbac52610a008c1bc5c8e4859c4504fff7433ad876cb126369");

    //     let raw_vaa = Vaa::parse(vaa.as_slice()).unwrap();
    //     assert_eq!(raw_vaa.version(), 1);
    //     assert_eq!(raw_vaa.guardian_set_index(), 2);
    //     assert_eq!(raw_vaa.signature_count(), 1);

    //     let body = raw_vaa.body();
    //     assert_eq!(body.timestamp(), 12345678);
    //     assert_eq!(body.nonce(), 0);
    //     assert_eq!(body.emitter_chain(), 1);

    //     let err = WormholeCctpGovPayload::try_from(raw_vaa.payload())
    //         .err()
    //         .unwrap();
    //     assert_eq!(err, "ContractUpgrade span too short. Need exactly 34 bytes");
    // }

    #[test]
    fn invalid_wormhole_cctp_gov() {
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

        let err = WormholeCctpGovPayload::try_from(raw_vaa.payload())
            .err()
            .unwrap();
        assert_eq!(err, "Invalid Wormhole CCTP governance message");
    }
}
