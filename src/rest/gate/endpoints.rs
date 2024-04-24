use http::Method;

pub enum MonetixGateEndpoint {
    CreateInvoicePayment,
    MakePayout(String),
}

impl From<&MonetixGateEndpoint> for String {
    fn from(item: &MonetixGateEndpoint) -> Self {
        match item {
            MonetixGateEndpoint::CreateInvoicePayment => "/v2/payment/invoice/create".to_string(),
            MonetixGateEndpoint::MakePayout(payment_method) => {
                format!("/v2/payment/bank-transfer/{payment_method}/payout")
            }
        }
    }
}

impl MonetixGateEndpoint {
    pub fn get_http_method(&self) -> Method {
        match &self {
            MonetixGateEndpoint::CreateInvoicePayment => Method::POST,
            MonetixGateEndpoint::MakePayout(_) => Method::POST,
        }
    }
}
