use alloy_primitives::FixedBytes;

use crate::{EncodedAmount, Readable, TypePrefixedPayload, Writeable};

use std::io;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Transfer {
    pub norm_amount: EncodedAmount,
    pub token_address: FixedBytes<32>,
    pub token_chain: u16,
    pub recipient: FixedBytes<32>,
    pub recipient_chain: u16,
    pub norm_relayer_fee: EncodedAmount,
}

impl TypePrefixedPayload for Transfer {
    const TYPE: Option<u8> = Some(1);
}

impl Readable for Transfer {
    const SIZE: Option<usize> = Some(32 + 32 + 2 + 32 + 2 + 32);
    fn read<R>(reader: &mut R) -> io::Result<Self>
    where
        Self: Sized,
        R: io::Read,
    {
        Ok(Self {
            norm_amount: Readable::read(reader)?,
            token_address: Readable::read(reader)?,
            token_chain: Readable::read(reader)?,
            recipient: Readable::read(reader)?,
            recipient_chain: Readable::read(reader)?,
            norm_relayer_fee: Readable::read(reader)?,
        })
    }
}

impl Writeable for Transfer {
    fn write<W>(&self, writer: &mut W) -> io::Result<()>
    where
        Self: Sized,
        W: io::Write,
    {
        self.norm_amount.write(writer)?;
        self.token_address.write(writer)?;
        self.token_chain.write(writer)?;
        self.recipient.write(writer)?;
        self.recipient_chain.write(writer)?;
        self.norm_relayer_fee.write(writer)?;
        Ok(())
    }

