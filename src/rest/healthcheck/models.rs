use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GetPaymentUrlArgs {
    pub payment_id: String,
    pub payment_amount: u32,
    pub payment_currency: String,
    pub project_id: u32,
    pub customer_id: String,
    pub customer_first_name: String,
    pub customer_last_name: String,
    pub customer_email: String,    
}