use clap::Args;

#[derive(Args, Debug, Clone)]
pub struct CreateCommand {
    name: String,
    /// choose a template to quick create
    #[arg(long, short = 't', default_value = "", value_name = "TEMPLATE", help = "choose a template to create")]
    template: String,
}
