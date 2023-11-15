pub use mockito;

pub async fn sending_sms_is_successful(server: &mut mockito::Server) -> AlcatelMock {
    let mock_send = server
        .mock("POST", "/jrd/webapi?api=SendSMS")
        .with_status(200)
        .with_header("content-type", "application/json")
        .create_async()
        .await;
    let mock_get_status = server
        .mock("POST", "/jrd/webapi?api=GetSendSMSResult")
        .with_status(200)
        .with_body(r#"{ "jsonrpc": "2.0", "result": { "SendStatus": 2 }, "id": "6.7" }"#)
        .with_header("content-type", "application/json")
        .create_async()
        .await;

    AlcatelMock {
        mock_send,
        mock_get_status,
    }
}

pub const MAX_RETRIES: usize = 3;

pub async fn sending_sms_failure(server: &mut mockito::Server) -> AlcatelMock {
    let mock_send = server
        .mock("POST", "/jrd/webapi?api=SendSMS")
        .with_status(200)
        .with_header("content-type", "application/json")
        .create_async()
        .await;
    let mock_get_status = server
        .mock("POST", "/jrd/webapi?api=GetSendSMSResult")
        .with_status(200)
        .with_body(r#"{ "jsonrpc": "2.0", "result": { "SendStatus": 1 }, "id": "6.7" }"#)
        .with_header("content-type", "application/json")
        .expect(MAX_RETRIES)
        .create_async()
        .await;

    AlcatelMock {
        mock_send,
        mock_get_status,
    }
}
pub struct AlcatelMock {
    mock_send: mockito::Mock,
    mock_get_status: mockito::Mock,
}

impl AlcatelMock {
    pub fn assert_called(&self) {
        self.mock_send.assert();
        self.mock_get_status.assert();
    }
}
