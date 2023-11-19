mod lib;

use lib::{InitService, CheckService};
use std::path::PathBuf;
use std::str::FromStr;
use clap::{Parser, Subcommand, Args};

#[derive(Parser)]
#[command(name = "Slimk")]
#[command(version = "0.0.1")]
#[command(author = "syf20020816@outlook.com")]
#[command(about = "Slimk is a tool for creating Slint with Rust Programming!")]
struct Slimk {
    /// create a new program
    #[command(subcommand)]
    subcommand: SubCommand,
    /// list all native templates and remote templates
    #[arg(long, short = 'l')]
    list: bool,
}

#[derive(Subcommand)]
enum SubCommand {
    /// create a new program
    Create(CreateCommand),
    Init(InitCommand),
}

#[derive(Args, Debug, Clone)]
struct CreateCommand {
    name: String,
    /// choose a template to quick create
    #[arg(long, short = 't', default_value = "", value_name = "TEMPLATE", help = "choose a template to ")]
    template: String,
}

#[derive(Args, Debug, Clone)]
struct InitCommand {
    path: PathBuf,
}


fn main() {
    //启动时检查配置文件和包结构
    let _ = InitService::init(CheckService::check());
    let app = Slimk::parse();
    match app.subcommand {
        SubCommand::Create(c) => println!("{:?}", c),
        SubCommand::Init(init) => println!("{:?}", init)
    }
}
