use monetix_connector::rest::config::MonetixApiConfig;
use monetix_connector::rest::models::MonetixPaymentModel;
use monetix_connector::rest::rest_client::MonetixRestClient;
use uuid::Uuid;

#[tokio::main]
async fn main() {
    let project_id = std::env::var("PROJECT_ID").unwrap().parse().unwrap();
    let secret_key = std::env::var("SECRET_KEY").unwrap();
    let callback_url = std::env::var("CALLBACK_URL").ok();

    let client = MonetixRestClient::new(
        project_id,
        secret_key,
        callback_url,
        MonetixApiConfig::prod(),);
        
        let result = client.create_invoice_payment(
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
            }
        ).await;

    println!("{:?}", result);
}
