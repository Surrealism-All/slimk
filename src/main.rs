mod lib;

use lib::{CheckService, InitService, SubCommand, Slimk, Conf};
use clap::Parser;
use std::thread;

fn main() {
    // check configuration files and package structure at startup
    let _ = InitService::init(CheckService::check());
    let _ = InitService::init_native_cache();
    let app = Slimk::parse();
    match app.sub_command() {
        SubCommand::Create(create) => {
            let mut u = Conf::from_json().update_strategy().clone();
            u.update_native();
        }
        SubCommand::Init(init) => {
            init.cargo_new();
            init.cargo_add_slint();
            init.add_build_rs();
            init.add_readme();
            init.create_ui_dir();
        }
        SubCommand::List(list) => list.work(),
        SubCommand::Config(conf) => conf.work(),
    }
}
