use std::time::Duration;

use alcatel::AlcatelSmsService;
use async_trait::async_trait;
use reqwest::StatusCode;
use sms_config::config::{SmsApiConf, SmsApiProvider};
use thiserror::Error;

mod alcatel;
#[cfg(feature = "sms_mock_api")]
pub mod sms_mock_api;
mod void;

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

#[async_trait]
pub trait SmsService {
    async fn send_sms(&self, msg: &str, phone_numbers: &[&str]) -> Result<(), SmsError>;
}

pub fn create_service(sms_api_config: &SmsApiConf) -> Result<Box<dyn SmsService>, SmsError> {
    match &sms_api_config.provider {
        SmsApiProvider::Void => Ok(Box::new(void::VoidSmsService)),
        SmsApiProvider::Alcatel {
            url,
            retry_count,
            retry_delay,
        } => Ok(Box::new(AlcatelSmsService::new(
            url,
            *retry_count,
            Duration::from_millis(*retry_delay),
        )?)),
    }
}
