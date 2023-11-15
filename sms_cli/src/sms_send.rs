use sms_api::SmsError;
use sms_config::config::SmsApiConf;

use crate::args_parser::SendSmsArgs;

pub async fn send_sms(
    send_args: SendSmsArgs,
    sms_api_config: &SmsApiConf,
) -> Result<String, String> {
    let number = send_args
        .to
        .number
        .ok_or_else(|| "Currently only number is supported".to_string())?;
    let message = send_args
        .message
        .plain
        .expect("Currently only plain message is supported");

    send(&message, &[&number], sms_api_config)
        .await
        .map(|_| "Message sent".to_string())
        .map_err(|e| format!("Could not send message, Reason: {:?}", e))
}

async fn send(
    message: &str,
    numbers: &[&str],
    sms_api_config: &SmsApiConf,
) -> Result<(), SmsError> {
    println!("Sending sms to {:?} with message '{}'", numbers, message);
    sms_api::create_service(sms_api_config)?
        .send_sms(message, numbers)
        .await
}
