use sms_api::sms_mock_api::{self, mockito};
use sms_cli::args_parser::{SendSmsArgs, SmsMessageArgs, SmsTargetArgs};
use sms_config::config::SmsApiConf;

#[tokio::test]
async fn should_send_sms_successfully() {
    // given
    let mut server = mockito::Server::new_async().await;
    let mock_handler = sms_mock_api::sending_sms_is_successful(&mut server).await;
    let send_args = SendSmsArgs {
        to: SmsTargetArgs {
            number: Some("123456789".to_string()),
            contact_name: None,
            group_name: None,
        },
        message: SmsMessageArgs {
            plain: Some("Hello world".to_string()),
            template: None,
        },
    };
    let sms_api_config = SmsApiConf {
        provider: sms_config::config::SmsApiProvider::Alcatel {
            url: server.url(),
            retry_count: 3,
            retry_delay: 50,
        },
    };

    // when
    sms_cli::sms_send::send_sms(send_args, &sms_api_config)
        .await
        .expect("send_sms_successfully");

    // then
    mock_handler.assert_called();
}

#[tokio::test]
async fn should_fail_when_sending_sms() {
    // given
    let mut server = mockito::Server::new_async().await;
    let mock_handler = sms_mock_api::sending_sms_failure(&mut server).await;
    let send_args = SendSmsArgs {
        to: SmsTargetArgs {
            number: Some("123456789".to_string()),
            contact_name: None,
            group_name: None,
        },
        message: SmsMessageArgs {
            plain: Some("Hello world".to_string()),
            template: None,
        },
    };
    let sms_api_config = SmsApiConf {
        provider: sms_config::config::SmsApiProvider::Alcatel {
            url: server.url(),
            retry_count: sms_mock_api::MAX_RETRIES,
            retry_delay: 50,
        },
    };

    // when
    let result = sms_cli::sms_send::send_sms(send_args, &sms_api_config).await;

    // then
    assert!(matches!(
        result,
        Err(output) if output.contains(
            "Service didn't confirmed successful send")
    ));
    mock_handler.assert_called();
}
