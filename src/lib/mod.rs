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
use std::error::Error;
use std::fs::{copy, create_dir, create_dir_all, File, read_dir, remove_dir, remove_dir_all};
use std::io;
use std::io::{Read, Write};
use clap::{Parser, Subcommand};
use zip::ZipArchive;

pub use check::CheckService;
pub use init::InitService;
pub use core::Conf;
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

pub fn unzip_file(zip_path: &Path, extract_to: &Path) -> Result<(), Box<dyn std::error::Error>> {
    if extract_to.exists() {
        let _ = remove_dir_all(extract_to);
    }
    match create_dir(extract_to) {
        Ok(_) => {
            // 打开 ZIP 文件
            let file = File::open(zip_path)?;
            // 创建 ZIP 归档对象
            let mut archive = ZipArchive::new(file)?;
            // 遍历 ZIP 归档中的每个文件
            for i in 0..archive.len() {
                let mut file = archive.by_index(i)?;
                // 构建文件的目标路径
                let dest_path = get_env_path("repo").join(file.name());
                if file.is_dir() {
                    std::fs::create_dir_all(&dest_path)?;
                } else {
                    // 如果是文件，创建文件并写入数据
                    let mut buffer = Vec::new();
                    file.read_to_end(&mut buffer)?;
                    let mut extracted_file = File::create(dest_path)?;
                    extracted_file.write_all(&buffer)?;
                }
            }
            Ok(())
        }
        Err(e) => {
            Err(Box::new(e))
        }
    }
}

pub fn copy_dir(src: &Path, dest: &Path) -> io::Result<()> {
    // 创建目标目录
    create_dir_all(dest)?;

    // 遍历源目录中的所有条目
    for entry in read_dir(src)? {
        let entry = entry?;
        let entry_path = entry.path();
        let dest_path = dest.join(entry.file_name());

        // 如果是目录，递归复制目录
        if entry_path.is_dir() {
            copy_dir(&entry_path, &dest_path)?;
        } else {
            // 如果是文件，复制文件
            copy(&entry_path, &dest_path)?;
        }
    }

    Ok(())
}

pub fn get_template_info() -> Option<Vec<(String, String)>> {
    let conf = Conf::from_json();
    return if conf.remotes().is_empty() {
        None
    } else {
        Some(conf.remotes().iter().map(|(name, template)| {
            (name.to_string(), template.url().to_string())
        }).collect::<Vec<(String, String)>>())
    };
}