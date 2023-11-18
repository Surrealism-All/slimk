mod lib;

use std::collections::HashMap;
use std::fs::read_dir;
use std::path::PathBuf;
use std::io;
use std::str::FromStr;
use clap::{Parser, Subcommand, Args};
use figment::Figment;
use figment::providers::{Format, Json};

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

/// conf 作为配置文件目录
/// repo 作为存储库目录
/// log 作为日志目录
/// cache 作为存储库的缓存目录（15天清理）
const DIRS: Vec<&str> = vec!["conf", "repo", "cache", "log"];
/// 配置文件:slimk.json
const CONF_FILE: &str = "slimk.json";
const AUTHOR: &str = "syf20020816@outlook.com";

/// 初始化服务
pub struct InitService;

impl InitService {



    pub fn check() {
        if InitService::check_dirs(){
            //完整则检查配置文件
            match InitService::check_conf_exist(){
                Ok(_) => {}
                Err(e) => {}
            }
        }
    }
    fn get_env_path(patch: &str) -> PathBuf {
        let env = PathBuf::from_str(env!()).expect("Invalid directory");
        return env.join(patch);
    }
    /// 检查文件目录是否完整
    /// Check if the file directory is complete
    fn check_dirs() -> bool {
        //获取当前目录
        let dir_path = InitService::get_env_path("");
        //遍历当前目录
        let dirs = dir_path.read_dir().expect("Invalid directory");
        let mut counter = 0;
        for dir in dirs {
            if DIRS.contains(&dir.unwrap().file_name().to_str().unwrap()) { counter += 1; }
        }
        return if counter == 4 {
            true
        } else {
            false
        };
    }
    /// 检查配置文件是否存在
    /// check whether the configuration file exists
    fn check_conf_exist() -> io::Result<bool> {
        let conf_path = InitService::get_env_path("slimk.json");
        return conf_path.try_exists();
    }
    /// 检查配置文件是否完整
    /// check configuration
    fn check_conf() -> Result<(), &'static str> {
        let conf_path = InitService::get_env_path("slimk.json");
        //获取配置信息
        let parser = Figment::new();
        let conf_data = parser.merge(Json::file(conf_path.as_path()).nested()).extract::<Conf>();
        return if conf_data.is_err() {
            Err("conf file parse error")
        } else {
            Ok(())
        };
    }
}

///Slimk配置类
pub struct Conf {
    /// 用户名
    user: String,
    /// 邮件
    email: String,
    /// 远程模板仓库地址
    remotes: HashMap<String, String>,
    /// 本地模板仓库地址
    natives: HashMap<String, String>,
    /// 创建策略
    create_strategy: CreateStrategy,
    /// 更新策略
    update_strategy: UpdateStrategy,
}

/// 更新策略只针对本地仓库和缓存
struct UpdateStrategy {
    /// 更新本地仓库间隔
    /// - n == 0 : 每次下载都更新本地仓库
    /// - n <= -1: 永远不更新本地仓库
    /// - n > 0 : n天进行一次更新（创建项目时进行检测，不创建不更新）
    native: i32,
    /// cache策略
    /// - <= -1 : 不启用cache
    /// - 0 ：每次本地仓库更新都生成cache
    cache: i32,
}

/// 创建策略
struct CreateStrategy {
    /// - true : 表示尝试远程拉取来创建库（版本依赖为最新）
    /// - false : 表示不进行远程拉取，使用本地仓库创建
    remote: bool,
    /// 默认创建时使用的仓库
    default: String,
    /// 默认创建不存在时采用次选
    secondary: String,
    /// true : 永远采用单步选择的方式不采用仓库直接创建
    define: bool,
}

fn main() {
    //启动时检查配置文件和包结构

    let app = Slimk::parse();
    match app.subcommand {
        SubCommand::Create(c) => println!("{:?}", c),
    }
}
