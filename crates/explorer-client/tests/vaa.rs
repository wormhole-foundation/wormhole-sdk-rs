#![allow(dead_code)]
use wormhole_explorer_client::{Client, VaaRequest};

use hex_literal::hex;

// #[tokio::test]
async fn retrieve_vaas() {
    let req = VaaRequest {
        chain_id: Some(2),
        emitter: None,
        sequence: None,
    };

    let client = Client::new(
        "https://api.wormscan.io/".parse().unwrap(),
        Default::default(),
    );

    let resp = client.send(&req).await;
    dbg!(&resp);

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

    let client = Client::new(
        "https://api.wormscan.io/".parse().unwrap(),
        Default::default(),
    );

    let resp = client.send(&req).await;

    let vaas = resp.unwrap().data;

    for vaa in vaas {
        vaa.deser_vaa().unwrap();
    }
}
