use http::Method;

pub enum MonetixHealthcheckEndpoint {
    PaymentUrl,
    PaymentHost,

}

impl From<&MonetixHealthcheckEndpoint> for String {
    fn from(item: &MonetixHealthcheckEndpoint) -> Self {
        String::from(match item {
            MonetixHealthcheckEndpoint::PaymentUrl => "/payment",
            MonetixHealthcheckEndpoint::PaymentHost => "/g2",

        })
    }
}

impl MonetixHealthcheckEndpoint {
    pub fn get_http_method(&self) -> Method {
        match &self {
            MonetixHealthcheckEndpoint::PaymentUrl => Method::GET,
            MonetixHealthcheckEndpoint::PaymentHost => Method::GET,

        }
    }
}
