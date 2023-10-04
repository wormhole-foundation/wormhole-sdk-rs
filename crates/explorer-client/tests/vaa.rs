#![allow(dead_code)]
use wormhole_explorer_client::{endpoints::vaa_by_tx::VaaByTxHashRequest, Client, VaaRequest};

use hex_literal::hex;

// #[tokio::test]
async fn retrieve_vaas() {
    let req = VaaRequest {
        chain_id: Some(2),
        emitter: None,
        sequence: None,
    };

    let client = Client::mainnet();
    let resp = client.send(&req).await;

    assert!(resp.is_ok());
}

// #[tokio::test]
async fn retrieve_eth_token_bridge() {
    let req = VaaRequest {
        chain_id: Some(2),
        emitter: Some(
            hex!("0000000000000000000000003ee18B2214AFF97000D974cf647E7C347E8fa585").into(),
        ),
        sequence: None,
    };

    let client = Client::mainnet();
    let resp = client.send(&req).await;

    let vaas = resp.unwrap().data;

    for vaa in vaas {
        vaa.deser_vaa().unwrap();
    }
}

// #[tokio::test]
async fn retrieve_single_vaa() {
    let req = VaaRequest {
        chain_id: Some(2),
        emitter: Some(
            hex!("0000000000000000000000003ee18B2214AFF97000D974cf647E7C347E8fa585").into(),
        ),
        sequence: Some(15),
    };

    let client = Client::mainnet();
    let resp = client.send(&req).await;

    let vaas = resp.unwrap().data;
    assert_eq!(vaas.len(), 1);
    for vaa in vaas {
        vaa.deser_vaa().unwrap();
    }
}

// #[tokio::test]
async fn vaa_by_tx_hash() {
    let req: VaaByTxHashRequest =
        "bd012959b806c6087f40e478fc895185d06a2203c9c04ec4ccfd5e56a67b4a89".into();
    let client = Client::testnet();

    dbg!(client.send(&req).await.unwrap());
}
