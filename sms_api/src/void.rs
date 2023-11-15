use async_trait::async_trait;

use crate::{SmsError, SmsService};

pub(crate) struct VoidSmsService;

#[async_trait]
impl SmsService for VoidSmsService {
    async fn send_sms(&self, _msg: &str, _phone_numbers: &[&str]) -> Result<(), SmsError> {
        Ok(())
    }
}
