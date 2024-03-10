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
        let key = hmac::Key::new(hmac::HMAC_SHA512, self.secret_key.as_bytes());
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
    use crate::rest::models::{
        MonetixCreateInvoicePaymentRequest, MonetixCustomerModel, MonetixGeneralModel,
        MonetixPaymentModel,
    };

    #[derive(Serialize)]
    struct TestRequest {}

    impl MonetixRequest for TestRequest {
        fn to_sign_string(&self) -> String {
            "card_operation_type:sale;customer:address:;customer:billing:;customer:birthplace:;customer:city:;customer:country:;customer:day_of_birth:;customer:first_name:;customer:id:customer_id;customer:language:;customer:last_name:;customer:middle_name:;customer:phone:;customer:ssn:;customer:state:;general:merchant_callback_url:self.callback_url.clone();general:payment_id:payment_id;general:project_id:0;general:signature:;payment:amount:1000;payment:best_before:3467;payment:currency:USD;payment:description:;payment:extra_param:;payment:force_method:;payment:moto_type:;return_url:;send_email:0".to_string()
        }
    }

    #[test]
    fn correct_signing_algorithm() {
        let signer = MonetixRequestSigner {
            secret_key: "fdsakjjfdslajfdasjkjfsd".to_string(),
        };
        let request = TestRequest {};
        let sign = signer.generate_sign(&request);
        
        assert_eq!(sign, "277e6a06abad45e05a5c3391eb42cd3d89453d187a5e4c0268f03b01ab4c54369c1726b00d3064881cfaf6328d85cd03b5b0de209577c69a55352d8ef7ace97e");
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
