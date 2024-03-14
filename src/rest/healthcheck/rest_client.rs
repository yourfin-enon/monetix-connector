use crate::rest::errors::Error;
use reqwest::header::{HeaderMap, HeaderValue};
use serde::de::DeserializeOwned;
use crate::rest::healthcheck::endpoints::MonetixHealthcheckEndpoint;
use crate::rest::healthcheck::models::GetPaymentUrlArgs;
use crate::rest::healthcheck::signer::MonetixHealthcheckSigner;

#[derive(Clone)]
pub struct MonetixHealthcheckRestClient {
    signer: MonetixHealthcheckSigner,
    host: String,
    inner_client: reqwest::Client,
    project_id: u32,
}

impl MonetixHealthcheckRestClient {
    pub fn new(
        project_id: u32,
        secret_key: String,
        api_url: String,
    ) -> Self {
        Self {
            signer: MonetixHealthcheckSigner::new(secret_key),
            host: api_url,
            inner_client: reqwest::Client::new(),
            project_id,
        }
    }

    pub async fn get_host(&self) -> Result<String, Error> {
        let endpoint = MonetixHealthcheckEndpoint::PaymentHost;

        self.get_signed(&self.host, endpoint, None).await
    }

    pub async fn get_payment_url(&self, args: GetPaymentUrlArgs) -> Result<String, Error> {
        let host = self.get_host().await?;
        let query = serde_qs::to_string(&args).unwrap();
        let endpoint = MonetixHealthcheckEndpoint::PaymentUrl;

        self.get_signed(&host, endpoint, Some(&query)).await
    }

    pub async fn get_signed<T: DeserializeOwned>(
        &self,
        host: &str,
        endpoint: MonetixHealthcheckEndpoint,
        query: Option<&str>,
    ) -> Result<T, Error> {
        let url = if let Some(query) = query  {
            let args = format!("/{}?{}", String::from(&endpoint), query);
            let sign = self.signer.generate_sign(&args)?;
            
            format!("{}/{}/{}", host, self.project_id, sign)
        } else {
            format!("{}/{}", host, String::from(&endpoint))
        };

        let headers = self.build_headers();
        let response = self.inner_client.get(&url).headers(headers).send().await?;

        crate::rest::response_handler::handle(response, None, &url).await
    }

    fn build_headers(&self) -> HeaderMap {
        let mut custom_headers = HeaderMap::new();

        custom_headers.insert(
            "content-type",
            HeaderValue::from_str("application/json").unwrap(),
        );

        custom_headers
    }
}
