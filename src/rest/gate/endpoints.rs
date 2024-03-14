use http::Method;

pub enum MonetixGateEndpoint {
    CreateInvoicePayment,
}

impl From<&MonetixGateEndpoint> for String {
    fn from(item: &MonetixGateEndpoint) -> Self {
        String::from(match item {
            MonetixGateEndpoint::CreateInvoicePayment => "/v2/payment/invoice/create",
        })
    }
}

impl MonetixGateEndpoint {
    pub fn get_http_method(&self) -> Method {
        match &self {
            MonetixGateEndpoint::CreateInvoicePayment => Method::POST,
        }
    }
}