    fn written_size(&self) -> usize {
        <Self as Readable>::SIZE.unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{payloads::token_bridge::TokenBridgeMessage, Vaa};
    use alloy_primitives::U64;
    use hex_literal::hex;

    // https://github.com/wormhole-foundation/wormhole/blob/b09a644dac97fa8e037a16765728217ff3a1d057/clients/js/parse_tests/token-bridge-transfer-1.expected
    #[test]
    fn token_bridge_transfer_1() {
        let vaa = hex!("010000000001007d204ad9447c4dfd6be62406e7f5a05eec96300da4048e70ff530cfb52aec44807e98194990710ff166eb1b2eac942d38bc1cd6018f93662a6578d985e87c8d0016221346b0000b8bd0001c69a1b1a65dd336bf1df6a77afb501fc25db7fc0938cb08595a9ef473265cb4f0000000000000003200100000000000000000000000000000000000000000000000000000002540be400165809739240a0ac03b98440fe8985548e3aa683cd0d4d9df5b5659669faa3010001000000000000000000000000c10820983f33456ce7beb3a046f5a83fa34f027d00020000000000000000000000000000000000000000000000000000000000000000");

        let vaa = Vaa::read(&mut vaa.as_slice()).unwrap();

        assert_eq!(vaa.header.version, 1);
        assert_eq!(vaa.header.guardian_set_index, 0);
        assert_eq!(vaa.header.signatures.len(), 1);
        assert_eq!(vaa.header.signatures[0].guardian_set_index, 0);
        assert_eq!(vaa.header.signatures[0].signature, hex!("7d204ad9447c4dfd6be62406e7f5a05eec96300da4048e70ff530cfb52aec44807e98194990710ff166eb1b2eac942d38bc1cd6018f93662a6578d985e87c8d001"));

        assert_eq!(vaa.body.timestamp, 1646343275);
        assert_eq!(vaa.body.nonce, 47293);
        assert_eq!(vaa.body.emitter_chain, 1);
        assert_eq!(
            vaa.body.emitter_address,
            hex!("c69a1b1a65dd336bf1df6a77afb501fc25db7fc0938cb08595a9ef473265cb4f")
        );
        assert_eq!(vaa.body.sequence, U64::from(3));
        assert_eq!(vaa.body.consistency_level, 32);
        assert_eq!(
            vaa.body.double_digest(),
            hex!("2862e5873955ea104bb3e122831bdc43bbcb413da5b1123514640b950d038967")
        );

        let msg = vaa.body.read_payload::<TokenBridgeMessage>().unwrap();

        assert_eq!(msg.to_vec(), vaa.body.payload_bytes().unwrap());

        if let TokenBridgeMessage::Transfer(transfer) = &msg {
            assert_eq!(
                transfer,
                &Transfer {
                    norm_amount: EncodedAmount::from(10000000000u64),
                    token_address: hex!(
                        "165809739240a0ac03b98440fe8985548e3aa683cd0d4d9df5b5659669faa301"
                    )
                    .into(),
                    token_chain: 1,
                    recipient: hex!(
                        "000000000000000000000000c10820983f33456ce7beb3a046f5a83fa34f027d"
                    )
                    .into(),
                    recipient_chain: 2,
                    norm_relayer_fee: EncodedAmount::ZERO,
                }
            );
        } else {
            panic!("wrong message type");
        }

        // let msg_2 = vaa.body.read_payload::<Transfer>().unwrap();

        let msg_2 = Transfer::read_payload(&mut vaa.body.payload_bytes().unwrap()).unwrap();

        assert_eq!(TokenBridgeMessage::Transfer(msg_2), msg);
    }

    // https://github.com/wormhole-foundation/wormhole/blob/b09a644dac97fa8e037a16765728217ff3a1d057/clients/js/parse_tests/token-bridge-transfer-2.expected
    #[test]
    fn token_bridge_transfer_2() {
        let vaa = hex!("01000000010d0078588270e30e3b4cf74572b6ad4270cdd7932079692170fddaf369c7574722b75defcecf5d372cdd8fdba0f275c6b902434259b5d7da8402e25ca852ca5affaa0003a8888cf66158970861329efa69ff2461d847078cec22fd7f62606b17a1ae283127712fa50dc365faa1e6db339fefce57b13c74c2dce7d14b79051676c74bb685000487272398eb59763bb1e2466f9ebdea4e75c290b6c0386f07c20e1296b1976cb814547378922dbc5490b7fcf7279eafc0c08bd59ca97c4dbbcbd478967e17aa2d0006dd38ecb6233f1cd872a75cc0627ded36aa8f89095436f7dbe32e6655e27f217459fda35a3d7f1d656962160bfeee4e5fc6d2e1447559e7bc3ba760416317b86c010792d27a749b398dc5f085e7bcd2e0f18d6262a1ba1916787ec01854c0ccde0a8247f8892e6dff83fad6839fc054f32734255e9037ff9adc33499514e2300ba439010989f08688ae363783bfe3f25a5960a0791ce327bab7e7593393f91395e06fe50e3f7e13862ac86b9fd1f9720669bc4504e918f7e481c395f17a2fa131da05b9e7010a097d187970710297d188a2ebaedff0ad13efd16872566bae8a56377e28466b2c3c4e47853c60fe716109e55f8b453fb03a34bb1929c96f74ebd796a476ec7ab6000b68a19d198350b3caebd3c0159b8bbce022e0f026d013a1c83e40d6100c87e8bb0d692baca89cb77f4b6832dd7aaf3f2f7c482fd50be7221c046ae668228ec013000cd6f464a174d7e34797e2869785feb5f05ab614be989d238c9bd55259dbdbab2568c14f316d1820ac766e513bf5225185f16d30f0f01a092af5fb6b072ad577f0010d663f2f3ad62baa8ad541b9c38bb9df805d2cfa7072894526505b654293bacdee5e9e8c4ded7be92a3338b964482b3ce6d5275817d6a4b6a0663e1e84dcd1de3500105f773ea1d7e74770e78c4779abe4594b6a46f9131304948265bc185dcb1cdba8114915e3b1d864f48e4c694c9578524e22752e2d898af4b8e67383d72a11856700118bdbd5b5a820ecd215faf134b698402da04cc698e64464dd8df6692342e8c44314e1ae53bfde71fb2b00cd5691dae4f9b310c6150bdb551645a72863f4ff965c011286c673c4f2213969d273b939318f93a5b50c665efa8c9e245a3b8823522dafec209b1be127e74a6d5c924831e339f8bffb769f7b0f5772ed16231700bf7eece200624092e10000f4150001ec7372995d5cc8732397fb0ad35c0121e0eaa90d26f828a534cab54391b3a4f5000000000001aec5200100000000000000000000000000000000000000000000000000000000f4610900069b8857feab8184fb687f634618c035dac439dc1aeb3b5598a0f000000000010001000000000000000000000000efd4aa8f954ebdea82b8757c029fc8475a45e9cd00020000000000000000000000000000000000000000000000000000000000000000");

        let vaa = Vaa::read(&mut vaa.as_slice()).unwrap();
        assert_eq!(vaa.header.version, 1);
        assert_eq!(vaa.header.guardian_set_index, 1);
        assert_eq!(vaa.header.signatures.len(), 13);

        assert_eq!(vaa.body.timestamp, 1648399073);
        assert_eq!(vaa.body.nonce, 62485);
        assert_eq!(vaa.body.emitter_chain, 1);
        assert_eq!(
            vaa.body.emitter_address,
            hex!("ec7372995d5cc8732397fb0ad35c0121e0eaa90d26f828a534cab54391b3a4f5")
        );
        assert_eq!(vaa.body.sequence, U64::from(110277));
        assert_eq!(vaa.body.consistency_level, 32);
        assert_eq!(
            vaa.body.double_digest(),
            hex!("c90519b2bdfacac401d2d2c15a329d4e33e8ca15862685f0220ddc6074d7def5")
        );

        let msg = vaa.body.read_payload::<TokenBridgeMessage>().unwrap();
        assert_eq!(msg.to_vec(), vaa.body.payload_bytes().unwrap());

        if let TokenBridgeMessage::Transfer(transfer) = msg {
            assert_eq!(transfer.norm_amount, EncodedAmount::from(4100000000u64));
            assert_eq!(
                transfer.token_address,
                hex!("069b8857feab8184fb687f634618c035dac439dc1aeb3b5598a0f00000000001")
            );
            assert_eq!(transfer.token_chain, 1);
            assert_eq!(
                transfer.recipient,
                hex!("000000000000000000000000efd4aa8f954ebdea82b8757c029fc8475a45e9cd")
            );
            assert_eq!(transfer.recipient_chain, 2);
            assert_eq!(transfer.norm_relayer_fee, EncodedAmount::ZERO);
        } else {
            panic!("wrong message type");
        }
    }
}
