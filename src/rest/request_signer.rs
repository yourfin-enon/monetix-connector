use crate::rest::endpoints::MonetixEndpoint;
use ring::hmac;

#[derive(Debug, Clone)]
pub struct MonetixSigner {}

impl MonetixSigner {
    pub fn generate_sign(key: &str, data: &str) -> String {
        let key = hmac::Key::new(hmac::HMAC_SHA256, key.as_bytes());
        let signature = hmac::sign(&key, data.as_bytes());

        signature
            .as_ref()
            .iter()
            .map(|byte| format!("{:02x}", byte))
            .collect()
    }
}

#[derive(Debug, Clone)]
pub struct MonetixRequestSigner {
    secret_key: String,
}

impl MonetixRequestSigner {
    pub fn new(secret_key: String) -> Self {
        Self { secret_key }
    }

    pub fn generate_sign(&self, endpoint: &MonetixEndpoint) -> String {
        let http_method = endpoint.get_http_method();
        let data = format!("{}{}", http_method.as_str(), String::from(endpoint));

        MonetixSigner::generate_sign(&self.secret_key, &data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_sign() {
        let key = "BKxkBRExdqfREsPwEwbydIBSEGssHNAo".to_string();
        let client = MonetixRequestSigner::new(key.clone());

        let sign = client.generate_sign(&MonetixEndpoint::PlatformConfig);

        let source_sign = "e09cb7d69cef805a0f3092c770df60f2e1e91fb3ebdedc8f85f713a7369ba0e5";
        assert_eq!(source_sign, sign);
    }

    #[test]
    fn generate_sign_with_body() {
        let key = "JHDOFKyrhlonPfoXrXiMeFqKRhWYdXTv".to_string();
        let data = "{\"cryptoAmount\":\"0.00163967\",\"cryptoCurrency\":\"BTC\",\"customOrderId\":\"\",\"destinationWallet\":\"mjEcj2LA3vj1nDi8ZD3QMCs9kNqVk7Dpee\",\"fiatAmount\":\"50\",\"fiatCurrency\":\"USD\",\"status\":\"created\",\"tapOnFeeAmount\":\"\",\"tapOnFeeCurrency\":\"\",\"transactionHashes\":null,\"transactionId\":\"cbd38c1b-721a-4fa5-948d-8a628073084b\"}";

        let sign = MonetixSigner::generate_sign(&key, &data);

        let source_sign = "a3c0624fdf21d89e4d35614874b3ac0ef1ed723606c39ab761c75d86e7be0047";
        assert_eq!(source_sign, sign);
    }
}
