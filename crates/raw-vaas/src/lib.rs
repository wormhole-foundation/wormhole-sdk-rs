mod protocol;
pub use protocol::{GuardianSetSig, Header, Payload, Vaa};

mod payloads;
pub use payloads::{core, token_bridge, GovernanceHeader, GovernanceMessage};

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
        assert_eq!(raw_vaa.signatures().len(), 1);

        let payload = TokenBridgePayload::try_from(raw_vaa.payload()).unwrap();

        let attestation = payload.message().attestation().unwrap();

        assert_eq!(payload.message().transfer().amount(), 1000000000000000000);
    }
}
