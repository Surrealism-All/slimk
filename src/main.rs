mod lib;

use lib::{CheckService, InitService, SubCommand, Slimk};
use clap::Parser;

fn main() {
    //check configuration files and package structure at startup
    let _ = InitService::init(CheckService::check());
    let app = Slimk::parse();
    match app.sub_command() {
        SubCommand::Create(c) => println!("{:?}", c),
        SubCommand::Init(init) => {
            init.cargo_new();
            init.cargo_add_slint();
            init.add_build_rs();
            init.add_readme();
            init.create_ui_dir();
        }
        SubCommand::List(list) => list.work(),
    }
}
