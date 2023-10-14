use std::time::Duration;

use reqwest::{Client, Response, StatusCode};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[cfg(feature = "sms_mock_api")]
pub mod sms_mock_api;

pub const RETRY_COUNT: usize = 3;
pub const RETRY_DELAY: Duration = Duration::from_millis(500);

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

pub struct SmsService {
    url: String,
    client: Client,
}

impl SmsService {
    pub fn new(url: &str) -> Result<Self, SmsError> {
        Ok(Self {
            url: url.to_string(),
            client: create_client(url)?,
        })
    }

    pub async fn send_sms(&self, msg: &str, phone_numbers: &[&str]) -> Result<(), SmsError> {
        for phone in phone_numbers {
            self.send_single_sms(msg, phone).await?;
        }
        Ok(())
    }

    async fn send_single_sms(&self, msg: &str, phone: &str) -> Result<(), SmsError> {
        self.call_sms_send(&msg, &phone).await?;
        self.wait_until_sent().await?;
        Ok(())
    }

    async fn call_sms_send(&self, msg: &&str, phone: &&str) -> Result<(), SmsError> {
        let res = self
            .client
            .post(format!("{}/jrd/webapi?api=SendSMS", self.url))
            .json(&SendSmsRequest::new(
                msg.to_string(),
                vec![phone.to_string()],
            ))
            .send()
            .await
            .map_err(|e| SmsError::NetworkError(e.to_string()))?;
        self.ensure_status_is_success(res).await?;
        Ok(())
    }

    async fn wait_until_sent(&self) -> Result<(), SmsError> {
        let mut current_try = 0;
        while current_try < RETRY_COUNT {
            let response = self
                .client
                .post(format!("{}/jrd/webapi?api=GetSendSMSResult", self.url))
                .body(r#"{"jsonrpc":"2.0","method":"GetSendSMSResult","params":null,"id":"6.7"}"#)
                .send()
                .await
                .map_err(|e| SmsError::NetworkError(e.to_string()))?;
            let status_code = self
                .ensure_status_is_success(response)
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

    async fn ensure_status_is_success(&self, response: Response) -> Result<Response, SmsError> {
        let status = response.status();
        match status {
        StatusCode::OK => Ok(response),
        StatusCode::UNAUTHORIZED => Err(SmsError::InvalidResponse(status, "Unauthorized".into())),
        StatusCode::NOT_FOUND => {
            Err(SmsError::InvalidResponse(status, format!("We could not find service under url {}, make sure usb modem is connected and service is running", self.url)))
        }
        _ => {
            Err(SmsError::UnknownError(format!(
                "Unexpected status code: {}",
                status
            )))
        }
    }
    }
}

fn create_client(url: &str) -> Result<Client, SmsError> {
    let mut default_headers = reqwest::header::HeaderMap::new();
    default_headers.insert("Referer", format!("{}/default.html", url).parse().unwrap());

    Client::builder()
        .timeout(Duration::from_secs(10))
        .default_headers(default_headers)
        .build()
        .map_err(|e| SmsError::UnknownError(e.to_string()))
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
