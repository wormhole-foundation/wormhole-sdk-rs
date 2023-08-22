use crate::Payload;

pub(crate) const GOV_MODULE: &[u8; 32] = b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00Core";

/// Core Bridge Governance payload, including type
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct CoreBridgeGovPayload<'a> {
    pub(crate) span: &'a [u8],

    decree: CoreBridgeDecree<'a>,
}

impl AsRef<[u8]> for CoreBridgeGovPayload<'_> {
    fn as_ref(&self) -> &[u8] {
        self.span
    }
}

impl<'a> TryFrom<Payload<'a>> for CoreBridgeGovPayload<'a> {
    type Error = &'static str;

    fn try_from(payload: Payload<'a>) -> Result<CoreBridgeGovPayload<'a>, &'static str> {
        CoreBridgeGovPayload::parse(payload.span)
    }
}

impl<'a> CoreBridgeGovPayload<'a> {
    pub fn span(&self) -> &[u8] {
        self.span
    }

    pub fn decree(&self) -> CoreBridgeDecree<'a> {
        self.decree
    }

    pub fn parse(span: &[u8]) -> Result<CoreBridgeGovPayload, &'static str> {
        if span.is_empty() {
            return Err("CoreBridgeGovPayload span too short. Need at least 1 byte");
        }

        if &span[..32] != GOV_MODULE {
            return Err("Invalid Core Bridge governance message");
        }

        let decree = CoreBridgeDecree::parse(&span[32..])?;

        Ok(CoreBridgeGovPayload { span, decree })
    }
}

/// The non-type-flag contents
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum CoreBridgeDecree<'a> {
    ContractUpgrade(ContractUpgrade<'a>),
    GuardianSetUpdate(GuardianSetUpdate<'a>),
    SetMessageFee(SetMessageFee<'a>),
    TransferFees(TransferFees<'a>),
    RecoverChainId(RecoverChainId<'a>),
}

impl AsRef<[u8]> for CoreBridgeDecree<'_> {
    fn as_ref(&self) -> &[u8] {
        match self {
            CoreBridgeDecree::ContractUpgrade(inner) => inner.as_ref(),
            CoreBridgeDecree::GuardianSetUpdate(inner) => inner.as_ref(),
            CoreBridgeDecree::SetMessageFee(inner) => inner.as_ref(),
            CoreBridgeDecree::TransferFees(inner) => inner.as_ref(),
            CoreBridgeDecree::RecoverChainId(inner) => inner.as_ref(),
        }
    }
}

impl<'a> TryFrom<&'a [u8]> for CoreBridgeDecree<'a> {
    type Error = &'static str;

    fn try_from(span: &'a [u8]) -> Result<CoreBridgeDecree<'a>, &'static str> {
        CoreBridgeDecree::parse(span)
    }
}

impl<'a> CoreBridgeDecree<'a> {
    pub fn span(&self) -> &[u8] {
        self.as_ref()
    }

    pub fn contract_upgrade(&self) -> Option<&ContractUpgrade> {
        match self {
            CoreBridgeDecree::ContractUpgrade(inner) => Some(inner),
            _ => None,
        }
    }

    pub fn guardian_set_update(&self) -> Option<&GuardianSetUpdate> {
        match self {
            CoreBridgeDecree::GuardianSetUpdate(inner) => Some(inner),
            _ => None,
        }
    }

    pub fn set_message_fee(&self) -> Option<&SetMessageFee> {
        match self {
            CoreBridgeDecree::SetMessageFee(inner) => Some(inner),
            _ => None,
        }
    }

    pub fn transfer_fees(&self) -> Option<&TransferFees> {
        match self {
            CoreBridgeDecree::TransferFees(inner) => Some(inner),
            _ => None,
        }
    }

    pub fn recover_chain_id(&self) -> Option<&RecoverChainId> {
        match self {
            CoreBridgeDecree::RecoverChainId(inner) => Some(inner),
            _ => None,
        }
    }

    pub fn parse(span: &'a [u8]) -> Result<Self, &'static str> {
        if span.is_empty() {
            return Err("CoreBridgeDecree span too short. Need at least 1 byte");
        }

        let decree = match span[0] {
            1 => CoreBridgeDecree::ContractUpgrade(TryFrom::try_from(&span[1..])?),
            2 => CoreBridgeDecree::GuardianSetUpdate(TryFrom::try_from(&span[1..])?),
            3 => CoreBridgeDecree::SetMessageFee(TryFrom::try_from(&span[1..])?),
            4 => CoreBridgeDecree::TransferFees(TryFrom::try_from(&span[1..])?),
            5 => CoreBridgeDecree::RecoverChainId(TryFrom::try_from(&span[1..])?),
            _ => {
                return Err("Invalid Core Bridge decree");
            }
        };

        Ok(decree)
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

/// Update guardian set
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct GuardianSetUpdate<'a> {
    span: &'a [u8],
}

impl AsRef<[u8]> for GuardianSetUpdate<'_> {
    fn as_ref(&self) -> &[u8] {
        self.span
    }
}

impl<'a> TryFrom<&'a [u8]> for GuardianSetUpdate<'a> {
    type Error = &'static str;

    fn try_from(span: &'a [u8]) -> Result<GuardianSetUpdate<'a>, &'static str> {
        GuardianSetUpdate::parse(span)
    }
}

impl<'a> GuardianSetUpdate<'a> {
    pub fn new_index(&self) -> u32 {
        u32::from_be_bytes(self.span[2..6].try_into().unwrap())
    }

    pub fn num_guardians(&self) -> u8 {
        self.span[6]
    }

    pub fn try_guardian_at(&self, i: usize) -> Result<[u8; 20], &'static str> {
        if i >= usize::from(self.num_guardians()) {
            return Err("Exceeds number of encoded guardians");
        }

        Ok(self.guardian_at(i))
    }

    pub fn guardian_at(&self, i: usize) -> [u8; 20] {
        self.span[(7 + i * 20)..(7 + (i + 1) * 20)]
            .try_into()
            .unwrap()
    }

    pub fn parse(span: &'a [u8]) -> Result<GuardianSetUpdate<'a>, &'static str> {
        if span.len() < 27 {
            return Err("GuardianSetUpdate span too short. Need at least 27 bytes (for at least 1 guardian)");
        }

        let expected_len = 7 + usize::from(span[6]) * 20;
        if span.len() != expected_len {
            return Err(
                "GuardianSetUpdate span too short. Need exactly 7 + num_guardians * 20 bytes",
            );
        }

        Ok(GuardianSetUpdate {
            span: &span[..expected_len],
        })
    }
}

/// Set the message fee
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct SetMessageFee<'a> {
    span: &'a [u8],
}

