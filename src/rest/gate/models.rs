use serde::{Deserialize, Serialize};
use crate::rest::signer::{MonetixRequest};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MonetixCreateInvoicePaymentRequest {
    pub general: MonetixGeneralModel,
    pub customer: MonetixCustomerModel,
    pub payment: MonetixPaymentModel,
    pub return_url: MonetixReturnUrlModel,
    /// Card operation type for customer to pay. Default is sale.
    //// Allowed values: sale, auth
    pub card_operation_type: String,
    pub send_email: bool,
}

impl MonetixRequest for MonetixCreateInvoicePaymentRequest {}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MonetixCreateInvoicePaymentResponse {
    #[serde(rename = "status")]
    pub status: String,
    #[serde(rename = "request_id")]
    pub request_id: String,
    #[serde(rename = "project_id")]
    pub project_id: u32,
    #[serde(rename = "payment_id")]
    pub payment_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MonetixGeneralModel {
    pub project_id: u32,
    pub payment_id: String,
    pub merchant_callback_url: Option<String>,
    pub signature: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MonetixCustomerModel {
    pub id: String,
    // customer country in ISO 3166-1 alpha-2 format
    //pub country: Option<String>,
    //pub city: Option<String>,
    //pub state: Option<String>,
    // Phone number of the customer that must have from 4 to 24 digits
    //pub phone: Option<String>,
    // customer day of birth must have format DD-MM-YYYY
    //pub day_of_birth: Option<String>,
    //pub birthplace: Option<String>,
    //pub first_name: Option<String>,
    //pub middle_name: Option<String>,
    //pub last_name: Option<String>,
    // user language (locale), for example en_US
    //pub language: Option<String>,
    //pub address: Option<String>,
    // The last 4 digits of the social security number of US
    //pub ssn: Option<i32>,
    //pub billing: Option<MonetixCustomerBillingModel>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MonetixCustomerBillingModel {
    /// customer country in ISO 3166-1 alpha-2 format
    pub country: Option<String>,
    pub region: Option<String>,
    pub city: Option<String>,
    pub address: Option<String>,
    pub postal: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MonetixPaymentModel {
    /// Payment amount in minor currency units without any decimal point or comma except for cases
    /// where the currency does not have any minor currency units. If the currency have no minor
    /// units (that is, the number of digits for minor currency units is zero), then you must set
    /// this parameter to the amount in the major currency units. For the information on whether
    /// the currency has any minor units, see https://developers.trxhost.com/en/en_currency_codes.html
    pub amount: u64,
    /// Payment currency in ISO 4217 alpha-3 format
    pub currency: String,
    pub description: Option<String>,
    /// Extra payment description
    pub extra_param: Option<String>,
    /// Date and time of payment expiration in format YYYY-MM-DDThh:mm:ss±hh:mm
    pub best_before: String,
    /// Mail Order / Telephone Order payment type:
    /// 0 - not MO/TO payment, 1 - Mail Order, 2 - Telephone Order
    /// Default: 0
    pub moto_type: i32,
    // The ID of the PS, which opens by default without the possibility of selecting another PS.
    //pub force_method: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MonetixReturnUrlModel {
    pub success: Option<String>,
    pub decline: Option<String>,
    #[serde(rename = "return")]
    pub return_url: Option<String>,
}
