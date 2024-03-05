use crate::rest::config::MonetixApiConfig;
use crate::rest::endpoints::MonetixEndpoint;
use crate::rest::errors::Error;
use crate::rest::models::{
    MonetixBuyAssetRequest, MonetixBuyAssetResponse, MonetixPaymentConfigResponse,
    MonetixPaymentMethodsRequest, MonetixPaymentMethodsResponse, MonetixPlatformConfigResponse,
    MonetixRatesResponse, GetQuoteRequest, GetQuoteResponse,
};
use crate::rest::request_signer::MonetixRequestSigner;
use error_chain::bail;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use reqwest::Response;
use reqwest::StatusCode;
use serde::de::DeserializeOwned;
use std::collections::HashMap;

#[derive(Clone)]
pub struct MonetixRestClient {
    signer: MonetixRequestSigner,
    access_key: String,
    host: String,
    inner_client: reqwest::Client,
    partner_id: String,
}

impl MonetixRestClient {
    pub fn new(
        partner_id: String,
        secret_key: String,
        access_key: String,
        config: MonetixApiConfig,
    ) -> Self {
        Self {
            signer: MonetixRequestSigner::new(secret_key),
            access_key,
            host: config.rest_api_host,
            inner_client: reqwest::Client::new(),
            partner_id,
        }
    }

    pub async fn get_quote(
        &self,
        amount: impl Into<String>,
        crypto_asset: impl Into<String>,
        fiat_asset: impl Into<String>,
        payment_method: impl Into<String>,
        region: impl Into<String>,
    ) -> Result<GetQuoteResponse, Error> {
        let request = GetQuoteRequest {
            amount: amount.into(),
            crypto: crypto_asset.into(),
            fiat: fiat_asset.into(),
            partner_id: self.partner_id.clone(),
            payment: payment_method.into(),
            region: region.into(),
        };
        let query_string = serde_qs::to_string(&request).unwrap(); // todo: handle err
        let resp: GetQuoteResponse = self
            .get_signed(MonetixEndpoint::Quotes, Some(&query_string))
            .await?;

        Ok(resp)
    }

    pub async fn get_rates(&self) -> Result<MonetixRatesResponse, Error> {
        let resp: MonetixRatesResponse = self.get_signed(MonetixEndpoint::Rates, None).await?;

        Ok(resp)
    }

    pub async fn get_payment_methods(
        &self,
        currency_ico: impl Into<String>,
        country_code: impl Into<String>,
    ) -> Result<MonetixPaymentMethodsResponse, Error> {
        let request = MonetixPaymentMethodsRequest {
            currency_iso: currency_ico.into(),
            country_code: country_code.into(),
        };
        let query = serde_qs::to_string(&request).unwrap();
        let resp = self
            .get_signed(MonetixEndpoint::PaymentMethods, Some(&query))
            .await?;

        Ok(resp)
    }

    pub async fn get_payment_config(&self) -> Result<MonetixPaymentConfigResponse, Error> {
        let resp: MonetixPaymentConfigResponse =
            self.get_signed(MonetixEndpoint::PaymentConfig, None).await?;

        Ok(resp)
    }

    pub async fn buy_asset(
        &self,
        params: MonetixBuyAssetParams,
    ) -> Result<MonetixBuyAssetResponse, Error> {
        let request = MonetixBuyAssetRequest {
            amount: params.amount,
            crypto: params.crypto,
            fiat: params.fiat,
            order_custom_id: params.order_custom_id,
            partner_account_id: self.partner_id.clone(),
            payment_method: params.payment_method.to_string(),
            redirect_url: params.redirect_url,
            region: params.region,
            wallet_address: params.wallet_address,
        };
        let query_params = serde_qs::to_string(&request).unwrap(); // todo: handle err
        let endpoint = MonetixEndpoint::BuyAsset;
        let url = format!("{}{}?{}", self.host, String::from(&endpoint), query_params);
        let sign = self.signer.generate_sign(&endpoint);
        let client = &self.inner_client;
        let headers = self.build_headers(Some(&sign));
        let response = client.get(url.as_str()).headers(headers).send().await?;

        if response.status() != StatusCode::OK {
            return self.handler(response, None, &url).await;
        }

        Ok(MonetixBuyAssetResponse {
            redirect_url: response.url().to_string(),
        })
    }

    pub async fn get_platform_config(&self) -> Result<MonetixPlatformConfigResponse, Error> {
        let resp: MonetixPlatformConfigResponse = self
            .get_signed(MonetixEndpoint::PlatformConfig, None)
            .await?;

        Ok(resp)
    }

    pub async fn post_signed<T: DeserializeOwned>(
        &self,
        endpoint: MonetixEndpoint,
        request_json: String,
    ) -> Result<T, Error> {
        let url: String = format!("{}{}", self.host, String::from(&endpoint));
        let sign = self.signer.generate_sign(&endpoint);

        let headers = self.build_headers(Some(&sign));
        let client = &self.inner_client;
        let response = client
            .post(&url)
            .body(request_json.clone())
            .headers(headers)
            .send()
            .await?;

        self.handler(response, Some(request_json), &url).await
    }

    pub async fn get_signed<T: DeserializeOwned>(
        &self,
        endpoint: MonetixEndpoint,
        query_params: Option<&str>,
    ) -> Result<T, Error> {
        let url: String = if let Some(query_params) = query_params {
            format!("{}{}?{}", self.host, String::from(&endpoint), query_params)
        } else {
            format!("{}{}", self.host, String::from(&endpoint))
        };
        let sign = self.signer.generate_sign(&endpoint);

        let client = &self.inner_client;
        let headers = self.build_headers(Some(&sign));
        let response = client.get(url.as_str()).headers(headers).send().await?;

        self.handler(response, None, &url).await
    }

    fn build_headers(&self, sign: Option<&str>) -> HeaderMap {
        let mut custom_headers = HeaderMap::new();

        custom_headers.insert(
            "access-control-allow-headers",
            HeaderValue::from_str("Accept").unwrap(),
        );

        custom_headers.insert(
            "X-merchantid",
            HeaderValue::from_str(self.partner_id.as_str()).unwrap(),
        );

        custom_headers.insert(
            HeaderName::from_static("api-key"),
            HeaderValue::from_str(&self.access_key).unwrap(),
        );

        if let Some(sign) = sign {
            custom_headers.insert(
                HeaderName::from_static("signature"),
                HeaderValue::from_str(sign).unwrap(),
            );
        }

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

#[derive(Debug)]
pub struct MonetixBuyAssetParams {
    pub amount: String,
    pub crypto: String,
    pub fiat: String,
    pub order_custom_id: String,
    pub payment_method: String,
    pub redirect_url: String,
    pub region: String,
    pub wallet_address: String,
}
