use crate::rest::gate::models::{MonetixCreateInvoicePaymentRequest, MonetixGeneralModel};
use crate::rest::signer::MonetixRequest;
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MonetixPayoutRequest {
    pub general: MonetixGeneralModel,
    pub customer: MonetixCustomerPayoutModel,
    pub account: MonetixCustomerAccountModel,
    pub payment: MonetixPayoutPaymentModel,
}

impl MonetixRequest for MonetixPayoutRequest {}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MonetixCustomerPayoutModel {
    pub id: String,
    pub ip_address: String,
    pub first_name: String,
    pub last_name: String,
    pub identify: MonetixCustomerIdentifyModel,
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MonetixCustomerIdentifyModel {
    /// CURP — при передаче в параметре doc_number идентификатора, который присваивается гражданам Мексики и иностранным гражданам, проживающим на территории Мексики (Clave Única de Registro de Población);
    /// RFC — при передаче в параметре doc_number идентификатора налогоплательщика в Мексике (Registro Federal de Contribuyentes).
    pub doc_type: String,
    pub doc_number: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MonetixCustomerAccountModel {
    /// CLABE — при передаче в параметре number номера счета CLABE получателя выплаты;
    /// PHONE — при передаче в параметре number номера телефона пользователя;
    /// DEBIT — при передаче в параметре number номера платежной карты пользователя.
    #[serde(rename = "type")]
    pub account_type: String,
    pub bank_id: Option<i32>,
    pub number: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MonetixPayoutPaymentModel {
    pub amount: u64,
    pub currency: String,
}
