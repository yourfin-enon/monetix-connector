use base64::Engine;
use base64::engine::general_purpose;
use libaes::Cipher;
use rand::RngCore;

#[derive(Debug, Clone)]
pub struct MonetixHealthcheckCipher {
    secret_key: String,
}

impl MonetixHealthcheckCipher {
    pub fn new(secret_key: impl Into<String>) -> Self {
        Self {
            secret_key: secret_key.into(),
        }
    }
}

impl MonetixHealthcheckCipher {
    pub fn encrypt(&self, data: &str) -> Result<String, String> {
        let iv = MonetixHealthcheckCipher::get_iv();
        let encrypted_data = MonetixHealthcheckCipher::encrypt_inner(data.as_bytes(), &self.secret_key, &iv);
        let encoded_data = MonetixHealthcheckCipher::encode(&encrypted_data);
        let encoded_iv = MonetixHealthcheckCipher::encode(&iv);
        let result = format!("{}::{}", encoded_data, encoded_iv);

        Ok(MonetixHealthcheckCipher::encode(result.as_bytes()))
    }

    fn encode(data: &[u8]) -> String {
        let base64_encoded = &general_purpose::STANDARD.encode(data);

        base64_encoded.to_owned()
    }

    fn encrypt_inner(data: &[u8], key: &str, iv: &[u8]) -> Vec<u8> {
        let mut aes_key = [0; 32];

        for (i, byte) in key.as_bytes().iter().enumerate() {
            aes_key[i] = *byte;
        }

        let cipher = Cipher::new_256(&aes_key);

        cipher.cbc_encrypt(iv, data)
    }

    fn get_iv() -> Vec<u8> {
        let mut buf = vec![0; 16];
        rand::thread_rng().fill_bytes(&mut buf);

        buf
    }
}

#[cfg(test)]
mod tests {
    use crate::rest::healthcheck::cipher::MonetixHealthcheckCipher;

    #[test]
    fn encrypt_1() {
        let data = "/payment?project_id=123&payment_amount=100&payment_id=payment_47&payment_currency=USD&customer_id=customer_123&customer_first_name=John&customer_last_6name=Doe&customer_email=johndoe%40example.com&signature=YWb6Z20ByxpQ%2B30hfTIjaCCsVIwVynXV%252BVLe";
        let cipher = MonetixHealthcheckCipher {
            secret_key: "secret".to_string(),
        };
        let _result = cipher.encrypt(data).unwrap();

        //assert_eq!(result, "cDROL0FqVC9wck1keWhrWmZ3Zm9EQ3NJQmZQKzNpYVdMb2dDbHU4bmJEUUc5TGVweVJoOTl2UXQ3NXFnZk9ZS2FUaHgzQzVnS081K0s4b3IvUDQ0VVhXNzlCLzFDL1BJQnYxZFdFQWU4RTNZK2hZbFdsbGtHN1pkRG82ZXlUb1FjOVpMVGQxVjQ5S0ZJYVZiY3BFcng4ZHB5ZG9YRnFBeElwckxzQnUyT1hDeEIzYlVqekVLRHV1dExRdXU3aUNHdWlEMUJiMzVDeHhiRktseGgraG5uSjFqY2U2RGg3YjVRY0M1b0RWc2VLSGhveDh0NEIrUk16a0NtM3BhWFhmZ3hMUytXeS9vcUxYcnZPS1pBQ3hEb2hBZXJTTjdRY0pISjczaDNWS0VQbUFyM2JFdnRyRVVrWkl6bG9oSVNsSDlBdEJqMFMyTlBCeG4xYnJEZjA1NUNGMHF5d3JtVjZaWTRsQkZHTWFrVU1ONmhZSTU0a0hzb1hXNUJ3QSt6b2x4aERCOENUUXZQMGgrRDBMSDZtODB1dk9hb1VwMDVOZWl5QWZYZjZpS0pqbkRLMlpYK3lkSUU0NW1zRGY2VDN4VVhTZDBJOVEwSnlqK3ZGNEdZT3ZWV3FpUm02NW5hWTlmZ0xwWWpmWXZXMFBMWlpoMSt1SmVkK3hmZ1h3QzVJNjY4cGlUajVQWHlzcVBEWFNJcVUzdjVkT0x4OVByWWRsMDVqcG9jTnJqUFc0Y0tueHplUmF4S0NNU3hxaG1FOUdhVWtCT0t3M0tSUHo1SzZQR1F1bVgxQT09OjpaVEl5WVRBeE5HSTFabVpoTkRreU1BPT0");
    }
}
