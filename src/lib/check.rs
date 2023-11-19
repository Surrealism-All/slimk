use std::fs::read_dir;
use std::path::PathBuf;
use std::io;
use std::str::FromStr;
use figment::Figment;
use figment::providers::{Format, Json};
use super::{DIRS, CONF_FILE, ConfCheckResult, get_env_path, Conf, CONF_FILE_PATH};


/// 初始化服务
/// CheckService
/// check directories and configuration use
pub struct CheckService;

impl CheckService {
    /// check the packages and configurations
    pub fn check() -> ConfCheckResult {
        let mut counter = 0;
        if CheckService::check_dirs() {
            //完整则检查配置文件
            match CheckService::check_conf_exist() {
                Ok(_) => {
                    //检查配置
                    match CheckService::check_conf() {
                        Ok(_) => counter = 100,
                        Err(_) => counter += 2,
                    };
                }
                Err(_) => counter += 1,
            };
        }

        match counter {
            0 => ConfCheckResult::DirUnCompleted,
            1 => ConfCheckResult::ConfFileNotFound,
            2 => ConfCheckResult::ConfParseError,
            100 => ConfCheckResult::ConfCheckSuccess,
            _ => ConfCheckResult::CheckError,
        }
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
        let conf_path = get_env_path(CONF_FILE_PATH);
        return conf_path.try_exists();
    }
    /// 检查配置文件是否完整
    /// check configuration
    fn check_conf() -> Result<(), &'static str> {
        let conf_path = get_env_path(CONF_FILE_PATH);
        //获取配置信息
        let parser = Figment::from(<Json as Format>::file(conf_path.as_path())).extract::<Conf>();
        return if parser.is_err() {
            Err("conf file parse error")
        } else {
            Ok(())
        };
    }
}


