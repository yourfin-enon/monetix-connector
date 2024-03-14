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
        let encrypted_data = MonetixHealthcheckCipher::encrypt_with_iv(data.as_bytes(), &self.secret_key, &iv);
        let encoded_data = MonetixHealthcheckCipher::encode(&encrypted_data);
        let encoded_iv = MonetixHealthcheckCipher::encode(&iv);
        let result = format!("{}::{}", encoded_data, encoded_iv);

        Ok(MonetixHealthcheckCipher::encode(result.as_bytes()))
    }

    pub fn decrypt(&self, data: &str) -> Result<String, String> {
        let decoded = MonetixHealthcheckCipher::decode(data.as_bytes())?;
        let data = std::str::from_utf8(&decoded).unwrap();
        let splits: Vec<&str> = data.split("::").collect();
        let data = MonetixHealthcheckCipher::decode(splits[0].as_bytes()).unwrap();
        let iv = MonetixHealthcheckCipher::decode(splits[1].as_bytes()).unwrap();
        let decrypted = MonetixHealthcheckCipher::decrypt_with_iv(&data, &self.secret_key, &iv).unwrap();
        let data = std::str::from_utf8(&decrypted).unwrap();

        Ok(data.to_string())
    }

    fn encode(data: &[u8]) -> String {
        let base64_encoded = &general_purpose::STANDARD.encode(data);

        base64_encoded.to_owned()
    }

    fn decode(data: &[u8]) -> Result<Vec<u8>, String> {
        let base64_decoded = general_purpose::STANDARD.decode(data);

        let Ok(base64_decoded) = base64_decoded else {
            return Err(format!("{}", base64_decoded.unwrap_err()));
        };

        Ok(base64_decoded)
    }

    fn encrypt_with_iv(data: &[u8], key: &str, iv: &[u8]) -> Vec<u8> {
        let mut aes_key = [0; 32];

        for (i, byte) in key.as_bytes().iter().enumerate() {
            aes_key[i] = *byte;
        }

        let cipher = Cipher::new_256(&aes_key);

        cipher.cbc_encrypt(iv, data)
    }

    fn decrypt_with_iv(src: &[u8], key: &str, iv: &[u8]) -> Result<Vec<u8>, String> {
        let mut aes_key = [0; 32];

        for (i, byte) in key.as_bytes().iter().enumerate() {
            aes_key[i] = *byte;
        }

        let cipher = Cipher::new_256(&aes_key);
        let decrypted = cipher.cbc_decrypt(iv, src);

        Ok(decrypted)
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
        let result = cipher.encrypt(data).unwrap();
        
        assert_eq!(data, cipher.decrypt(&result).unwrap());
    }
}