impl AsRef<[u8]> for SetMessageFee<'_> {
    fn as_ref(&self) -> &[u8] {
        self.span
    }
}

impl<'a> TryFrom<&'a [u8]> for SetMessageFee<'a> {
    type Error = &'static str;

    fn try_from(span: &'a [u8]) -> Result<SetMessageFee<'a>, &'static str> {
        SetMessageFee::parse(span)
    }
}

impl<'a> SetMessageFee<'a> {
    pub fn chain(&self) -> u16 {
        u16::from_be_bytes(self.span[..2].try_into().unwrap())
    }

    pub fn fee(&self) -> [u8; 32] {
        self.span[2..34].try_into().unwrap()
    }

    pub fn parse(span: &'a [u8]) -> Result<SetMessageFee<'a>, &'static str> {
        if span.len() != 34 {
            return Err("SetMessageFee span too short. Need exactly 34 bytes");
        }

        Ok(SetMessageFee { span: &span[..34] })
    }
}

/// Transfer fees to someone
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct TransferFees<'a> {
    span: &'a [u8],
}

impl AsRef<[u8]> for TransferFees<'_> {
    fn as_ref(&self) -> &[u8] {
        self.span
    }
}

impl<'a> TryFrom<&'a [u8]> for TransferFees<'a> {
    type Error = &'static str;

    fn try_from(span: &'a [u8]) -> Result<TransferFees<'a>, &'static str> {
        TransferFees::parse(span)
    }
}

impl<'a> TransferFees<'a> {
    pub fn chain(&self) -> u16 {
        u16::from_be_bytes(self.span[..2].try_into().unwrap())
    }

    pub fn amount(&self) -> [u8; 32] {
        self.span[2..34].try_into().unwrap()
    }

