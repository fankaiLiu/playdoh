use askama::Template;

#[derive(Template)]
#[template(path = "alerts/success_alert_with_button.html")]
pub struct SuccessAlertWithButtonTemplate {
    pub msg:String
}