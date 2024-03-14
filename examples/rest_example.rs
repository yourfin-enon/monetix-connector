use monetix_connector::rest::gate::models::MonetixPaymentModel;
use monetix_connector::rest::gate::rest_client::MonetixGateRestClient;
use monetix_connector::rest::healthcheck::models::GetPaymentUrlArgs;
use monetix_connector::rest::healthcheck::rest_client::MonetixHealthcheckRestClient;
use uuid::Uuid;

#[tokio::main]
async fn main() {
    let project_id = std::env::var("PROJECT_ID").unwrap().parse().unwrap();
    let secret_key = std::env::var("SECRET_KEY").unwrap();
    let callback_url = std::env::var("CALLBACK_URL").ok();
    let encryption_key = std::env::var("ENCRYPTION_KEY").unwrap();
    let healthcheck_url = std::env::var("HEALTHCHECK_URL").unwrap();

    let gate_client =
        MonetixHealthcheckRestClient::new(project_id, encryption_key, healthcheck_url);

    let result = gate_client
        .get_payment_url(GetPaymentUrlArgs {
            payment_id: format!("test-{}", Uuid::new_v4()),
            payment_amount: 10,
            payment_currency: "USD".to_string(),
            project_id,
            customer_id: "test".to_string(),
            customer_first_name: "test".to_string(),
            customer_last_name: "test".to_string(),
            customer_email: "test@test.com".to_string(),
        })
        .await;

    println!("{:?}", result);

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
