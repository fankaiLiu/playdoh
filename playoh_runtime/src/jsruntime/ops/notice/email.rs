use deno_core::anyhow::Error;
use deno_core::op;
use lettre::message::Mailbox;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
extern crate lettre;
#[op]
fn op_send_email(msg: String) -> Result<(), Error> {
    let mailbox: Mailbox = "<@.com>".parse()?;
    let now = chrono::Local::now().format("%Y-%m-%d").to_string();
    let subject = " ".to_owned() + &now + "】";
    let email = Message::builder()
        .from("炉火 <15136996437@163.com>".parse().unwrap())
        .reply_to(mailbox.clone())
        .to(mailbox)
        .subject(subject)
        .body(msg)
        .unwrap();
    let creds = Credentials::new(" @.com".to_string(), " ".to_string());
    // Open a remote connection to gmail
    let mailer = SmtpTransport::relay("smtp.163.com")
        .unwrap()
        .credentials(creds)
        .build();
    // Send the email
    match mailer.send(&email) {
        Ok(_) => {
            println!("Email sent successfully!");
            Ok(())
        }
        Err(e) => Err(e.into()),
    }
}
