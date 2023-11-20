use clap::{Args, Subcommand, ValueEnum};
use crate::lib::core::Conf;

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
                ConfigEnum::Remotes => println!("{:?}", conf_data.display_remotes()),
                ConfigEnum::Natives => println!("{:?}", conf_data.display_natives()),
                ConfigEnum::Create => println!("{:?}", conf_data.display_create_strategy()),
                ConfigEnum::Update => println!("{:?}", conf_data.display_update_strategy()),
            };
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
