use base64::engine::general_purpose;
use base64::Engine;
use ring::hmac;
use serde::Serialize;
use serde_json::Value;

#[derive(Debug, Clone)]
pub struct MonetixSigner {
    secret_key: String,
}

impl MonetixSigner {
    pub fn new(secret_key: impl Into<String>) -> Self {
        Self {
            secret_key: secret_key.into(),
        }
    }
}

pub trait MonetixRequest: Serialize {}

impl MonetixSigner {
    pub fn generate_sign<T: MonetixRequest>(&self, data: &T) -> Result<String, String> {
        let data = serde_json::to_string(data);

        let Ok(data) = data else {
            return Err(format!("{}", data.unwrap_err()));
        };

        let data = MonetixSigner::convert_to_sign_string(&data)?;

        if std::env::var("MONETIX_DEBUG").is_ok() {
            println!("MONETIX_DEBUG->sign_string: {}", data);
        }

        Ok(self.sign_str(&data))
    }

    pub fn generate_sign_from_str(&self, data: &str) -> Result<String, String> {
        let data = MonetixSigner::convert_to_sign_string(data)?;

        if std::env::var("MONETIX_DEBUG").is_ok() {
            println!("MONETIX_DEBUG->sign_string: {}", data);
        }

        Ok(self.sign_str(&data))
    }

    pub fn convert_to_sign_string(data: &str) -> Result<String, String> {
        let parsed_value: Result<Value, _> = serde_json::from_str(data);

        let Ok(parsed_value) = parsed_value else {
            return Err(format!("Invalid json: {}", parsed_value.unwrap_err()));
        };

        let Some(values_by_keys) = parsed_value.as_object() else {
            return Err("Invalid json: not an object".to_string());
        };

        let mut parts = Vec::with_capacity(values_by_keys.len());

        for (key, value) in values_by_keys {
            if let Some(part) = MonetixSigner::key_value_to_string(key, value) {
                if !part.is_empty() {
                    parts.push(part);
                }
            }
        }

        parts.sort();

        Ok(parts.join(";"))
    }

    fn sign_str(&self, data: &str) -> String {
        let key = hmac::Key::new(hmac::HMAC_SHA512, self.secret_key.as_bytes());
        let signature = hmac::sign(&key, data.as_bytes());

        general_purpose::STANDARD.encode(signature)
    }

