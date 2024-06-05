use monetix_connector::rest::gate::models::MonetixPaymentModel;
use monetix_connector::rest::gate::payout::{
    MonetixCardModel, MonetixCustomerAccountModel, MonetixCustomerIdentifyModel,
    MonetixCustomerPayoutModel, MonetixPayoutPaymentModel,
};
use monetix_connector::rest::gate::rest_client::MonetixGateRestClient;
use monetix_connector::rest::healthcheck::models::GetPaymentPageArgs;
use monetix_connector::rest::healthcheck::rest_client::MonetixHealthcheckRestClient;
use monetix_connector::rest::payment_page::PaymentPage;
use uuid::Uuid;

#[tokio::main]
async fn main() {
    make_card_payout().await;
}

pub async fn make_card_payout() {
    let project_id = std::env::var("PROJECT_ID").unwrap().parse().unwrap();
    let secret_key = std::env::var("SECRET_KEY").unwrap();
    let callback_url = std::env::var("CALLBACK_URL").ok();

    let gate_client = MonetixGateRestClient::new(
        project_id,
        secret_key,
        "google.com".to_string(),
        callback_url,
    );
    let customer = create_customer_model();
    let card = MonetixCardModel {
        pan: "4000000000000077".to_string(),
    };
    let payment = MonetixPayoutPaymentModel {
        amount: 1000000,
        currency: "MXN".to_string(),
    };

    let result = gate_client
        .make_card_payout(generate_payment_id(), customer, card, payment)
        .await;
    println!("{:?}", result);
}

pub async fn make_payout() {
    let project_id = std::env::var("PROJECT_ID").unwrap().parse().unwrap();
    let secret_key = std::env::var("SECRET_KEY").unwrap();
    let callback_url = std::env::var("CALLBACK_URL").ok();

    let gate_client = MonetixGateRestClient::new(
        project_id,
        secret_key,
        "google.com".to_string(),
        callback_url,
    );
    let customer = create_customer_model();
    let account = MonetixCustomerAccountModel {
        account_type: None,
        bank_id: None,
        number: "380506666666".to_string(),
    };
    let payment = MonetixPayoutPaymentModel {
        amount: 1000000,
        currency: "MXN".to_string(),
    };

    let payment_method = "colombia"; // spei, nequi (has other endpoint), colombia

    let result = gate_client
        .make_payout(
            generate_payment_id(),
            payment_method,
            customer,
            account,
            payment,
        )
        .await;
    println!("{:?}", result);
}

pub async fn payment_page() {
    let project_id = std::env::var("PROJECT_ID").unwrap().parse().unwrap();
    let secret_key = std::env::var("SECRET_KEY").unwrap();
    let callback_url = std::env::var("CALLBACK_URL").ok();
    let encryption_key = std::env::var("ENCRYPTION_KEY").unwrap();
    let healthcheck_url = std::env::var("HEALTHCHECK_URL").unwrap();

    let client =
        MonetixHealthcheckRestClient::new(project_id, &secret_key, encryption_key, healthcheck_url);

    let result = client.get_payment_host().await;

    //println!("get_payment_host: {:?}", result);

    let payment_args = GetPaymentPageArgs {
        payment_id: generate_payment_id(),
        payment_amount: 10,
        payment_currency: "USD".to_string(),
        project_id,
        customer_id: "test".to_string(),
        customer_first_name: "test".to_string(),
        customer_last_name: "test".to_string(),
        customer_email: "test@test.com".to_string(),
    };

    //println!("{:?}", payment_args);
    let result = client.get_payment_sign(payment_args.clone()).await;

    //println!("get_payment_sign: {:?}", result);

    let result = client.get_payment_url(payment_args.clone()).await;
    //println!("get_payment_url: {:?}", result);

    let result = client.get_payment_page_config(payment_args).await;
    //println!("get_payment_page_config: {:?}", result);

    let payment_page = PaymentPage::new(result.unwrap());

    println!("{}", payment_page.to_html());
}

pub async fn create_invoice() {
    let project_id = std::env::var("PROJECT_ID").unwrap().parse().unwrap();
    let secret_key = std::env::var("SECRET_KEY").unwrap();
    let callback_url = std::env::var("CALLBACK_URL").ok();

    let gate_client = MonetixGateRestClient::new(
        project_id,
        secret_key,
        "google.com".to_string(),
        callback_url,
    );
    let result = gate_client
        .create_invoice_payment(
            format!("test-{}", Uuid::new_v4()),
            "test-customer-id-1",
            MonetixPaymentModel {
                amount: 5,
                currency: "USD".to_string(),
                description: Some("test".into()),
                extra_param: Some("test".into()),
                best_before: "2050-01-01T00:00:00+00:00".to_string(),
                moto_type: 0,
                //force_method: None,
            },
        )
        .await;

    println!("{:?}", result);
}

pub fn create_customer_model() -> MonetixCustomerPayoutModel {
    MonetixCustomerPayoutModel {
        id: "test-customer".to_string(),
        ip_address: "192.168.1.1".to_string(),
        first_name: "test".to_string(),
        last_name: "test".to_string(),
        identify: MonetixCustomerIdentifyModel {
            doc_type: Some("CURP".to_string()),
            doc_number: "123456789123456789".to_string(),
        },
        email: "fadfadfdassf@gmail.com".to_string(),
    }
}

pub fn generate_payment_id() -> String {
    format!("test:{}", Uuid::new_v4())
}
