use crate::rest::config::MonetixApiConfig;
use crate::rest::endpoints::MonetixEndpoint;
use crate::rest::errors::Error;
use crate::rest::models::{
    MonetixCreateInvoicePaymentRequest, MonetixCreateInvoicePaymentResponse, MonetixCustomerModel,
    MonetixGeneralModel, MonetixPaymentModel,
};
use crate::rest::request_signer::{MonetixRequest, MonetixRequestSigner};
use error_chain::bail;
use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::Response;
use reqwest::StatusCode;
use serde::de::DeserializeOwned;
use std::collections::HashMap;

#[derive(Clone)]
pub struct MonetixRestClient {
    signer: MonetixRequestSigner,
    _api_token: String,
    host: String,
    inner_client: reqwest::Client,
    project_id: u32,
    callback_url: String,
}

impl MonetixRestClient {
    pub fn new(
        project_id: u32,
        secret_key: String,
        api_token: String,
        callback_url: String,
        config: MonetixApiConfig,
    ) -> Self {
        Self {
            signer: MonetixRequestSigner::new(secret_key),
            _api_token: api_token,
            host: config.rest_api_host,
            inner_client: reqwest::Client::new(),
            project_id,
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
                merchant_callback_url: Some(self.callback_url.clone()),
                signature: "".to_string(),
            },
            customer: MonetixCustomerModel {
                id: customer_id.into(),
                country: None,
                city: None,
                state: None,
                phone: None,
                day_of_birth: None,
                birthplace: None,
                first_name: None,
                middle_name: None,
                last_name: None,
                language: None,
                address: None,
                ssn: None,
                billing: None,
            },
            payment,
            return_url: None,
            card_operation_type: "sale".to_string(),
            send_email: false,
        };
        let sign = self.signer.generate_sign(&request);
        request.general.signature = sign;

        let endpoint = MonetixEndpoint::CreateInvoicePayment;
        let result = self.post(endpoint, request).await;

        result
    }

    pub async fn post<R: MonetixRequest, T: DeserializeOwned>(
        &self,
        endpoint: MonetixEndpoint,
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
            .await?;

        self.handler(response, Some(request_json), &url).await
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

    async fn handler<T: DeserializeOwned>(
        &self,
        response: Response,
        request_json: Option<String>,
        request_url: &str,
    ) -> Result<T, Error> {
        match response.status() {
            StatusCode::OK => {
                let json: Result<String, _> = response.text().await;
                let Ok(json) = json else {
                    bail!("Failed to read response body. Url {}", request_url);
                };

                let body: Result<T, _> = serde_json::from_str(&json);
                if let Err(err) = body {
                    bail!(
                        "Url {}. Failed to deserialize body {:?}: {}",
                        request_url,
                        err,
                        json
                    );
                }

                Ok(body.unwrap())
            }
            StatusCode::CREATED => {
                let json: Result<String, _> = response.text().await;
                let Ok(json) = json else {
                    bail!("Failed to read response body");
                };
                let body: Result<T, _> = serde_json::from_str(&json);
                if let Err(err) = body {
                    bail!("Failed to deserialize body {:?}: {}", err, json);
                }

                Ok(body.unwrap())
            }
            StatusCode::INTERNAL_SERVER_ERROR => {
                bail!("Internal Server Error {}", request_url,);
            }
            StatusCode::SERVICE_UNAVAILABLE => {
                bail!("Service Unavailable {}", request_url,);
            }
            StatusCode::UNAUTHORIZED => {
                bail!("Unauthorized {}", request_url);
            }
            StatusCode::BAD_REQUEST => {
                let error = response.text().await?;
                bail!(format!(
                    "Received bad request status. Url: {}. Request: {:?}. Response: {:?}",
                    request_url, request_json, error
                ));
            }
            s => {
                let error = response.text().await?;

                bail!(format!("Received response code: {s:?} error: {error:?}"));
            }
        }
    }
}
