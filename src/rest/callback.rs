use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MonetixCallBackModel {
    /*
    /// The object that contains the details of the customer bank card or other payment account
    pub account: Option<MonetixAccountModel>,
    /// The object that contains the result of the customer authentication by using 3‑D Secure.
    /// This object is available in callback, if the payment was made with the card that supports 3‑D Secure.
    pub acs: Option<MonetixAcsModel>,
    /// The object that contains the results of the AVS matching (Address Verification Service).
    /// For more information, see Checking with Address Verification Service
    pub avs_data: Option<MonetixAvsDataModel>,
    /// Result of the Address Verification Service (AVS) matching.
    /// For more information, see Checking with Address Verification Service
    pub avs_result: Option<String>,
    /// The object that contains the data about a card issuer bank.
    pub bank: Option<MonetixBankModel>,
    */
    pub customer: MonetixCallbackCustomerModel,
    pub errors: Option<Vec<MonetixErrorItemModel>>,
    pub payment: MonetixCallbackPaymentModel,
    pub redirect_data: Option<MonetixRedirectDataModel>,
    pub signature: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MonetixAccountModel {}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MonetixAcsModel {}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MonetixAvsDataModel {}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MonetixBankModel {}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MonetixCallbackCustomerModel {
    pub id: String,
    pub ip_address: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MonetixErrorItemModel {}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MonetixCallbackPaymentModel {
    pub id: String,
    pub status: String,
    #[serde(rename = "type")]
    pub payment_type: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MonetixRedirectDataModel {
    pub body: Option<String>,
    pub method: Option<String>,
    pub url: Option<String>,
}
