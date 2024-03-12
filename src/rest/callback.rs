use serde::{Deserialize, Serialize};
use crate::rest::request_signer::{MonetixRequest, MonetixSignPart};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MonetixCallbackModel {
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
    pub customer: MonetixCallbackCustomerModel,
    /// The array of strings with the messages from the Risk Control System related to the decision regarding the payment.
    ///
    /// Example: reject.message("RCS reject. Amount less than allowed").
    pub decision_message: Option<Vec<String>>,
    pub decision: Option<String>,
    /// The object that contains the data from the payment provider that are required to display
    /// QR code with payment details to the customer.
    pub display_data: Option<String>,
    pub errors: Option<Vec<MonetixErrorItemModel>>,
    pub interface_type: Option<MonetixInterfaceTypeModel>,
    /// The object that contains information about the operation that triggered the callback
    pub operation: Option<MonetixOperationModel>,
    pub payment: MonetixCallbackPaymentModel,
    pub project_id: i32,
    pub redirect_data: Option<MonetixRedirectDataModel>,
    pub signature: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MonetixOperationModel {
    pub code: Option<String>,
    /// Date and time the operation was created.
    ///
    /// Example: 2017-07-27T15:19:13+0000
    pub created_date: Option<String>,
    /// The indicator that shows the result of the 3‑D Secure customer authentication. For more information, see Electronic Commerce Indicator (ECI) codes
    pub eci: Option<String>,
    pub id: Option<i32>,
    pub message: Option<String>,
    pub provider: Option<MonetixOperationProviderModel>,
    pub request_id: String,
    pub status: String,
    pub sum_converted: Option<MonetixSumModel>,
    pub sum_initial: Option<MonetixSumModel>,
    #[serde(rename = "type")]
    pub op_type: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MonetixOperationProviderModel {
    pub auth_code: Option<String>,
    pub date: Option<String>,
    /// CRC32 ID of the external provider gate.
    ///
    /// In some cases, because of payment system or provider requirements,
    /// the type of this parameter may be integer.
    pub endpoint_id: Option<i32>,
    pub id: Option<i32>,
    pub payment_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MonetixSumModel {
    /// The amount in minor units of the payment provider currency
    pub currency: Option<String>,
    /// The currency of the payment provider account in ISO 4217 alpha-3 format.
    ///
    /// Example: EUR
    pub amount: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MonetixInterfaceTypeModel {
    /// Parameter that specifies the interface the payment request originates from. Possible values:
    /// 1—the request issued by using API Gate
    /// 2–4— the request is issued by Monetix
    /// 5—the request is issued by using Dashboard
    /// 6—the request is issued by Payment Page in modal window
    /// 7—the request is issued by Payment Page in iframe
    pub id: Option<i32>,
    pub user: Option<String>,
}


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MonetixAccountModel {
    pub card_holder: Option<String>,
    pub expiry_month: Option<String>,
    pub expiry_year: Option<String>,
    pub id: Option<i32>,
    /// Masked bank card or other account number.
    pub number: String,
    pub token: Option<String>,
    #[serde(rename = "type")]
    pub a_type: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MonetixAcsModel {
    /// URL of the issuing bank ACS page.
    pub acs_url: String,
    /// Merchant technical data in the payment system.
    pub md: String,
    /// The authentication request that need to be sent to the issuing bank. The parameter contains
    /// encoded information about the card holder, the merchant, and the payment.
    pub pa_req: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MonetixAvsDataModel {
    pub avs_post_code: Option<String>,
    pub avs_street_address: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MonetixBankModel {
    pub name: Option<String>,
}

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
pub struct MonetixErrorItemModel {
    pub code: Option<i32>,
    pub description: Option<String>,
    pub field: Option<String>,
    pub message: Option<String>,
}

impl MonetixSignPart for MonetixErrorItemModel {
    fn add_sign_parts_sorted(&self, _prefix: &str, targer: &mut Vec<String>) {
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

impl MonetixRequest for MonetixCallbackModel {
    fn to_sign_string(&self) -> String {
        let mut parts = Vec::with_capacity(100);

        self.customer.add_sign_parts_sorted("customer", &mut parts);
        self.payment.add_sign_parts_sorted("payment", &mut parts);

        if let Some(redirect_data) = self.redirect_data.as_ref() {
            redirect_data.add_sign_parts_sorted("redirect_data", &mut parts);
        }

        if let Some(_errors) = self.errors.as_ref() {
            // errors.add_sign_parts_sorted("errors", &mut parts);
            // todo
        }

        parts.sort();
        parts.join(";")
    }
}