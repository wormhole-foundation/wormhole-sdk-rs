use crate::Payload;

/// A Wormhole CCTP payload with type flag
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct WormholeCctpPayload<'a> {
    span: &'a [u8],

    message: WormholeCctpMessage<'a>,
}

impl<'a> AsRef<[u8]> for WormholeCctpPayload<'a> {
    fn as_ref(&self) -> &[u8] {
        self.span
    }
}

impl<'a> TryFrom<Payload<'a>> for WormholeCctpPayload<'a> {
    type Error = &'static str;

    fn try_from(payload: Payload<'a>) -> Result<Self, &'static str> {
        Self::parse(payload.0)
    }
}

impl<'a> WormholeCctpPayload<'a> {
    pub fn span(&self) -> &[u8] {
        self.span
    }

    pub fn message(&self) -> WormholeCctpMessage<'a> {
        self.message
    }

    pub fn parse(span: &'a [u8]) -> Result<Self, &'static str> {
        if span.is_empty() {
            return Err("WormholeCctpPayload span too short. Need at least 1 byte");
        }

        let message = WormholeCctpMessage::parse(span)?;

        Ok(Self { span, message })
    }
}

/// The non-type-flag contents
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum WormholeCctpMessage<'a> {
    Deposit(Deposit<'a>),
    ReservedUnknown(&'a [u8]),
}

impl<'a> TryFrom<Payload<'a>> for WormholeCctpMessage<'a> {
    type Error = &'static str;

    fn try_from(payload: Payload<'a>) -> Result<Self, &'static str> {
        Self::parse(payload.0)
    }
}

impl AsRef<[u8]> for WormholeCctpMessage<'_> {
    fn as_ref(&self) -> &[u8] {
        match self {
            Self::Deposit(inner) => inner.as_ref(),
            Self::ReservedUnknown(inner) => inner,
        }
    }
}

impl<'a> WormholeCctpMessage<'a> {
    pub fn span(&self) -> &[u8] {
        self.as_ref()
    }

    pub fn deposit(&self) -> Option<&Deposit> {
        match self {
            Self::Deposit(inner) => Some(inner),
            _ => None,
        }
    }

    pub fn to_deposit_unchecked(self) -> Deposit<'a> {
        match self {
            Self::Deposit(inner) => inner,
            _ => panic!("WormholeCctpMessage is not Deposit"),
        }
    }

    pub fn parse(span: &'a [u8]) -> Result<Self, &'static str> {
        if span.is_empty() {
            return Err("WormholeCctpMessage span too short. Need at least 1 byte");
        }

        match span[0] {
            1 => Ok(Self::Deposit(Deposit::parse(&span[1..])?)),
            2..=10 => Ok(Self::ReservedUnknown(&span[1..])),
            _ => Err("Unknown WormholeCctpMessage type"),
        }
    }
}

/// A CCTP deposit transfer with message
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Deposit<'a>(&'a [u8]);

impl AsRef<[u8]> for Deposit<'_> {
    fn as_ref(&self) -> &[u8] {
        self.0
    }
}

impl<'a> Deposit<'a> {
    pub fn token_address(&self) -> [u8; 32] {
        self.0[..32].try_into().unwrap()
    }

    pub fn amount(&self) -> [u8; 32] {
        self.0[32..64].try_into().unwrap()
    }

    pub fn source_cctp_domain(&self) -> u32 {
        u32::from_be_bytes(self.0[64..68].try_into().unwrap())
    }

    pub fn destination_cctp_domain(&self) -> u32 {
        u32::from_be_bytes(self.0[68..72].try_into().unwrap())
    }

    pub fn cctp_nonce(&self) -> u64 {
        u64::from_be_bytes(self.0[72..80].try_into().unwrap())
    }

    pub fn burn_source(&self) -> [u8; 32] {
        self.0[80..112].try_into().unwrap()
    }

    pub fn mint_recipient(&self) -> [u8; 32] {
        self.0[112..144].try_into().unwrap()
    }

    pub fn payload_len(&self) -> u16 {
        u16::from_be_bytes(self.0[144..146].try_into().unwrap())
    }

    pub fn payload(&self) -> &[u8] {
        &self.0[146..]
    }

    pub fn parse(span: &'a [u8]) -> Result<Self, &'static str> {
        if span.len() < 146 {
            return Err("Deposit span too short. Need at least 146 bytes");
        }

        let deposit = Self(span);

        // Check payload length vs actual payload.
        if deposit.payload().len() != deposit.payload_len().into() {
            return Err("Deposit payload length mismatch");
        }

        Ok(deposit)
    }
}

#[cfg(test)]
mod test {
    use crate::{cctp::WormholeCctpPayload, Vaa};
    use hex_literal::hex;