    pub fn recipient(&self) -> [u8; 32] {
        self.span[34..66].try_into().unwrap()
    }

    pub fn parse(span: &'a [u8]) -> Result<TransferFees<'a>, &'static str> {
        if span.len() != 66 {
            return Err("TransferFees span too short. Need exactly 66 bytes");
        }

        Ok(TransferFees { span: &span[..66] })
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
    use crate::{core::CoreBridgeGovPayload, Vaa};
    use hex_literal::hex;

    #[test]
    fn contract_upgrade() {
        let vaa = hex!("01000000020100077a563ab1e788609439fe527229852601665ae086ffa358cd1f9495fd85dcfe52ea11a87ef7fe6f005264561f07291ccf89b73347b4ab568fef468c4487d0910000bc614e0000000000010000000000000000000000000000000000000000000000000000000000000004000000000010c1100100000000000000000000000000000000000000000000000000000000436f72650100015cdecd10d40cee2b1601e3adc3bbe914e3625925d0c9bd7f7ba0c153465978e7");

        let raw_vaa = Vaa::parse(vaa.as_slice()).unwrap();
        assert_eq!(raw_vaa.version(), 1);
        assert_eq!(raw_vaa.guardian_set_index(), 2);
        assert_eq!(raw_vaa.signature_count(), 1);

        let body = raw_vaa.body();
        assert_eq!(body.timestamp(), 12345678);
        assert_eq!(body.nonce(), 0);
        assert_eq!(body.emitter_chain(), 1);

        let payload = CoreBridgeGovPayload::try_from(raw_vaa.payload())
            .unwrap()
            .decree();

        let contract_upgrade = payload.contract_upgrade().unwrap();

        assert_eq!(contract_upgrade.chain(), 1);
        assert_eq!(
            contract_upgrade.implementation(),
            hex!("5cdecd10d40cee2b1601e3adc3bbe914e3625925d0c9bd7f7ba0c153465978e7")
        );
    }

    #[test]
    fn invalid_contract_upgrade() {
        let vaa = hex!("01000000020100077a563ab1e788609439fe527229852601665ae086ffa358cd1f9495fd85dcfe52ea11a87ef7fe6f005264561f07291ccf89b73347b4ab568fef468c4487d0910000bc614e0000000000010000000000000000000000000000000000000000000000000000000000000004000000000010c1100100000000000000000000000000000000000000000000000000000000436f72650100015cdecd10d40cee2b1601e3adc3bbe914e3625925d0c9bd7f7ba0c153465978e769");

        let raw_vaa = Vaa::parse(vaa.as_slice()).unwrap();
        assert_eq!(raw_vaa.version(), 1);
        assert_eq!(raw_vaa.guardian_set_index(), 2);
        assert_eq!(raw_vaa.signature_count(), 1);

        let body = raw_vaa.body();
        assert_eq!(body.timestamp(), 12345678);
        assert_eq!(body.nonce(), 0);
        assert_eq!(body.emitter_chain(), 1);

        let err = CoreBridgeGovPayload::try_from(raw_vaa.payload())
            .err()
            .unwrap();
        assert_eq!(err, "ContractUpgrade span too short. Need exactly 34 bytes");
    }

    #[test]
    fn guardian_set_update() {
        let vaa = hex!("01000000000100f2bf6e30b3f45777d23938f4dbeb4e04c5c60de7720f1f9765aa4a1d1a9a9b0c32a0b8c359b12916e67cb510192c1fd8a09d21ef201ae4db8961c05ffebac533011194d7ff000000000001000000000000000000000000000000000000000000000000000000000000000400000000000f78f20100000000000000000000000000000000000000000000000000000000436f72650200000000000102befa429d57cd18b7f8a4d91a2da9ab4af05d0fbe88d7d8b32a9105d228100e72dffe2fae0705d31c");

        let raw_vaa = Vaa::parse(vaa.as_slice()).unwrap();
        assert_eq!(raw_vaa.version(), 1);
        assert_eq!(raw_vaa.guardian_set_index(), 0);
        assert_eq!(raw_vaa.signature_count(), 1);

        let body = raw_vaa.body();
        assert_eq!(body.timestamp(), 294967295);
        assert_eq!(body.nonce(), 0);
        assert_eq!(body.emitter_chain(), 1);

        let payload = CoreBridgeGovPayload::try_from(raw_vaa.payload())
            .unwrap()
            .decree();

        let guardian_set_update = payload.guardian_set_update().unwrap();

        assert_eq!(guardian_set_update.new_index(), 1);
        assert_eq!(guardian_set_update.num_guardians(), 2);

        let expected_guardians = [
            hex!("befa429d57cd18b7f8a4d91a2da9ab4af05d0fbe"),
            hex!("88d7d8b32a9105d228100e72dffe2fae0705d31c"),
        ];
        for (i, expected) in expected_guardians.iter().enumerate() {
            let guardian = guardian_set_update.guardian_at(i);
            assert_eq!(guardian, *expected);
        }

        // Try to access out of bounds.
        let err = guardian_set_update.try_guardian_at(2).err().unwrap();
        assert_eq!(err, "Exceeds number of encoded guardians");
    }

    #[test]
    fn invalid_guardian_set_update() {
        let vaa = hex!("01000000000100f2bf6e30b3f45777d23938f4dbeb4e04c5c60de7720f1f9765aa4a1d1a9a9b0c32a0b8c359b12916e67cb510192c1fd8a09d21ef201ae4db8961c05ffebac533011194d7ff000000000001000000000000000000000000000000000000000000000000000000000000000400000000000f78f20100000000000000000000000000000000000000000000000000000000436f72650200000000000102befa429d57cd18b7f8a4d91a2da9ab4af05d0fbe88d7d8b32a9105d228100e72dffe2fae0705d31c69");

        let raw_vaa = Vaa::parse(vaa.as_slice()).unwrap();
        assert_eq!(raw_vaa.version(), 1);
        assert_eq!(raw_vaa.guardian_set_index(), 0);
        assert_eq!(raw_vaa.signature_count(), 1);

        let body = raw_vaa.body();
        assert_eq!(body.timestamp(), 294967295);
        assert_eq!(body.nonce(), 0);
        assert_eq!(body.emitter_chain(), 1);

        let err = CoreBridgeGovPayload::try_from(raw_vaa.payload())
            .err()
            .unwrap();
        assert_eq!(
            err,
            "GuardianSetUpdate span too short. Need exactly 7 + num_guardians * 20 bytes"
        );
    }

    #[test]
    fn set_message_fee() {
        let vaa = hex!("0100000000010052610605890d9228bd3275591db81af1deef815241e5e8311f32361c3ea3aa931d79efbbe00acaf29f2f5a69787c4ecd4af98e4f936bd53fd32143c4bd57753c0100bc614e000000000001000000000000000000000000000000000000000000000000000000000000000400000000000f69520100000000000000000000000000000000000000000000000000000000436f72650300010000000000000000000000000000000000000000000000000000000000001b39");

        let raw_vaa = Vaa::parse(vaa.as_slice()).unwrap();
        assert_eq!(raw_vaa.version(), 1);
        assert_eq!(raw_vaa.guardian_set_index(), 0);
        assert_eq!(raw_vaa.signature_count(), 1);

        let body = raw_vaa.body();
        assert_eq!(body.timestamp(), 12345678);
        assert_eq!(body.nonce(), 0);
        assert_eq!(body.emitter_chain(), 1);

        let payload = CoreBridgeGovPayload::try_from(raw_vaa.payload())
            .unwrap()
            .decree();

        let set_message_fee = payload.set_message_fee().unwrap();

        assert_eq!(set_message_fee.chain(), 1);
        assert_eq!(
            set_message_fee.fee(),
            hex!("0000000000000000000000000000000000000000000000000000000000001b39")
        );
    }

    #[test]
    fn invalid_set_message_fee() {
        let vaa = hex!("0100000000010052610605890d9228bd3275591db81af1deef815241e5e8311f32361c3ea3aa931d79efbbe00acaf29f2f5a69787c4ecd4af98e4f936bd53fd32143c4bd57753c0100bc614e000000000001000000000000000000000000000000000000000000000000000000000000000400000000000f69520100000000000000000000000000000000000000000000000000000000436f72650300010000000000000000000000000000000000000000000000000000000000001b3969");

        let raw_vaa = Vaa::parse(vaa.as_slice()).unwrap();
        assert_eq!(raw_vaa.version(), 1);
        assert_eq!(raw_vaa.guardian_set_index(), 0);
        assert_eq!(raw_vaa.signature_count(), 1);

        let body = raw_vaa.body();
        assert_eq!(body.timestamp(), 12345678);
        assert_eq!(body.nonce(), 0);
        assert_eq!(body.emitter_chain(), 1);

        let err = CoreBridgeGovPayload::try_from(raw_vaa.payload())
            .err()
            .unwrap();
        assert_eq!(err, "SetMessageFee span too short. Need exactly 34 bytes");
    }

    #[test]
    fn transfer_fees() {
        let vaa = hex!("01000000000100fe09b96ec6e58b1d8ca1bb5721a30f678b0c90dc6462ca8b6d82edbf1fd05eaf11b1ae69ad08b42d2b6a933b838a6a81a5e1932d1df1cb34097a59e0278215e00100bc614e000000000001000000000000000000000000000000000000000000000000000000000000000400000000000f71220100000000000000000000000000000000000000000000000000000000436f7265040001000000000000000000000000000000000000000000000000000000000281edacb2d526e2ab22d8cec062eb8e2b7b8d809803cb4ad11e98667927a1160b9fe408");

        let raw_vaa = Vaa::parse(vaa.as_slice()).unwrap();
        assert_eq!(raw_vaa.version(), 1);
        assert_eq!(raw_vaa.guardian_set_index(), 0);
        assert_eq!(raw_vaa.signature_count(), 1);

        let body = raw_vaa.body();
        assert_eq!(body.timestamp(), 12345678);
        assert_eq!(body.nonce(), 0);
        assert_eq!(body.emitter_chain(), 1);

        let payload = CoreBridgeGovPayload::try_from(raw_vaa.payload())
            .unwrap()
            .decree();

        let transfer_fees = payload.transfer_fees().unwrap();

        assert_eq!(transfer_fees.chain(), 1);
        assert_eq!(
            transfer_fees.amount(),
            hex!("000000000000000000000000000000000000000000000000000000000281edac")
        );
        assert_eq!(
            transfer_fees.recipient(),
            hex!("b2d526e2ab22d8cec062eb8e2b7b8d809803cb4ad11e98667927a1160b9fe408")
        );
    }

    #[test]
    fn invalid_transfer_fees() {
        let vaa = hex!("01000000000100fe09b96ec6e58b1d8ca1bb5721a30f678b0c90dc6462ca8b6d82edbf1fd05eaf11b1ae69ad08b42d2b6a933b838a6a81a5e1932d1df1cb34097a59e0278215e00100bc614e000000000001000000000000000000000000000000000000000000000000000000000000000400000000000f71220100000000000000000000000000000000000000000000000000000000436f7265040001000000000000000000000000000000000000000000000000000000000281edacb2d526e2ab22d8cec062eb8e2b7b8d809803cb4ad11e98667927a1160b9fe40869");

        let raw_vaa = Vaa::parse(vaa.as_slice()).unwrap();
        assert_eq!(raw_vaa.version(), 1);
        assert_eq!(raw_vaa.guardian_set_index(), 0);
        assert_eq!(raw_vaa.signature_count(), 1);

        let body = raw_vaa.body();
        assert_eq!(body.timestamp(), 12345678);
        assert_eq!(body.nonce(), 0);
        assert_eq!(body.emitter_chain(), 1);

        let err = CoreBridgeGovPayload::try_from(raw_vaa.payload())
            .err()
            .unwrap();
        assert_eq!(err, "TransferFees span too short. Need exactly 66 bytes");
    }

    #[test]
    fn invalid_core_bridge_gov() {
        let vaa = hex!("010000000201002424a14044fa5538a5572c519e3b969a716fdf09d9129db2139ba1c3dca9767a53474fb37928e0a0d71c075d8e430d606347a95d4296bade3f6c52e64c4bf7d30100bc614e000000000001000000000000000000000000000000000000000000000000000000000000000400000000001eab9001000000000000000000000000000000000000000000546f6b656e42726964676501000000020000000000000000000000003ee18b2214aff97000d974cf647e7c347e8fa585");

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

        let err = CoreBridgeGovPayload::try_from(raw_vaa.payload())
            .err()
            .unwrap();
        assert_eq!(err, "Invalid Core Bridge governance message");
    }
}
