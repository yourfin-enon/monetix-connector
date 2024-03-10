use ring::hmac;
use serde::Serialize;
use base64::Engine;
use base64::engine::general_purpose;

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
        let key = hmac::Key::new(hmac::HMAC_SHA512, self.secret_key.as_bytes());
        let signature = hmac::sign(&key, data.as_bytes());

        general_purpose::STANDARD.encode(signature)

    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rest::models::{
        MonetixCreateInvoicePaymentRequest, MonetixCustomerModel, MonetixGeneralModel,
        MonetixPaymentModel,
    };

    #[derive(Serialize)]
    struct TestRequest {}

    impl MonetixRequest for TestRequest {
        fn to_sign_string(&self) -> String {
            "card_operation_type:sale;customer:id:customer_id;general:merchant_callback_url:https//google.com;general:payment_id:payment_id;general:project_id:1000;general:signature:;payment:amount:1000;payment:best_before:3467;payment:currency:USD;payment:description:description;payment:extra_param:extra_param;payment:moto_type:0;return_url:;send_email:0".to_string()
        }
    }

    #[test]
    fn correct_signing_algorithm() {
        let signer = MonetixRequestSigner {
            secret_key: "123abc123abc".to_string(),
        };
        let request = TestRequest {};
        let sign = signer.generate_sign(&request);

        assert_eq!(sign, "ciI5AviOsoIACDm1McI7evYvYKLwjo7bv3+TF4MJkVNh9tPd9RWEYM49w7kgnFg50BpSGD4oU4JUZZkpfg4uTg==");
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
            return_url: None,
            card_operation_type: "sale".to_string(),
            send_email: false,
        };
        let json = serde_json::to_string(&request).unwrap();
        println!("json: {}", json);

        let signer = MonetixRequestSigner {
            secret_key: "123".to_string(),
        };
        let sign_string = request.to_sign_string();
        println!("sign_string: {}", sign_string);

        let sign = signer.generate_sign(&request);
        println!("sign: {}", sign);
    }
}