    #[test]
    fn deposit() {
        // Taken from https://etherscan.io/tx/0xa8e7944904722c4be7cf262add16216c797550a7810b4b8bc4e38ec7bd815abb.
        let vaa = hex!("01000000030d008f3ba001a8989c503cb16f8ddd9383fff5ae2c08d19180622e77abebe213d90841c8fb320c921418a5b852fd08f5795d92150d1ea926b52223ca5ee81e7672940101cd7ca803056c103cf5020d8981432d7b3019decd234ea66034a08d8a899b7b2a0e801b276682cbfeaa45d3a9273ff1ff66c08f7c098c58daeaa50f67a93ec33b0102f182520f56fa2252933e572e27fb0302517270550cf561de5b4cd1c8694981772aea0142a790b3c0caea96b9926d8e026608a1b210e85cad7d64731d0fca36c90103804c109e4598cdfe06765d4715760017c8dca6ea731268900d12dd53c172683716ac9ea33409c8975e56af046d475e42ce06d7ffcbde21ea1bb3e9defa11e5e10104193f41fecb73c65068247b0e5892e45187b365c1458e2a6bdacfdc98df9c767503859856e4200608fdf9605e70b2f5ac73f07b0ea4b0795483b1248d1782881d0106d5d667ab1c911a5f1a74362dfacb73bbbc3f62537123e103af2e7f8ffb6fe94c5f10cecd34e77f9b318f39285be7966751232e9db7568dabe4a761bfc779c76501088fc193d3e88d170ebb36d48fd83f51988db202aa07dad917c4690521492bc592204970b3a67aea15b95f8baabbe11f95c9517bbce98ce300923e855dafe14dec01096160db93b351b1b101a7d592ba42c1d79920690bc8e5ad1f9de82faf86dd94410c462eb5f201d507bc9c1136124d5f0ade8b26da01b262ce1cb94b3e7a254d1e000a5c23a43489865898a1790d67bf5583e2386e9cd7ee00ed3398ad2a1a1e642da4580630b10fb613a4b9a003aaea7fa8f9b09e501cd5a08180b64e784bad9d0506010cda6e0e5eba3ff2b0995edd62e0ca33e1b9af879af11ee7b834975d555581a04b61c8efaad51f38dbc4bf297a0452f5fa48bd6ff328c50da9d28c388dd0245f91010fb51583f772a9bb9f7545a8f76ad21f1bf54d6bd94734cc5b8ae858a2ca29c5bf2f1a71973f60da2dbcd20cab71b5ab94a4963a65db733541a6a07d18b404d2aa0111e26f2bbffb60143465026bbc7366035107c572ced056d53fde9aa1630bd3e8a144effe8ee9f9c710ef564c153597f3cf7e689b25a4b9bd42d542ad1411701e0200122d75e9c8c4f8ff25e5702eee2286bb12a4ca892221c991b651fe3ca2758a2e087c715a4a5486d6247ad391cf0fbc3c1d0c0e84573f7081e93ff6373e1117209b0064f34b4b00000000000600000000000000000000000009fb06a271faff70a651047395aaeb6265265f13000000000000058a0101000000000000000000000000b97ef9ef8734c71904d8002f8b6bc66dd9c48a6e0000000000000000000000000000000000000000000000000000000005f5e10000000001000000000000000000001a0c00000000000000000000000068742c08bd367031216aa14725bd347e49be895b00000000000000000000000068742c08bd367031216aa14725bd347e49be895b0000");

        let raw_vaa = Vaa::parse(vaa.as_slice()).unwrap();
        assert_eq!(raw_vaa.version(), 1);
        assert_eq!(raw_vaa.guardian_set_index(), 3);
        assert_eq!(raw_vaa.signature_count(), 13);

        let body = raw_vaa.body();
        assert_eq!(body.timestamp(), 1693666123);
        assert_eq!(body.nonce(), 0);
        assert_eq!(body.emitter_chain(), 6);

        let payload = WormholeCctpPayload::try_from(raw_vaa.payload())
            .unwrap()
            .message();

        let deposit = payload.deposit().unwrap();

        assert_eq!(
            deposit.token_address(),
            hex!("000000000000000000000000b97ef9ef8734c71904d8002f8b6bc66dd9c48a6e")
        );
        assert_eq!(
            deposit.amount(),
            hex!("0000000000000000000000000000000000000000000000000000000005f5e100")
        );
        assert_eq!(deposit.source_cctp_domain(), 1);
        assert_eq!(deposit.destination_cctp_domain(), 0);
        assert_eq!(deposit.cctp_nonce(), 6668);
        assert_eq!(
            deposit.burn_source(),
            hex!("00000000000000000000000068742c08bd367031216aa14725bd347e49be895b")
        );
        assert_eq!(
            deposit.mint_recipient(),
            hex!("00000000000000000000000068742c08bd367031216aa14725bd347e49be895b")
        );
        assert_eq!(deposit.payload_len(), 0);
        assert_eq!(deposit.payload(), &[]);
    }

