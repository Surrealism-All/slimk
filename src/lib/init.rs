use super::{ConfCheckResult, DIRS, CONF_FILE, get_env_path, CONF_FILE_PATH};
use std::fs::{create_dir, create_dir_all};
use std::path::Path;
use std::fs::File;
use std::io;
use std::io::{stdout, Write};
use crate::lib::core::Conf;

pub struct InitService;

impl InitService {
    pub fn init(result: ConfCheckResult) {
        dbg!(&result);
        match result {
            ConfCheckResult::DirUnCompleted => InitService::init_all(),
            ConfCheckResult::ConfFileNotFound => {
                if InitService::init_conf() {
                    if InitService::init_conf_data() {
                        println!("{}", "Slimk : init conf successfully!");
                    }else{
                        panic!("Slimk : init conf data error , Slimk can not open conf file or write configuration into conf file!");
                    }
                } else {
                    panic!("Slimk : can not create conf file!")
                }
            }
            ConfCheckResult::ConfParseError => {
                if !InitService::init_conf_data() {
                    panic!("Slimk : init conf data error , Slimk can not open conf file or write configuration into conf file!");
                }
            }
            ConfCheckResult::ConfCheckSuccess => println!("Slimk : check and init successfully!"),
            ConfCheckResult::CheckError => println!("Slimk : check Error!")
        };
    }
    /// match ConfCheckResult::DirUnCompleted
    /// - 1. init_dirs()
    /// - 2. init_conf()
    /// - 3. init_conf_data
    fn init_all() {
        if InitService::init_dirs() {
            if InitService::init_conf() {
                if InitService::init_conf_data() {
                    println!("{}", "Slimk : init conf successfully!");
                }else{
                    panic!("Slimk : init conf data error , Slimk can not open conf file or write configuration into conf file!");
                }
            } else {
                panic!("Slimk : can not create conf file!")
            }
        } else {
            panic!("Slimk : can not create needed directories!")
        }
    }
    fn init_dirs() -> bool {
        let dir_path = get_env_path("");
        for dir in DIRS {
            let target_path = dir_path.join(Path::new(dir));
            if create_dir(target_path.as_path()).is_err() {
                return false;
            }
        }
        return true;
    }
    // 创建配置文件
    fn init_conf() -> bool {
        let conf_path = get_env_path(CONF_FILE_PATH);
        if File::create(conf_path.as_path()).unwrap().write(b"").is_ok() {
            return true;
        }
        return false;
    }
    // 创建配置文件中的数据
    fn init_conf_data() -> bool {
        let conf_path = get_env_path(CONF_FILE_PATH);
        if conf_path.exists() {
            return match File::create(conf_path.as_path()) {
                Ok(mut file) => {
                    if file.write_all(Conf::default().to_json().as_bytes()).is_ok() {
                        return true;
                    }
                    false
                }
                Err(_) => false,
            };
        }
        false
    }
}