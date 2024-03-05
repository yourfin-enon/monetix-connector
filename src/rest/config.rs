#[derive(Clone, Debug)]
pub struct MonetixApiConfig {
    pub rest_api_host: String,
}

impl MonetixApiConfig {
    pub fn sandbox() -> Self {
        todo!();
    }

    pub fn prod() -> Self {
        Self {
            rest_api_host: "https://api.trxhost.com.".into(),
        }
    }
}
