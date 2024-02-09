pub mod tx;
pub mod vaa;
pub mod vaa_by_tx;

use std::{fmt::Debug, future::Future, pin::Pin};

use reqwest::Url;
use tracing::Instrument;

use crate::{error::ApiError, Client};

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

    /// Append the endpoint to the URL.
    ///
    /// This is typically computed from the request contents and a constant
    /// stem. No POST requests are currently supported.
    fn add_endpoint(&self, url: &mut Url);

    /// Append any query args to the URL.
    fn add_query_args(&self, _url: &mut Url) {}

    /// Send the request over an API client.
    ///
    /// Do not override this method unless you know what you're doing.
    #[tracing::instrument(skip(self, client), fields(url, uuid))]
    fn send(&self, client: &Client) -> Pin<Box<dyn Future<Output = crate::Result<Self::Return>>>> {
        // UUID ensures event lifecycles can be tracked
        let uuid = uuid::Uuid::new_v4();
        let mut url = client.root.clone();

        self.add_endpoint(&mut url);
        self.add_query_args(&mut url);

        // populate span with above
        tracing::Span::current().record("url", url.as_str());
        tracing::Span::current().record("uuid", &uuid.to_string());

        tracing::debug!("prepped response");
        let fut = client.get(url.clone()).send();
        Box::pin(
            async move {
                tracing::debug!("sending request");
                let resp = fut.await?;
                let text = resp.text().await?;
                tracing::debug!("received response");
                tracing::trace!(text);

                let res = serde_json::from_str::<Self::Return>(&text).map_err(Into::into);
                // if the res is  an error, try to deser the text as an API error object.
                if res.is_err() {
                    if let Ok(err) = serde_json::from_str::<ApiError>(&text) {
                        return Err(err.into());
                    }
                    tracing::error!(text, "unknown error response from server");
                }
                res
            }
            .in_current_span(),
        )
    }
}
