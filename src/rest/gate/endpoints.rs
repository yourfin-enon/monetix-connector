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
                if payment_method == "nequi" {
                    format!("/v2/payment/wallet/{payment_method}/payout")
                } else if payment_method == "card" {
                    format!("/v2/payment/{payment_method}/payout")
                } else {
                    format!("/v2/payment/bank-transfer/{payment_method}/payout")
                }
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
