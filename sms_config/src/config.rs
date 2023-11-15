use serde::Deserialize;

#[derive(Deserialize, Default, Debug)]
pub struct SmsConfig {
    #[serde(default)]
    pub db: SmsDbConfig,
    #[serde(default)]
    pub sms_api: SmsApiConf,
}

#[derive(Deserialize, Debug)]
pub struct SmsDbConfig {
    pub storage_path: String,
}

impl Default for SmsDbConfig {
    fn default() -> Self {
        Self {
            storage_path: "~/.config/sms_modem/sms_modem.db".to_string(),
        }
    }
}

#[derive(Deserialize, Default, Debug)]
pub struct SmsApiConf {
    #[serde(default)]
    pub provider: SmsApiProvider,
}

#[derive(Deserialize, Default, Debug)]
#[serde(tag = "type", rename_all = "PascalCase")]
pub enum SmsApiProvider {
    #[default]
    Void,
    Alcatel {
        #[serde(default = "default_alcatel_url")]
        url: String,
        #[serde(default = "default_retry_count")]
        retry_count: usize,
        #[serde(default = "default_retry_delay")]
        retry_delay: u64,
    },
}

fn default_alcatel_url() -> String {
    "http://192.168.1.1".to_string()
}

fn default_retry_count() -> usize {
    3
}
fn default_retry_delay() -> u64 {
    500
}
