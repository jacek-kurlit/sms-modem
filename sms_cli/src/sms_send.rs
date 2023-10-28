use sms_api::SmsError;

use crate::args_parser::SendSmsArgs;

pub async fn send_sms(send_args: SendSmsArgs, service_url: &str) -> Result<String, String> {
    let number = send_args
        .to
        .number
        .ok_or_else(|| "Currently only number is supported".to_string())?;
    let message = send_args
        .message
        .plain
        .expect("Currently only plain message is supported");

    send(&message, &[&number], service_url)
        .await
        .map(|_| "Message sent".to_string())
        .map_err(|e| format!("Could not send message, Reason: {:?}", e))
}

async fn send(message: &str, numbers: &[&str], service_url: &str) -> Result<(), SmsError> {
    println!("Sending sms to {:?} with message '{}'", numbers, message);
    sms_api::SmsService::new(service_url)?
        .send_sms(message, numbers)
        .await
}
