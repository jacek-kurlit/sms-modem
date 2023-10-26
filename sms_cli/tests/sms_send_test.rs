use sms_api::sms_mock_api::{self, mockito};
use sms_cli::args_parser::{SendSmsArgs, SmsMessageArgs, SmsTargetArgs};

#[tokio::test]
async fn should_send_sms_successfully() {
    // given
    let mut server = mockito::Server::new_async().await;
    let mock_handler = sms_mock_api::sending_sms_is_successful(&mut server).await;
    let send_args = SendSmsArgs {
        to: SmsTargetArgs {
            number: Some("123456789".to_string()),
            contact_alias: None,
            group_alias: None,
        },
        message: SmsMessageArgs {
            plain: Some("Hello world".to_string()),
            template: None,
        },
    };

    // when
    sms_cli::sms_send::send_sms(send_args, &server.url()).await;

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
            contact_alias: None,
            group_alias: None,
        },
        message: SmsMessageArgs {
            plain: Some("Hello world".to_string()),
            template: None,
        },
    };

    // when
    sms_cli::sms_send::send_sms(send_args, &server.url()).await;

    // then
    // FIXME: I have changed api and we do not return error now because evry method hndler know what to do with errors (tey just print it)
    // Either we need single behaviour so that all handlers will return result and main function will print it
    // or we keep it as it is
    // assert!(matches!(
    //     result,
    //     Err(SmsError::UnknownError(x)) if x ==
    //         "Service didn't confirmed successful send"
    // ));
    mock_handler.assert_called();
}
