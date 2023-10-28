use crate::args_parser::TemplatesCommands;

pub async fn manage_templates(_cmd: TemplatesCommands) -> Result<String, String> {
    match _cmd {
        TemplatesCommands::Create { alias, text } => handle_create_template(alias, text).await,
        _ => todo!(),
    }
}

async fn handle_create_template(_alias: String, _text: String) -> Result<String, String> {
    todo!()
}
