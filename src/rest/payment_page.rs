use crate::rest::healthcheck::models::PaymentPageConfig;

pub struct PaymentPage {
    html_template: &'static str,
    config: PaymentPageConfig,
}

impl PaymentPage {
    pub fn new(config: PaymentPageConfig) -> Self {
        Self {
            html_template: "<html><head><link rel='stylesheet' href='TEMPLATE_PAYMENT_HOST/shared/merchant.css'><script src='TEMPLATE_PAYMENT_HOST/shared/merchant.js'></script></head><body><div class='container'><div class='cart-info'>...</div><div id='widget-container'></div></div><script type='text/javascript'>var configObj={payment_id:'TEMPLATE_PAYMENT_ID',payment_amount:TEMPLATE_PAYMENT_AMOUNT,payment_currency:'TEMPLATE_PAYMENT_CURRENCY',project_id:TEMPLATE_PROJECT_ID,customer_id:'TEMPLATE_CUSTOMER_ID',customer_first_name:'TEMPLATE_CUSTOMER_FIRST_NAME',customer_last_name:'TEMPLATE_CUSTOMER_LAST_NAME',customer_email:'TEMPLATE_CUSTOMER_EMAIL',target_element:'widget-container',signature:'TEMPLATE_SIGNATURE'},widget=EPayWidget.create(configObj),url=widget.buildUrl(),encryptedUrl='/TEMPLATE_PROJECT_ID/TEMPLATE_ENCRYPTED_DATA';widget.setEncryptedURL(encryptedUrl).run()</script></body></html>",
            config,
        }
    }

    pub fn to_html(&self) -> String {
        self.replace_template_variables()
    }

    fn replace_template_variables(&self) -> String {
        // todo: allocate only once
        let mut html_template = self.html_template.replace("TEMPLATE_PAYMENT_HOST", &self.config.host);
        html_template = html_template.replace("TEMPLATE_PAYMENT_ID", &self.config.payment_id);
        html_template = html_template.replace("TEMPLATE_PAYMENT_AMOUNT", &self.config.payment_amount.to_string());
        html_template = html_template.replace("TEMPLATE_PAYMENT_CURRENCY", &self.config.payment_currency);
        html_template = html_template.replace("TEMPLATE_PROJECT_ID", &self.config.project_id.to_string());
        html_template = html_template.replace("TEMPLATE_CUSTOMER_ID", &self.config.customer_id);
        html_template = html_template.replace("TEMPLATE_CUSTOMER_FIRST_NAME", &self.config.customer_first_name);
        html_template = html_template.replace("TEMPLATE_CUSTOMER_LAST_NAME", &self.config.customer_last_name);
        html_template = html_template.replace("TEMPLATE_CUSTOMER_EMAIL", &self.config.customer_email);
        html_template = html_template.replace("TEMPLATE_SIGNATURE", &self.config.signature);
        html_template = html_template.replace("TEMPLATE_ENCRYPTED_DATA", &self.config.encrypted_data);

        html_template
    }
}
