mod lib;

use std::collections::HashMap;
use lib::{
    InitService, CheckService, Conf, get_env_path, get_work_path, README_MD, BUILD_RS, UI_DIRS,
    UI_DIRS_ASSETS, UI_FILES_COMPONENTS, UI_FILES_VIEWS, INDEX_SLINT, INDEX_SLINT_VIEWS, UI_FILES,
    INDEX_SLINT_COMPONENTS, MAIN_SLINT_VIEWS, GLOBAL_SLINT, APP_SLINT, HELLO_SLINT_COMPONENTS,
};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::str::FromStr;
use std::fs::{create_dir, File};
use std::io::Write;
use std::time::{SystemTime, UNIX_EPOCH};
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
    /// create a project by selecting configuration items
    Create(CreateCommand),
    /// creates a new project but use the default strategy with no template
    Init(InitCommand),
    /// list all native templates and remote templates
    List(ListCommand),
}

#[derive(Args, Debug, Clone)]
struct CreateCommand {
    name: String,
    /// choose a template to quick create
    #[arg(long, short = 't', default_value = "", value_name = "TEMPLATE", help = "choose a template to create")]
    template: String,
}

#[derive(Args, Debug, Clone)]
struct InitCommand {
    name: String,
}

impl InitCommand {
    fn name(&self) -> &str {
        &self.name
    }
    fn cargo_new(&self) {
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
        let mut name = String::from(self.name());
        let cmd = Command::new("cargo").args(&["new", &name]).output().expect("cargo new fail, please check your environment or rust toolchain is not installed in your computer!");
        if cmd.status.success() {
            println!("Slimk - [{}] : cargo new ---> (success)", timestamp);
        } else {
            eprintln!("Slimk - [{}] : {} ---> (fail)", timestamp, String::from_utf8_lossy(&cmd.stderr));
        }
    }
    fn cargo_add_slint(&self) {
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
        let path = get_work_path(self.name());
        // add slint
        let slint = Command::new("cargo").args(&["add", "slint"]).current_dir(path.as_path()).output().expect("cargo add slint fail, please check your network!");
        if slint.status.success() {
            println!("Slimk - [{}] : cargo add slint ---> (success)", timestamp);
        } else {
            eprintln!("Slimk - [{}] : {} ---> (fail)", timestamp, String::from_utf8_lossy(&slint.stderr));
        }
        // add slint-build
        let slint_build = Command::new("cargo").args(&["add", "slint-build", "--build"]).current_dir(path.as_path()).output().expect("cargo add slint-build fail, please check your network!");
        if slint_build.status.success() {
            println!("Slimk - [{}] : cargo add slint-build --build ---> (success)", timestamp);
        } else {
            eprintln!("Slimk - [{}] : {} ---> (fail)", timestamp, String::from_utf8_lossy(&slint_build.stderr));
        }
    }
    fn add_build_rs(&self) {
        let build_rs = get_work_path(&format!("{}/{}", self.name(), "build.rs"));
        match File::create(build_rs.as_path()).unwrap().write_all(BUILD_RS.as_bytes()) {
            Ok(_) => println!("Slimk : Add build.rs ---> (success)"),
            Err(e) => panic!("Slimk : Add build.rs ---> (fail) => reason :\n{}", e),
        };
    }
    fn add_readme(&self) {
        let readme = get_work_path(&format!("{}/{}", self.name(), "README.md"));
        match File::create(readme.as_path()).unwrap().write_all(README_MD.as_bytes()) {
            Ok(_) => println!("Slimk : Add README.md ---> (success)"),
            Err(e) => panic!("Slimk : Add README.md ---> (fail) => reason :\n{}", e),
        };
    }
    fn create_ui_dir(&self) {
        let ui_path = get_work_path(&format!("{}/{}", self.name(), "ui"));
        create_dir(&ui_path.as_path());
        create_dirs(&ui_path, UI_DIRS.to_vec());
        let assets_path = &ui_path.join("assets");
        create_dirs(assets_path, UI_DIRS_ASSETS.to_vec());
        let mut map1: HashMap<String, String> = HashMap::new();
        combine_to_map(&mut map1, UI_FILES.to_vec(), vec![INDEX_SLINT, GLOBAL_SLINT, APP_SLINT]);
        create_files(&ui_path, map1);
        let components_path = &ui_path.join("components");
        let mut map2: HashMap<String, String> = HashMap::new();
        combine_to_map(&mut map2, UI_FILES_COMPONENTS.to_vec(), vec![HELLO_SLINT_COMPONENTS, INDEX_SLINT_COMPONENTS]);
        create_files(components_path, map2);
        let views_path = &ui_path.join("views");
        let mut map3: HashMap<String, String> = HashMap::new();
        combine_to_map(&mut map3, UI_FILES_VIEWS.to_vec(), vec![MAIN_SLINT_VIEWS, INDEX_SLINT_VIEWS]);
        create_files(views_path, map3);
    }
}

#[derive(Args, Debug, Clone)]
struct ListCommand {
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

fn create_dirs(prefix: &PathBuf, dirs: Vec<&str>) {
    for dir in dirs {
        let target_path = prefix.join(Path::new(dir));
        if create_dir(target_path.as_path()).is_err() {
            panic!("Slimk : Can not create {}", target_path.to_str().unwrap());
        }
    }
}

fn combine_to_map(map: &mut HashMap<String, String>, keys: Vec<&str>, values: Vec<&str>) {
    for (k, v) in keys.iter().zip(values.iter()) {
        map.insert(k.to_string(), v.to_string());
    }
}

fn create_files(prefix: &PathBuf, files: HashMap<String, String>) {
    for (file, data) in files {
        let target_path = prefix.join(Path::new(&file));
        if File::create(target_path.as_path()).unwrap().write_all(data.as_bytes()).is_err() {
            panic!("Slimk : Can not create {}", target_path.to_str().unwrap());
        }
    }
}

fn main() {
    //check configuration files and package structure at startup
    let _ = InitService::init(CheckService::check());
    let app = Slimk::parse();
    match app.subcommand {
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
