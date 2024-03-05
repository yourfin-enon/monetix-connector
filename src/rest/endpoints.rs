use http::Method;

pub enum MonetixEndpoint {
    PlatformConfig,
    Quotes,
    BuyAsset,
    Rates,
    PaymentConfig,
    PaymentMethods,
}

impl From<&MonetixEndpoint> for String {
    fn from(item: &MonetixEndpoint) -> Self {
        String::from(match item {
            MonetixEndpoint::PlatformConfig => "/onramp/v1/configuration",
            MonetixEndpoint::Quotes => "/onramp/v1/quotes",
            MonetixEndpoint::BuyAsset => "/onramp/v1/buy",
            MonetixEndpoint::Rates => "/api/v1/rates",
            MonetixEndpoint::PaymentConfig => "/api/v1/config",
            MonetixEndpoint::PaymentMethods => "/api/v1/methods/currencies",
        })
    }
}

impl MonetixEndpoint {
    pub fn get_http_method(&self) -> Method {
        match &self {
            MonetixEndpoint::PlatformConfig => Method::GET,
            MonetixEndpoint::Quotes => Method::GET,
            MonetixEndpoint::BuyAsset => Method::GET,
            MonetixEndpoint::Rates => Method::GET,
            MonetixEndpoint::PaymentConfig => Method::GET,
            MonetixEndpoint::PaymentMethods => Method::GET,
        }
    }
}
