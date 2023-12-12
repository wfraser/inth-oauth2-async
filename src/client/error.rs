use std::error::Error;
use std::{fmt, io};

use crate::client::response::ParseError;
use crate::error::OAuth2Error;

/// Errors that can occur during authorization.
#[derive(Debug)]
pub enum ClientError {
    /// IO error.
    Io(io::Error),

    /// URL error.
    Url(url::ParseError),

    /// Reqwest error.
    #[cfg(feature = "reqwest-client")]
    Reqwest(reqwest::Error),

    /// HTTP error.
    #[cfg(feature = "hyper-client")]
    Http(hyper::http::Error),

    /// Hyper error
    #[cfg(feature = "hyper-client")]
    Hyper(hyper::Error),

    /// Hyper client error
    #[cfg(feature = "hyper-client")]
    HyperClient(hyper_util::client::legacy::Error),

    /// JSON error.
    Json(serde_json::Error),

    /// Response parse error.
    Parse(ParseError),

    /// OAuth 2.0 error.
    OAuth2(OAuth2Error),
}

impl fmt::Display for ClientError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", self.source().unwrap())
    }
}

impl Error for ClientError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match *self {
            ClientError::Io(ref err) => Some(err),
            ClientError::Url(ref err) => Some(err),
            ClientError::Json(ref err) => Some(err),
            ClientError::Parse(ref err) => Some(err),
            ClientError::OAuth2(ref err) => Some(err),

            #[cfg(feature = "reqwest-client")]
            ClientError::Reqwest(ref err) => Some(err),

            #[cfg(feature = "hyper-client")]
            ClientError::Hyper(ref err) => Some(err),

            #[cfg(feature = "hyper-client")]
            ClientError::HyperClient(ref err) => Some(err),

            #[cfg(feature = "hyper-client")]
            ClientError::Http(ref err) => Some(err),
        }
    }
}

macro_rules! impl_from {
    ($v:path, $t:ty) => {
        impl From<$t> for ClientError {
            fn from(err: $t) -> Self {
                $v(err)
            }
        }
    }
}

impl_from!(ClientError::Io, io::Error);
impl_from!(ClientError::Url, url::ParseError);
impl_from!(ClientError::Json, serde_json::Error);
impl_from!(ClientError::Parse, ParseError);
impl_from!(ClientError::OAuth2, OAuth2Error);

#[cfg(feature = "reqwest-client")]
impl_from!(ClientError::Reqwest, reqwest::Error);

#[cfg(feature = "hyper-client")]
impl_from!(ClientError::Http, hyper::http::Error);
#[cfg(feature = "hyper-client")]
impl_from!(ClientError::Hyper, hyper::Error);
#[cfg(feature = "hyper-client")]
impl_from!(ClientError::HyperClient, hyper_util::client::legacy::Error);
