use monetix_connector::rest::config::MonetixApiConfig;
use monetix_connector::rest::models::MonetixPaymentModel;
use monetix_connector::rest::rest_client::MonetixRestClient;

#[tokio::main]
async fn main() {
    let client = MonetixRestClient::new(
        0,
        "test".to_owned(),
        "test".to_owned(),
        "test".to_owned(),
        MonetixApiConfig::sandbox(),);
        
        let result = client.create_invoice_payment(
            "",
            "",
            MonetixPaymentModel {
                amount: 0,
                currency: "".to_string(),
                description: None,
                extra_param: None,
                best_before: "".to_string(),
                moto_type: None,
                force_method: None,
            }
        ).await;
    
    println!("{:?}", result);
}
