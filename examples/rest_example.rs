use monetix_connector::rest::config::MonetixApiConfig;
use monetix_connector::rest::models::MonetixPaymentModel;
use monetix_connector::rest::rest_client::MonetixRestClient;

#[tokio::main]
async fn main() {
    let project_id = std::env::var("PROJECT_ID").unwrap().parse().unwrap();
    let secret_key = std::env::var("SECRET_KEY").unwrap();
    let app_token = std::env::var("APP_TOKEN").unwrap();
    let callback_url = std::env::var("CALLBACK_URL").unwrap();

    let client = MonetixRestClient::new(
        project_id,
        secret_key,
        app_token,
        callback_url,
        MonetixApiConfig::prod(),);
        
        let result = client.create_invoice_payment(
            "test-payment-id-1",
            "test-customer-id-1",
            MonetixPaymentModel {
                amount: 5,
                currency: "USD".to_string(),
                description: Some("test".into()),
                extra_param: None,
                best_before: "2050-01-01T00:00:00±00:00".to_string(),
                moto_type: None,
                force_method: None,
            }
        ).await;
    
    println!("{:?}", result);
}
