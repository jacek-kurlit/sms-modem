#[tokio::main]
async fn main() {
    let result = sms_api::send_sms("It works!", &["12345"]).await;
    match result {
        Ok(_) => {
            println!("SMS sent successfully");
        }
        Err(e) => {
            println!("Error sending SMS: {}", e);
        }
    }
}
