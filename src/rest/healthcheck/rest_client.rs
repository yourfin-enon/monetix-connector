use crate::rest::errors::Error;
use reqwest::header::{HeaderMap, HeaderValue};
use crate::rest::signer::MonetixSigner;
use crate::rest::healthcheck::endpoints::MonetixHealthcheckEndpoint;
use crate::rest::healthcheck::models::{GetPaymentPageArgs, PaymentPageConfig};
use crate::rest::cipher::MonetixCipher;

#[derive(Clone)]
pub struct MonetixHealthcheckRestClient {
    cipher: MonetixCipher,
    signer: MonetixSigner,
    host: String,
    inner_client: reqwest::Client,
    project_id: u32,
}

impl MonetixHealthcheckRestClient {
    pub fn new(
        project_id: u32,
        secret_key: impl Into<String>,
        encryption_key: impl Into<String>,
        api_url: impl Into<String>,
    ) -> Self {
        Self {
            cipher: MonetixCipher::new(encryption_key),
            signer: MonetixSigner::new(secret_key),
            host: api_url.into(),
            inner_client: reqwest::Client::new(),
            project_id,
        }
    }

    pub async fn get_payment_host(&self) -> Result<String, Error> {
        let endpoint = MonetixHealthcheckEndpoint::PaymentHost;
        let resp = self.get_string(&self.host, endpoint, None).await?;

        Ok(format!("https://{}", resp.trim()))
    }

    pub async fn get_payment_url(&self, args: GetPaymentPageArgs) -> Result<String, Error> {
        let host = self.get_payment_host().await?;
        let query = serde_qs::to_string(&args).unwrap();
        let endpoint = MonetixHealthcheckEndpoint::PaymentUrl;
        let sign = self.signer.generate_sign(&args)?;
        let args = format!("{}?{}&signature={}", String::from(&endpoint), query, sign);
        let encrypted = self.cipher.encrypt(&args)?;
        let url = format!("{}/{}/{}", host, self.project_id, encrypted);

        Ok(url)
    }

    pub async fn get_payment_page_config(&self, args: GetPaymentPageArgs) -> Result<PaymentPageConfig, Error> {
        let host = self.get_payment_host().await?;
        let query = serde_qs::to_string(&args).unwrap();
        let endpoint = MonetixHealthcheckEndpoint::PaymentUrl;
        let signature = self.signer.generate_sign(&args)?;
        let signature = urlencoding::encode(&signature).to_string();
        let args_string = format!("{}?{}&signature={}", String::from(&endpoint), query, signature);
        let encrypted_data = self.cipher.encrypt(&args_string)?;
        
        Ok(PaymentPageConfig {
            host,
            signature,
            encrypted_data,
            payment_id: args.payment_id,
            payment_amount: args.payment_amount,
            payment_currency: args.payment_currency,
            project_id: args.project_id,
            customer_id: args.customer_id,
            customer_first_name: args.customer_first_name,
            customer_last_name: args.customer_last_name,
            customer_email: args.customer_email,
        })
    }

    pub async fn get_payment_sign(&self, args: GetPaymentPageArgs) -> Result<String, Error> {
        let sign = self.signer.generate_sign(&args)?;

        Ok(sign)
    }

    pub async fn get_string(
        &self,
        host: &str,
        endpoint: MonetixHealthcheckEndpoint,
        query: Option<&str>,
    ) -> Result<String, Error> {
        let url = if let Some(query) = query  {
            let args = format!("{}?{}", String::from(&endpoint), query);
            let sign = self.cipher.encrypt(&args)?;
            
            format!("{}/{}/{}", host, self.project_id, sign)
        } else {
            format!("{}{}", host, String::from(&endpoint))
        };

        let headers = self.build_headers();
        let response = self.inner_client.get(&url).headers(headers).send().await?;

        Ok(response.text().await?)
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
