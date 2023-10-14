use std::time::Duration;

use reqwest::{Client, Response, StatusCode};
use serde::{Deserialize, Serialize};
use thiserror::Error;

const SERVICE_URL: &str = "http://192.168.1.1";

#[derive(Error, Debug)]
pub enum SmsError {
    #[error("Something went wrong {0}")]
    UnknownError(String),
    #[error("Network error: {0}")]
    NetworkError(String),
    #[error("Invalid response: status {0}, message {1}")]
    InvalidResponse(StatusCode, String),
    #[error("Could not parse json")]
    ResponseParseError(#[from] reqwest::Error),
}

pub async fn send_sms(msg: &str, phone_numbers: &[&str]) -> Result<(), SmsError> {
    let client = create_client()?;
    for phone in phone_numbers {
        send_single_sms(msg, phone, &client).await?;
    }
    Ok(())
}

fn create_client() -> Result<Client, SmsError> {
    let mut default_headers = reqwest::header::HeaderMap::new();
    default_headers.insert(
        "Referer",
        format!("{}/default.html", SERVICE_URL).parse().unwrap(),
    );

    Client::builder()
        .timeout(Duration::from_secs(10))
        .default_headers(default_headers)
        .build()
        .map_err(|e| SmsError::UnknownError(e.to_string()))
}

async fn send_single_sms(msg: &str, phone: &str, client: &Client) -> Result<(), SmsError> {
    call_sms_send(&msg, &phone, client).await?;
    wait_until_sent(client).await?;
    Ok(())
}

async fn call_sms_send(msg: &&str, phone: &&str, client: &Client) -> Result<(), SmsError> {
    let res = client
        .post(format!("{}/jrd/webapi?api=SendSMS", SERVICE_URL))
        .json(&SendSmsRequest::new(
            msg.to_string(),
            vec![phone.to_string()],
        ))
        .send()
        .await
        .map_err(|e| SmsError::NetworkError(e.to_string()))?;
    ensure_status_is_success(res).await?;
    Ok(())
}

const RETRY_COUNT: usize = 3;
const RETRY_DELAY: Duration = Duration::from_millis(500);

async fn wait_until_sent(client: &Client) -> Result<(), SmsError> {
    let mut current_try = 0;
    while current_try < RETRY_COUNT {
        let response = client
            .post(format!("{}/jrd/webapi?api=GetSendSMSResult", SERVICE_URL))
            .body(r#"{"jsonrpc":"2.0","method":"GetSendSMSResult","params":null,"id":"6.7"}"#)
            .send()
            .await
            .map_err(|e| SmsError::NetworkError(e.to_string()))?;
        let status_code = ensure_status_is_success(response)
            .await?
            .json::<GetSendSmsResultResponse>()
            .await?
            .result
            .send_status;
        if status_code == 2 {
            return Ok(());
        }
        tokio::time::sleep(RETRY_DELAY).await;
        current_try += 1;
    }
    Err(SmsError::UnknownError(
        "Service didn't confirmed successful send".into(),
    ))
}

async fn ensure_status_is_success(response: Response) -> Result<Response, SmsError> {
    let status = response.status();
    match status {
        StatusCode::OK => Ok(response),
        StatusCode::UNAUTHORIZED => Err(SmsError::InvalidResponse(status, "Unauthorized".into())),
        StatusCode::NOT_FOUND => {
            Err(SmsError::InvalidResponse(status, format!("We could not find service under url {}, make sure usb modem is connected and service is running", SERVICE_URL)))
        }
        _ => {
            Err(SmsError::UnknownError(format!(
                "Unexpected status code: {}",
                status
            )))
        }
    }
}

#[derive(Serialize, Debug)]
struct SendSmsRequest {
    id: String,
    jsonrpc: String,
    method: String,
    params: SendSmsParams,
}

impl SendSmsRequest {
    fn new(msg: String, phone_numbers: Vec<String>) -> Self {
        let current_time = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        SendSmsRequest {
            id: "6.6".into(),
            jsonrpc: "2.0".into(),
            method: "SendSMS".into(),
            params: SendSmsParams {
                sms_id: -1,
                sms_content: msg,
                phone_number: phone_numbers,
                sms_time: current_time,
            },
        }
    }
}

#[derive(Serialize, Debug)]
struct SendSmsParams {
    #[serde(rename = "SMSId")]
    sms_id: i8,
    #[serde(rename = "SMSContent")]
    sms_content: String,
    #[serde(rename = "PhoneNumber")]
    phone_number: Vec<String>,
    #[serde(rename = "SMSTime")]
    sms_time: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct GetSendSmsResultResponse {
    pub jsonrpc: String,
    pub result: SendStatus,
    pub id: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct SendStatus {
    #[serde(rename = "SendStatus")]
    pub send_status: i8,
}
