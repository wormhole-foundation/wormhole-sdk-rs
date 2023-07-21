use wormhole_explorer_client::{Client, VaaRequest};

#[allow(dead_code)]
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
