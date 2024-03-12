use serde::{Deserialize, Serialize};
use crate::rest::models::{MonetixCreateInvoicePaymentRequest, MonetixGeneralModel};
use crate::rest::request_signer::{MonetixRequest, MonetixSignPart};

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

impl MonetixSignPart for MonetixCallbackCustomerModel {
    fn add_sign_parts_sorted(&self, prefix: &str, targer: &mut Vec<String>) {
        let mut parts = Vec::with_capacity(2);
        parts.push(format!("{}:id:{}", prefix, self.id));
        parts.push(format!("{}:ip_address:{}", prefix, self.id));

        targer.append(&mut parts);
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MonetixErrorItemModel {}

impl MonetixSignPart for MonetixCallbackPaymentModel {
    fn add_sign_parts_sorted(&self, prefix: &str, targer: &mut Vec<String>) {
        let mut parts = Vec::with_capacity(3);

        targer.append(&mut parts);
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MonetixCallbackPaymentModel {
    pub id: String,
    pub status: String,
    #[serde(rename = "type")]
    pub payment_type: String,
}

impl MonetixSignPart for MonetixCallbackPaymentModel {
    fn add_sign_parts_sorted(&self, prefix: &str, targer: &mut Vec<String>) {
        let mut parts = Vec::with_capacity(3);
        parts.push(format!("{}:id:{}", prefix, self.id));
        parts.push(format!("{}:status:{}", prefix, self.status));
        parts.push(format!("{}:type:{}", prefix, self.payment_type));

        targer.append(&mut parts);
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MonetixRedirectDataModel {
    pub body: Option<String>,
    pub method: Option<String>,
    pub url: Option<String>,
}

impl MonetixSignPart for MonetixRedirectDataModel {
    fn add_sign_parts_sorted(&self, prefix: &str, targer: &mut Vec<String>) {
        let mut parts = Vec::with_capacity(3);

        if let Some(body) = self.body.as_ref() {
            parts.push(format!("{}:body:{}", prefix, body));
        }

        if let Some(method) = self.method.as_ref() {
            parts.push(format!("{}:method:{}", prefix, method));
        }

        if let Some(url) = self.url.as_ref() {
            parts.push(format!("{}:body:{}", prefix, url));
        }

        targer.append(&mut parts);
    }
}

impl MonetixRequest for MonetixCallBackModel {
    fn to_sign_string(&self) -> String {
        let mut parts = Vec::with_capacity(100);

        self.customer.add_sign_parts_sorted("customer", &mut parts);
        self.payment.add_sign_parts_sorted("payment", &mut parts);

        if let Some(redirect_data) = self.redirect_data.as_ref() {
            redirect_data.add_sign_parts_sorted("redirect_data", &mut parts);
        }

        if let Some(errors) = self.errors.as_ref() {
            errors.add_sign_parts_sorted("errors", &mut parts);
        }

        parts.sort();
        parts.join(";")
    }
}