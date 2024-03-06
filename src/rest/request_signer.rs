use ring::hmac;
use serde::Serialize;

#[derive(Debug, Clone)]
pub struct MonetixRequestSigner {
    secret_key: String,
}

impl MonetixRequestSigner {
    pub fn new(secret_key: impl Into<String>) -> Self {
        Self {
            secret_key: secret_key.into(),
        }
    }
}

pub trait MonetixRequest: Serialize {
    fn to_sign_string(&self) -> String;
}

pub trait MonetixSignPart {
    fn add_sign_parts_sorted(&self, prefix: &str, targer: &mut Vec<String>);
}

impl MonetixRequestSigner {
    pub fn generate_sign<T: MonetixRequest>(&self, data: &T) -> String {
        let data = data.to_sign_string();
        let key = hmac::Key::new(hmac::HMAC_SHA256, self.secret_key.as_bytes());
        let signature = hmac::sign(&key, data.as_bytes());

        signature
            .as_ref()
            .iter()
            .map(|byte| format!("{:02x}", byte))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_sign() {}

    #[test]
    fn generate_sign_with_body() {}
}
