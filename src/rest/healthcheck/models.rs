use serde_derive::{Deserialize, Serialize};
use crate::rest::signer::MonetixRequest;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GetPaymentPageArgs {
    pub payment_id: String,
    pub payment_amount: u64,
    pub payment_currency: String,
    pub project_id: u32,
    pub customer_id: String,
    pub customer_first_name: String,
    pub customer_last_name: String,
    pub customer_email: String,    
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PaymentPageConfig {
    pub host: String,
    pub signature: String,
    pub encrypted_data: String,
    pub payment_id: String,
    pub payment_amount: u64,
    pub payment_currency: String,
    pub project_id: u32,
    pub customer_id: String,
    pub customer_first_name: String,
    pub customer_last_name: String,
    pub customer_email: String,
}


impl MonetixRequest for GetPaymentPageArgs {}