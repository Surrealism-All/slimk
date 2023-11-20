mod core;
mod init_command;
mod list_command;
mod config_command;
mod create_command;
mod check;
mod init;
mod constant;

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::env::{current_dir, current_exe};
use std::fs::{create_dir, File};
use std::io::Write;
use clap::{Parser, Subcommand};

pub use check::CheckService;
pub use init::InitService;
use create_command::CreateCommand;
use init_command::InitCommand;
use list_command::ListCommand;
use config_command::ConfigCommand;

#[derive(Parser)]
#[command(name = "Slimk")]
#[command(version = "0.0.1")]
#[command(author = "syf20020816@outlook.com")]
#[command(about = "Slimk is a tool for creating Slint with Rust Programming!")]
pub struct Slimk {
    /// create a new program
    #[command(subcommand)]
    subcommand: SubCommand,
    /// list all native templates and remote templates
    #[arg(long, short = 'l')]
    list: bool,
}

impl Slimk {
    pub fn sub_command(&self) -> &SubCommand {
        &self.subcommand
    }
}

#[derive(Subcommand)]
pub enum SubCommand {
    /// create a project by selecting configuration items
    Create(CreateCommand),
    /// creates a new project but use the default strategy with no template
    Init(InitCommand),
    /// list all native templates and remote templates
    List(ListCommand),
    /// get or set slimk configurations
    Config(ConfigCommand),
}

///配置检测结果
#[derive(Debug, PartialEq, Clone)]
pub enum ConfCheckResult {
    ///文件目录不完整
    DirUnCompleted,
    ///配置文件不存在
    ConfFileNotFound,
    ///配置解析错误
    ConfParseError,
    ///检测成功
    ConfCheckSuccess,
    ///意外的检测
    CheckError,
}

pub fn get_env_path(patch: &str) -> PathBuf {
    let env = current_exe().expect("can not operate the target directory");
    let env: Vec<&str> = env.to_str().unwrap().split("slimk.exe").collect();
    return if patch.is_empty() {
        PathBuf::from(env[0])
    } else {
        PathBuf::from(env[0]).join(patch)
    };
}

pub fn get_work_path(patch: &str) -> PathBuf {
    let env = current_dir().expect("can not operate the target directory");
    return if patch.is_empty() {
        env
    } else {
        env.join(patch)
    };
}


pub fn create_dirs(prefix: &PathBuf, dirs: Vec<&str>) {
    for dir in dirs {
        let target_path = prefix.join(Path::new(dir));
        if create_dir(target_path.as_path()).is_err() {
            panic!("Slimk : Can not create_command {}", target_path.to_str().unwrap());
        }
    }
}

pub fn combine_to_map(map: &mut HashMap<String, String>, keys: Vec<&str>, values: Vec<&str>) {
    for (k, v) in keys.iter().zip(values.iter()) {
        map.insert(k.to_string(), v.to_string());
    }
}

pub fn create_files(prefix: &PathBuf, files: HashMap<String, String>) {
    for (file, data) in files {
        let target_path = prefix.join(Path::new(&file));
        if File::create(target_path.as_path()).unwrap().write_all(data.as_bytes()).is_err() {
            panic!("Slimk : Can not create_command {}", target_path.to_str().unwrap());
        }
    }
}

pub fn format_dir_name(prefix: &str, other: &str) -> PathBuf {
    return if prefix.is_empty() {
        get_work_path(other)
    } else {
        get_work_path(&format!("{}/{}", prefix, other))
    };
}