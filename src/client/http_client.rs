//! HTTP client abstraction and implementations.
//!
//! We provide out-of-the-box implementations for two crates: Hyper, and Reqwest; both of which are
//! gated by Cargo features. The [`HttpClient`] trait can alternatively be implemented for any other
//! client type you need.

use crate::client::error::ClientError;

/// Abstraction of the parts of a HTTP client implementation that this crate needs.
#[async_trait::async_trait]
pub trait HttpClient {
    /// Make a HTTP POST request.
    ///
    /// [`client_id`] and [`client_secret`] are to be given as HTTP Basic Auth credentials username
    /// and password, respectively.
    ///
    /// The [`body`] is of content-type `application/x-www-form-urlencoded`, and the response body
    /// is expected to be `application/json`.
    ///
    /// The response body must be deserialized into a json value.
    async fn post(
        &self,
        url: &str,
        client_id: &str,
        client_secret: &str,
        body: String,
    ) -> Result<serde_json::Value, ClientError>;
}

/// Implementation for Reqwest.
#[cfg(feature = "reqwest-client")]
pub mod reqwest_client {
    use super::*;
    use reqwest::header::{ACCEPT, CONTENT_TYPE};

    #[async_trait::async_trait]
    impl HttpClient for reqwest::Client {
        async fn post(
            &self,
            url: &str,
            client_id: &str,
            client_secret: &str,
            body: String,
        ) -> Result<serde_json::Value, ClientError> {
            let response = reqwest::Client::post(self, url)
                .basic_auth(client_id, Some(client_secret))
                .header(ACCEPT, "application/json")
                .header(CONTENT_TYPE, "application/x-www-form-urlencoded")
                .body(body)
                .send()
                .await?;

            let full = response.bytes().await?;
            let json = serde_json::from_slice(&full)?;

            Ok(json)
        }
    }
}

/// Implementation for Hyper
#[cfg(feature = "hyper-client")]
pub mod hyper_client {
    use super::*;
    use base64::write::EncoderWriter as Base64Encoder;
    use http_body_util::BodyExt;
    use hyper::body::Body;
    use hyper::header::{AUTHORIZATION, ACCEPT, CONTENT_TYPE, HeaderValue};
    use hyper::Request;
    use hyper_util::client::legacy::connect::Connect;
    use std::io::Write;

    #[async_trait::async_trait]
    impl<C, B> HttpClient for hyper_util::client::legacy::Client<C, B> where
        // (°ー°) ...
        C: Connect + Clone + Send + Sync + 'static,
        B: Body + From<String> + Send + Unpin + 'static,
        B::Data: Send,
        B::Error: Into<Box<dyn std::error::Error + Send + Sync>>,
    {
        async fn post(
            &self,
            url: &str,
            client_id: &str,
            client_secret: &str,
            body: String,
        ) -> Result<serde_json::Value, ClientError> {
            let mut auth_header = b"Basic ".to_vec();
            {
                let mut enc = Base64Encoder::new(&mut auth_header, base64::STANDARD);
                write!(enc, "{}:{}", client_id, client_secret)?;
            }

            let mut auth_header_val = HeaderValue::from_bytes(&auth_header)
                .expect("invalid header value"); // should never happen for base64 data
            auth_header_val.set_sensitive(true);

            let req = Request::post(url)
                .header(AUTHORIZATION, auth_header_val)
                .header(ACCEPT, "application/json")
                .header(CONTENT_TYPE, "application/x-www-form-urlencoded")
                .body(body.into())?;

            let response = self.request(req).await?;
            let full = response.into_body().collect().await?.to_bytes();
            let json = serde_json::from_slice(&full)?;
            Ok(json)
        }
    }
}
