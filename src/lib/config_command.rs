use std::option::Option::Some;
use clap::{Args, Subcommand, ValueEnum};
use crate::lib::core::Conf;
use crate::lib::get_env_path;
use std::io::{Read, stdin};
use std::process::Command;
use crate::lib::constant::CONF_FILE_PATH;

#[derive(Args, Debug, Clone)]
pub struct ConfigCommand {
    #[arg(short = 'g', long, help = "get configuration", value_enum, group = "config")]
    get: Option<ConfigEnum>,
    #[arg(short = 's', long, help = "set configuration", value_enum, group = "config")]
    set: Option<ConfigEnum>,
    #[arg(long, short = 'p', help = "get where the configuration is", group = "config")]
    place: bool,
}

impl ConfigCommand {
    pub fn work(&self) {
        if let Some(conf) = self.get.as_ref() {
            let conf_data = Conf::from_json();
            match conf {
                ConfigEnum::User => println!("{:?}", conf_data.user()),
                ConfigEnum::Email => println!("{:?}", conf_data.email()),
                ConfigEnum::Remotes => println!("{}", conf_data.display_remotes()),
                ConfigEnum::Natives => println!("{}", conf_data.display_natives()),
                ConfigEnum::Create => println!("{}", conf_data.display_create_strategy()),
                ConfigEnum::Update => println!("{}", conf_data.display_update_strategy()),
            };
        }
        if let Some(conf_set) = self.set.as_ref() {
            let mut conf = Conf::from_json();
            match conf_set {
                ConfigEnum::User => {
                    let username = parse_stdin("please enter your username");
                    let _ = conf.set_user(&username);
                }
                ConfigEnum::Email => {
                    let email = parse_stdin("please enter your email");
                    let _ = conf.set_email(&email);
                }
                _ => {
                    println!("Slimk : {}", "The configuration is quite complex. Please open the file directly for modification");
                    println!("{}", get_env_path("conf").join("slimk.json").to_str().unwrap());
                }
            }
            let _ = conf.write_back();
        }
        if self.place {
            println!("{}", get_env_path("conf").join("slimk.json").to_str().unwrap());
        }
    }
}


#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, ValueEnum)]
pub enum ConfigEnum {
    User,
    Email,
    Remotes,
    Natives,
    Create,
    Update,
}

fn parse_stdin(prompt: &str) -> String {
    println!("Slimk: {}", prompt);
    let mut stdin_str = String::new();
    stdin().read_line(&mut stdin_str).expect("can not parse your enter!");
    return String::from(stdin_str.trim());
}