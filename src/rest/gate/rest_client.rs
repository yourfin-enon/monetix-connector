use crate::rest::gate::endpoints::MonetixGateEndpoint;
use crate::rest::errors::Error;
use crate::rest::gate::models::{MonetixCreateInvoicePaymentRequest, MonetixCreateInvoicePaymentResponse, MonetixCustomerModel, MonetixGeneralModel, MonetixPaymentModel, MonetixReturnUrlModel};
use crate::rest::signer::{MonetixRequest, MonetixSigner};
use reqwest::header::{HeaderMap, HeaderValue};
use serde::de::DeserializeOwned;
use std::collections::HashMap;
use crate::rest::gate::payout::{MonetixCustomerAccountModel, MonetixCustomerPayoutModel, MonetixPayoutPaymentModel, MonetixPayoutRequest};

#[derive(Clone)]
pub struct MonetixGateRestClient {
    signer: MonetixSigner,
    host: String,
    inner_client: reqwest::Client,
    project_id: u32,
    callback_url: Option<String>,
    return_url: String,
}

impl MonetixGateRestClient {
    pub fn new(
        project_id: u32,
        secret_key: String,
        return_url: String,
        callback_url: Option<String>,
    ) -> Self {
        Self {
            signer: MonetixSigner::new(secret_key),
            host: "https://api.trxhost.com".to_string(),
            inner_client: reqwest::Client::new(),
            project_id,
            return_url,
            callback_url,            
        }
    }

    pub async fn create_invoice_payment(
        &self,
        payment_id: impl Into<String>,
        customer_id: impl Into<String>,
        payment: MonetixPaymentModel,
    ) -> Result<MonetixCreateInvoicePaymentResponse, Error> {
        let mut request = MonetixCreateInvoicePaymentRequest {
            general: MonetixGeneralModel {
                project_id: self.project_id,
                payment_id: payment_id.into(),
                merchant_callback_url: self.callback_url.clone(),
                signature: "".to_string(),
            },
            customer: MonetixCustomerModel {
                id: customer_id.into(),
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
            payment,
            return_url: MonetixReturnUrlModel {
                success: Some(self.return_url.clone()),
                decline: Some(self.return_url.clone()),
                return_url: Some(self.return_url.clone()),
            },
            card_operation_type: "sale".to_string(),
            send_email: false,
        };
        let sign = self.signer.generate_sign(&request)?;

        request.general.signature = sign;

        let endpoint = MonetixGateEndpoint::CreateInvoicePayment;
        let result = self.post(endpoint, request).await;

        result
    }

    pub async fn make_payout(
        &self,
        payment_id: impl Into<String>,
        payment_method: impl Into<String>,
        customer: MonetixCustomerPayoutModel,
        account: MonetixCustomerAccountModel,
        payment: MonetixPayoutPaymentModel,
    ) -> Result<MonetixCreateInvoicePaymentResponse, Error> {
        let mut request = MonetixPayoutRequest {
            general: MonetixGeneralModel {
                project_id: self.project_id,
                payment_id: payment_id.into(),
                merchant_callback_url: self.callback_url.clone(),
                signature: "".to_string(),
            },
            customer,
            account,
            payment,          
        };
        let sign = self.signer.generate_sign(&request)?;

        request.general.signature = sign;

        let endpoint = MonetixGateEndpoint::MakePayout(payment_method.into());
        let result = self.post(endpoint, request).await;

        result
    }

    pub async fn post<R: MonetixRequest, T: DeserializeOwned>(
        &self,
        endpoint: MonetixGateEndpoint,
        request: R,
    ) -> Result<T, Error> {
        let url: String = format!("{}{}", self.host, String::from(&endpoint));
        let headers = self.build_headers();
        let client = &self.inner_client;
        let request_json = serde_json::to_string(&request)?;        
       
        let response = client
            .post(&url)
            .body(request_json.clone())
            .headers(headers)
            .send()
            .await;

        crate::rest::response_handler::handle(response?, Some(request_json), &url).await
    }

    fn build_headers(&self) -> HeaderMap {
        let mut custom_headers = HeaderMap::new();

        custom_headers.insert(
            "content-type",
            HeaderValue::from_str("application/json").unwrap(),
        );

        custom_headers
    }

    pub fn build_query(&self, parameters: HashMap<String, String>) -> String {
        let mut request = String::new();
        for (key, value) in parameters {
            let param = format!("{key}={value}&");
            request.push_str(param.as_ref());
        }
        request.pop();
        request
    }   
}
