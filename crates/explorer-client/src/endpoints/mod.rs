pub mod vaa;

use std::{future::Future, pin::Pin};

use crate::Client;

pub trait ApiCall: Send + Sync {
    type Return: serde::de::DeserializeOwned;

    const METHOD: reqwest::Method = reqwest::Method::GET;

    fn endpoint(&self) -> String;

    fn send(&self, client: &Client) -> Pin<Box<dyn Future<Output = crate::Result<Self::Return>>>> {
        let mut url = client.root.clone();
        url.set_path(&self.endpoint());

        let fut = client.get(url).send();
        Box::pin(async move { fut.await?.json::<Self::Return>().await.map_err(Into::into) })
    }
}