    #[test]
    fn payload_length_mismatch_deposit_with_message() {
        // Taken from https://etherscan.io/tx/0xa8e7944904722c4be7cf262add16216c797550a7810b4b8bc4e38ec7bd815abb.
        let vaa = hex!("01000000030d008f3ba001a8989c503cb16f8ddd9383fff5ae2c08d19180622e77abebe213d90841c8fb320c921418a5b852fd08f5795d92150d1ea926b52223ca5ee81e7672940101cd7ca803056c103cf5020d8981432d7b3019decd234ea66034a08d8a899b7b2a0e801b276682cbfeaa45d3a9273ff1ff66c08f7c098c58daeaa50f67a93ec33b0102f182520f56fa2252933e572e27fb0302517270550cf561de5b4cd1c8694981772aea0142a790b3c0caea96b9926d8e026608a1b210e85cad7d64731d0fca36c90103804c109e4598cdfe06765d4715760017c8dca6ea731268900d12dd53c172683716ac9ea33409c8975e56af046d475e42ce06d7ffcbde21ea1bb3e9defa11e5e10104193f41fecb73c65068247b0e5892e45187b365c1458e2a6bdacfdc98df9c767503859856e4200608fdf9605e70b2f5ac73f07b0ea4b0795483b1248d1782881d0106d5d667ab1c911a5f1a74362dfacb73bbbc3f62537123e103af2e7f8ffb6fe94c5f10cecd34e77f9b318f39285be7966751232e9db7568dabe4a761bfc779c76501088fc193d3e88d170ebb36d48fd83f51988db202aa07dad917c4690521492bc592204970b3a67aea15b95f8baabbe11f95c9517bbce98ce300923e855dafe14dec01096160db93b351b1b101a7d592ba42c1d79920690bc8e5ad1f9de82faf86dd94410c462eb5f201d507bc9c1136124d5f0ade8b26da01b262ce1cb94b3e7a254d1e000a5c23a43489865898a1790d67bf5583e2386e9cd7ee00ed3398ad2a1a1e642da4580630b10fb613a4b9a003aaea7fa8f9b09e501cd5a08180b64e784bad9d0506010cda6e0e5eba3ff2b0995edd62e0ca33e1b9af879af11ee7b834975d555581a04b61c8efaad51f38dbc4bf297a0452f5fa48bd6ff328c50da9d28c388dd0245f91010fb51583f772a9bb9f7545a8f76ad21f1bf54d6bd94734cc5b8ae858a2ca29c5bf2f1a71973f60da2dbcd20cab71b5ab94a4963a65db733541a6a07d18b404d2aa0111e26f2bbffb60143465026bbc7366035107c572ced056d53fde9aa1630bd3e8a144effe8ee9f9c710ef564c153597f3cf7e689b25a4b9bd42d542ad1411701e0200122d75e9c8c4f8ff25e5702eee2286bb12a4ca892221c991b651fe3ca2758a2e087c715a4a5486d6247ad391cf0fbc3c1d0c0e84573f7081e93ff6373e1117209b0064f34b4b00000000000600000000000000000000000009fb06a271faff70a651047395aaeb6265265f13000000000000058a0101000000000000000000000000b97ef9ef8734c71904d8002f8b6bc66dd9c48a6e0000000000000000000000000000000000000000000000000000000005f5e10000000001000000000000000000001a0c00000000000000000000000068742c08bd367031216aa14725bd347e49be895b00000000000000000000000068742c08bd367031216aa14725bd347e49be895b0001");

        let raw_vaa = Vaa::parse(vaa.as_slice()).unwrap();
        assert_eq!(raw_vaa.version(), 1);
        assert_eq!(raw_vaa.guardian_set_index(), 3);
        assert_eq!(raw_vaa.signature_count(), 13);

        let body = raw_vaa.body();
        assert_eq!(body.timestamp(), 1693666123);
        assert_eq!(body.nonce(), 0);
        assert_eq!(body.emitter_chain(), 6);

        let err = WormholeCctpPayload::try_from(raw_vaa.payload())
            .err()
            .unwrap();
        assert_eq!(err, "Deposit payload length mismatch");
    }
}
