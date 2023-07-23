pub mod vaa;

use std::{fmt::Debug, future::Future, pin::Pin};

use tracing::Instrument;

use crate::Client;

/// A call to the [Wormhole Explorer API].
///
/// [Wormhole Explorer API]: https://doc.wormscan.io/
pub trait ApiCall: Send + Sync + Debug {
    /// We expect the API to return some data of this type in a json object of
    /// the following shape:
    ///
    /// ```ignore
    /// {
    ///   "data": Self::Return,
    ///   "pagination": { "next": "" },
    /// }
    /// ```
    type Return: serde::de::DeserializeOwned;

    /// Return the endpoint to which this request should be sent.
    ///
    /// This is typically computed from the request contents and a constant
    /// stem. No POST requests are currently supported
    fn endpoint(&self) -> String;

    /// Send the request over an API client.
    ///
    /// Do not override this method unless you know what you're doing.
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
