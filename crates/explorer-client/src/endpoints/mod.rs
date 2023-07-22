pub mod vaa;

use std::{fmt::Debug, future::Future, pin::Pin};

use tracing::Instrument;

use crate::Client;

pub trait ApiCall: Send + Sync + Debug {
    type Return: serde::de::DeserializeOwned;

    const METHOD: reqwest::Method = reqwest::Method::GET;

    fn endpoint(&self) -> String;

    #[tracing::instrument(skip(self, client), fields(url, uuid))]
    fn send(&self, client: &Client) -> Pin<Box<dyn Future<Output = crate::Result<Self::Return>>>> {
        // UUID ensures event lifecycles can be tracked
        let uuid = uuid::Uuid::new_v4();
        let mut url = client.root.clone();
        url.set_path(&self.endpoint());

        // populate span with above
        tracing::Span::current().record("url", &url.as_str());
        tracing::Span::current().record("uuid", &uuid.to_string());

        tracing::debug!("prepped response");
        let fut = client.get(url.clone()).send();
        Box::pin(
            async move {
                tracing::debug!("sending request");
                let resp = fut.await?;
                let text = resp.text().await?;
                tracing::debug!(text, "received response");

                serde_json::from_str::<Self::Return>(&text).map_err(Into::into)
            }
            .in_current_span(),
        )
    }
}
