use http::Method;

pub enum MonetixEndpoint {
    CreateInvoicePayment,
}

impl From<&MonetixEndpoint> for String {
    fn from(item: &MonetixEndpoint) -> Self {
        String::from(match item {
            MonetixEndpoint::CreateInvoicePayment => "/v2/payment/invoice/create",
        })
    }
}

impl MonetixEndpoint {
    pub fn get_http_method(&self) -> Method {
        match &self {
            MonetixEndpoint::CreateInvoicePayment => Method::POST,
        }
    }
}