    fn key_value_to_string(key: &str, value: &Value) -> Option<String> {
        if key.contains("signature") {
            return None;
        }

        let result = match value {
            Value::Null => format!("{}:", key),
            Value::Bool(value) => format!("{}:{}", key, *value as i32),
            Value::Number(value) => format!("{}:{}", key, value),
            Value::String(value) => format!("{}:{}", key, value),
            Value::Array(value) => {
                let mut parts = Vec::with_capacity(value.len());

                for (i, v) in value.iter().enumerate() {
                    let key = format!("{}:{}", key, i);
                    if let Some(part) = MonetixSigner::key_value_to_string(&key, v) {
                        parts.push(part);
                    }
                }

                parts.sort();
                parts.join(";")
            }
            Value::Object(value) => {
                let mut parts = Vec::with_capacity(value.len());

                for (inner_key, inner_value) in value.iter() {
                    println!("{}", key);
                    println!("{}", inner_key);
                    println!("{}", inner_value);

                    if let Some(part) =
                        MonetixSigner::key_value_to_string(&format!("{}:{}", key, inner_key), inner_value)
                    {
                        parts.push(part);
                    }
                }

                parts.sort();
                parts.join(";")
            }
        };

        Some(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rest::gate::models::{MonetixCreateInvoicePaymentRequest, MonetixCustomerModel, MonetixGeneralModel, MonetixPaymentModel, MonetixReturnUrlModel};

    #[test]
    fn correct_signing_algorithm() {
        let signer = MonetixSigner {
            secret_key: "123abc123abc".to_string(),
        };
        let data = "card_operation_type:sale;customer:id:customer_id;general:merchant_callback_url:https//google.com;general:payment_id:payment_id;general:project_id:1000;general:signature:;payment:amount:1000;payment:best_before:3467;payment:currency:USD;payment:description:description;payment:extra_param:extra_param;payment:moto_type:0;return_url:;send_email:0";
        let sign = signer.sign_str(&data);

        assert_eq!(sign, "ciI5AviOsoIACDm1McI7evYvYKLwjo7bv3+TF4MJkVNh9tPd9RWEYM49w7kgnFg50BpSGD4oU4JUZZkpfg4uTg==");
    }

    #[test]
    fn convert_to_sign_string_1() {
        let json = r#"
        {
            "name": "John Doe",
            "general": {"signature": "sfdfds"},
            "last_name": null,
            "middle_name": "",
            "age": 43,
            "phones": [
                "+44 1234567",
                "+44 2345678"
            ]
        }"#;
        let result = MonetixSigner::convert_to_sign_string(json).unwrap();

        assert_eq!(result, "age:43;last_name:;middle_name:;name:John Doe;phones:0:+44 1234567;phones:1:+44 2345678");
    }

    #[test]
    fn convert_to_sign_string_bool() {
        let json = r#"
        {
            "is_true": true,
            "is_false": false
        }"#;
        let result = MonetixSigner::convert_to_sign_string(json).unwrap();

        assert_eq!(result, "is_false:0;is_true:1");
    }

    #[test]
    fn convert_to_sign_string_object() {
        let json = r#"
        {
            "object": {
                "is_true": "true",
                "is_false": "false"
            }
        }"#;
        let result = MonetixSigner::convert_to_sign_string(json).unwrap();

        assert_eq!(result, "object:is_false:false;object:is_true:true");
    }

    #[test]
    fn convert_to_sign_string_double_object_double_fields() {
        let json = r#"
        {
    "operation": {
        "sum_initial": {
            "amount": 10,
            "currency": "USD"
        }
    },
    "signature": "IvyzxypTVp\/8LLVImQGyghpUqA0T7ripi8aNmxVSjoqYm8k2G7VmNkFAMoISUuG6lz1AGIVUh6ilvhtIYhan2Q=="
}"#;
        let result = MonetixSigner::convert_to_sign_string(json).unwrap();

        assert_eq!(result, "operation:sum_initial:amount:10;operation:sum_initial:currency:USD");
    }

    #[test]
    fn to_sign_string() {
        let request = MonetixCreateInvoicePaymentRequest {
            general: MonetixGeneralModel {
                project_id: 1000,
                payment_id: "payment_id".into(),
                merchant_callback_url: Some("https//google.com".into()),
                signature: "".to_string(),
            },
            customer: MonetixCustomerModel {
                id: "customer_id".into(),
                //country: None,
                //city: None,
                //state: None,
                //phone: None,
                //day_of_birth: None,
                //birthplace: None,
                //first_name: None,
                //middle_name: None,
                //last_name: None,
                //language: None,
                //address: None,
                //ssn: None,
                //billing: None,
            },
            payment: MonetixPaymentModel {
                amount: 1000,
                currency: "USD".to_string(),
                description: Some("description".to_string()),
                extra_param: Some("extra_param".to_string()),
                best_before: "3467".to_string(),
                moto_type: 0,
                //force_method: None,
            },
            return_url: MonetixReturnUrlModel {
                success: None,
                decline: None,
                return_url: None,
            },
            card_operation_type: "sale".to_string(),
            send_email: false,
        };
        let signer = MonetixSigner {
            secret_key: "123".to_string(),
        };

        let _sign = signer.generate_sign(&request).unwrap();
        //assert_eq!(sign, "Y+5QpmclSM9RATg0IoXaOFYSPmpCgd+IV2k7jK2gHHChk/sL7H5dcjjR1gZ8gNV0vz9sNIqrOpQo/HcInJ1iTQ==");
    }

    #[test]
    fn test_array_obj_1() {
        let json = r#"
{
    "errors": [{
            "field": "card.pan",
            "message": "Invalid pan number"
        }
    ]    
}
"#;
        let result = MonetixSigner::convert_to_sign_string(json).unwrap();

        assert_eq!(result, "errors:0:field:card.pan;errors:0:message:Invalid pan number");
    }
}
