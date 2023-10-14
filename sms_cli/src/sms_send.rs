use crate::args_parser::SendSmsArgs;

pub async fn send_sms(send_args: SendSmsArgs) -> Result<(), sms_api::SmsError> {
    let number = send_args
        .to
        .number
        .expect("Currently only number is supported");
    let message = send_args
        .message
        .plain
        .expect("Currently only plain message is supported");
    println!("Sending sms to {} with message '{}'", number, message);
    sms_api::send_sms(&message, &[&number]).await
}
