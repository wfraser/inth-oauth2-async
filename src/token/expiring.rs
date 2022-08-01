use serde_json::Value;
use std::time::{Duration, SystemTime};

use crate::client::response::{FromResponse, ParseError};
use crate::token::Lifetime;

/// An expiring token.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Expiring {
    expires: SystemTime,
}

impl Expiring {
    /// Returns the expiry time of the access token.
    pub fn expires(&self) -> SystemTime { self.expires }
}

impl Lifetime for Expiring {
    fn expired(&self) -> bool { self.expires < SystemTime::now() }
}

impl FromResponse for Expiring {
    fn from_response(json: &Value) -> Result<Self, ParseError> {
        let obj = json.as_object().ok_or(ParseError::ExpectedType("object"))?;

        if obj.contains_key("refresh_token") {
            return Err(ParseError::UnexpectedField("refresh_token"));
        }

        let expires_in = obj.get("expires_in")
            .and_then(Value::as_i64)
            .ok_or(ParseError::ExpectedFieldType("expires_in", "i64"))?;

        Ok(Expiring {
            expires: SystemTime::now() + Duration::from_secs(expires_in.try_into().unwrap_or(0)),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_response() {
        let json = r#"{"expires_in":3600}"#.parse().unwrap();
        let expiring = Expiring::from_response(&json).unwrap();
        assert!(expiring.expires > SystemTime::now());
        assert!(expiring.expires <= SystemTime::now() + Duration::from_secs(3600));
    }
}
