use std::fs::read_dir;
use std::path::PathBuf;
use std::io;
use std::str::FromStr;
use figment::Figment;
use figment::providers::{Format, Json};
use crate::Conf;
use super::{DIRS, CONF_FILE,ConfCheckResult,get_env_path};


/// 初始化服务
pub struct CheckService;

impl CheckService {
    pub fn check() -> ConfCheckResult {
        if CheckService::check_dirs() {
            //完整则检查配置文件
            match CheckService::check_conf_exist() {
                Ok(_) => {
                    //检查配置
                    match CheckService::check_conf() {
                        Ok(_) => ConfCheckResult::ConfCheckSuccess,
                        Err(_) => ConfCheckResult::ConfParseError
                    }
                }
                Err(_) => ConfCheckResult::ConfFileNotFound
            }
        }
        return ConfCheckResult::DirUnCompleted;
    }

    /// 检查文件目录是否完整
    /// Check if the file directory is complete
    fn check_dirs() -> bool {
        //获取当前目录
        let dir_path = get_env_path("");
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
        let conf_path = get_env_path("slimk.json");
        return conf_path.try_exists();
    }
    /// 检查配置文件是否完整
    /// check configuration
    fn check_conf() -> Result<(), &'static str> {
        let conf_path = get_env_path("slimk.json");
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


