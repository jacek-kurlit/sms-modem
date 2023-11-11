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
#[serde(tag="type", rename_all = "PascalCase")]
pub enum SmsApiProvider {
    #[default]
    Mock,
    Alcatel {
        host: String,
    },
}
