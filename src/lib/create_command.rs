use std::path::Path;
use clap::Args;
use crate::lib::{Conf, copy_dir, get_env_path, get_work_path};
use crate::lib::core::Template;
use crate::lib::init_command::InitCommand;

#[derive(Args, Debug, Clone)]
pub struct CreateCommand {
    name: String,
    /// choose a template to quick create
    #[arg(long, short = 't', default_value = "", value_name = "TEMPLATE", help = "choose a template to create")]
    template: String,
}

impl CreateCommand {
    fn name(&self) -> &str {
        &self.name
    }
    fn template(&self) -> &str { &self.template }
    fn has_template(&self) -> CreateMatch {
        let t = self.template().to_string();
        //judge template is empty
        return if t.is_empty() {
            let natives = Conf::from_json().natives().clone();
            match natives.get("slimk-binary") {
                None => panic!("Slimk : {}", "The configuration file is missing. Please delete all files and directories except slimk.exe and execute the command again"),
                Some(default) => CreateMatch::Template(default.url().to_string())
            }
        } else {
            let natives = Conf::from_json().natives().clone();
            if let Some(template) = natives.get(&t) {
                return CreateMatch::Template(template.url().to_string());
            }

            CreateMatch::NoTemplate
        };
    }
    pub fn work(&self) {
        match self.has_template() {
            CreateMatch::NoTemplate => println!("Slimk : {}", "Template does not exist"),
            CreateMatch::Template(url) => {
                let path = get_work_path(self.name());
                let _ = copy_dir(Path::new(&url), path.as_path());
                let init = InitCommand::new(self.name());
                init.cargo_new();
                init.cargo_add_slint();
                init.add_build_rs();
                init.add_readme();
            }
        }
    }
}

enum CreateMatch {
    NoTemplate,
    Template(String),
}