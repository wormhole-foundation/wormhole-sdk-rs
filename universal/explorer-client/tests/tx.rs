#![allow(dead_code)]
use wormhole_explorer_client::{
    endpoints::tx::{AllTxnsRequest, SingleTxRequest},
    Client,
};

use hex_literal::hex;

// #[tokio::test]
async fn retrieve_txs() {
    let req = AllTxnsRequest;

    let client = Client::new(
        "https://api.wormholescan.io/".parse().unwrap(),
        Default::default(),
    );

    let resp = client.send(&req).await;
    dbg!(&resp);

    assert!(resp.is_ok());
}

// #[tokio::test]
async fn retrieve_eth_token_bridge() {
    let req = SingleTxRequest {
        chain_id: 2,
        emitter: hex!("0000000000000000000000003ee18B2214AFF97000D974cf647E7C347E8fa585").into(),
        sequence: 5,
    };

    let client = Client::new(
        "https://api.wormholescan.io/".parse().unwrap(),
        Default::default(),
    );

    let resp = client.send(&req).await;

    let tx = resp.unwrap();
    dbg!(&tx);
}
