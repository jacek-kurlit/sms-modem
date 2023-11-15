use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(name = "sms")]
#[command(about = "Sending sms via usb modem", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    #[command(subcommand, about = "Manage contacts")]
    Contacts(ContactsCommands),
    #[command(subcommand, about = "Manage groups")]
    Groups(GroupsCommands),
    #[command(subcommand, about = "Manage message templates")]
    Templates(TemplatesCommands),
    #[command(about = "Send sms")]
    Send(SendSmsArgs),
}

#[derive(Debug, Subcommand)]
pub enum ContactsCommands {
    #[command(arg_required_else_help = true, about = "Add new contact")]
    Create {
        first_name: String,
        surname_name: String,
        phone: String,
        contact_name: Option<String>,
    },
    #[command(arg_required_else_help = true, about = "Delete contact")]
    Delete(ContactTargetArgs),
    #[command(arg_required_else_help = true, about = "Get contact")]
    Get(ContactTargetArgs),
    #[command(arg_required_else_help = true, about = "Update contact")]
    Update(ContactUpdateArgs),
    #[command(about = "List all contacts")]
    List,
}

#[derive(Debug, Args, Clone)]
pub struct ContactTargetArgs {
    pub contact_name: String,
    #[arg(
        short,
        help = "If many contacts have the same name, select one by index. Index starts from 0"
    )]
    pub index: Option<usize>,
}

#[derive(Debug, Args)]
#[command()]
pub struct ContactUpdateArgs {
    #[command(flatten)]
    pub contact_target: ContactTargetArgs,
    pub first_name: String,
    pub surname_name: String,
    pub phone: String,
    pub new_contact_name: Option<String>,
}

#[derive(Debug, Subcommand)]
pub enum GroupsCommands {
    #[command(arg_required_else_help = true, about = "Create new group")]
    Create { name: String },
    #[command(arg_required_else_help = true, about = "Delete group")]
    Delete { name: String },
    #[command(arg_required_else_help = true, about = "Get group")]
    Get { name: String },
    #[command(arg_required_else_help = true, about = "Assign concat to group")]
    Assign(AssignGroupArgs),
    #[command(arg_required_else_help = true, about = "Unassign concat from group")]
    Unassign(AssignGroupArgs),
    #[command(about = "List all groups")]
    List,
}

#[derive(Debug, Args)]
#[command()]
pub struct AssignGroupArgs {
    pub group_name: String,
    #[command(flatten)]
    pub contact_target: ContactTargetArgs,
}

#[derive(Debug, Subcommand)]
pub enum TemplatesCommands {
    #[command(arg_required_else_help = true, about = "Create new template")]
    Create { name: String, text: String },
    #[command(arg_required_else_help = true, about = "Delete template")]
    Delete { name: String },
    #[command(arg_required_else_help = true, about = "Get template")]
    Get { name: String },
    #[command(arg_required_else_help = true, about = "Update template")]
    Update { name: String, text: String },
    #[command(about = "List all templates")]
    List,
}

#[derive(Debug, Args)]
#[command()]
pub struct SendSmsArgs {
    #[command(flatten)]
    pub to: SmsTargetArgs,
    #[command(flatten)]
    pub message: SmsMessageArgs,
}

#[derive(Debug, Args, Clone)]
#[clap(group(
    clap::ArgGroup::new("target")
        .required(true)
        .args(&["number","contact_name", "group_name"]),
))]
pub struct SmsTargetArgs {
    #[arg(short)]
    pub number: Option<String>,
    #[arg(short)]
    pub contact_name: Option<String>,
    #[arg(short)]
    pub group_name: Option<String>,
}

#[derive(Debug, Args, Clone)]
#[clap(group(
    clap::ArgGroup::new("source")
        .required(true)
        .args(&["plain", "template"]),
))]
pub struct SmsMessageArgs {
    #[arg(short)]
    pub plain: Option<String>,
    #[arg(short)]
    pub template: Option<String>,
}
