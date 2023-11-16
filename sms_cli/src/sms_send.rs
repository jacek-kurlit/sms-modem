use sms_api::SmsError;
use sms_config::config::SmsApiConf;
use sms_db::{groups::Group, templates::Template};

use crate::args_parser::{SendSmsArgs, SmsMessageArgs, SmsTargetArgs};

pub async fn send_sms(
    send_args: SendSmsArgs,
    sms_api_config: &SmsApiConf,
) -> Result<String, String> {
    let numbers = get_recipient_numbers(send_args.to).await?;
    let message = get_message_to_send(send_args.message).await?;
    println!(
        "Sending sms to {} number of people with message '{}'",
        numbers.len(),
        message
    );

    send(&message, numbers, sms_api_config)
        .await
        .map(|_| "Message sent".to_string())
        .map_err(|e| format!("Could not send message, Reason: {:?}", e))
}

async fn get_recipient_numbers(target_args: SmsTargetArgs) -> Result<Vec<String>, String> {
    if let Some(contact) = target_args.contact_name {
        return sms_db::repository::contacts()
            .find_all_by_contact_name(&contact)
            .await
            .map(|contacts| contacts.into_iter().map(|c| c.phone).collect());
    }
    if let Some(phone) = target_args.number {
        return Ok(vec![phone]);
    }
    if let Some(group) = target_args.group_name {
        return find_all_group_numbers(group).await;
    }
    panic!("Invalid state, no target were specified")
}

async fn find_all_group_numbers(group_name: String) -> Result<Vec<String>, String> {
    sms_db::repository::groups()
        .find_group_details(&Group::id_from_name(&group_name))
        .await?
        .ok_or_else(|| format!("Group {} not found", group_name))
        .map(|group_details| {
            group_details
                .contacts
                .into_iter()
                .map(|c| c.phone)
                .collect()
        })
}

async fn get_message_to_send(args: SmsMessageArgs) -> Result<String, String> {
    if let Some(plain) = args.plain {
        return Ok(plain);
    }
    if let Some(template) = args.template {
        return sms_db::repository::templates()
            .get(&Template::id_from_name(&template))
            .await?
            .map(|t| t.text)
            .ok_or_else(|| format!("Template {} not found", template));
    }
    panic!("Invalid state, no message were specified")
}

async fn send(
    message: &str,
    numbers: Vec<String>,
    sms_api_config: &SmsApiConf,
) -> Result<(), SmsError> {
    sms_api::create_service(sms_api_config)?
        .send_sms(
            message,
            numbers
                .iter()
                .map(|s| s.as_str())
                .collect::<Vec<&str>>()
                .as_slice(),
        )
        .await
}
