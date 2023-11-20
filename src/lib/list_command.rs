use super::core::Conf;
use clap::Args;

#[derive(Args, Debug, Clone)]
pub struct ListCommand {
    #[arg(long, short = 'a', help = "list both native templates and remotes templates")]
    all: bool,
    #[arg(long, short = 'n', help = "list native templates")]
    native: bool,
    #[arg(long, short = 'r', help = "list remote templates")]
    remote: bool,
}

impl ListCommand {
    pub fn is_all(&self) -> bool {
        self.all
    }
    pub fn is_native(&self) -> bool {
        self.native
    }
    pub fn is_remote(&self) -> bool {
        self.remote
    }
    pub fn work(&self) {
        let conf = Conf::from_json();
        if self.is_all() {
            println!("{}", conf.display_remotes());
            println!("{}", conf.display_natives());
            return;
        }
        if self.is_remote() {
            /// get configuration : remotes
            println!("{}", conf.display_remotes());
            return;
        }
        if self.is_native() {
            println!("{}", conf.display_natives());
            return;
        }
    }
}