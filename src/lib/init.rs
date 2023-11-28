use super::constant::{DIRS, CONF_FILE_PATH};
use super::{ConfCheckResult, get_env_path};
use std::fs::{create_dir};
use std::path::Path;
use std::fs::File;
use std::io::{Write};
use super::core::Conf;

pub struct InitService;

impl InitService {
    pub fn init(result: ConfCheckResult) {
        match result {
            ConfCheckResult::DirUnCompleted => InitService::init_all(),
            ConfCheckResult::ConfFileNotFound => {
                if InitService::init_conf() {
                    if InitService::init_conf_data() {
                        println!("{}", "Slimk : init conf successfully!");
                    } else {
                        panic!("Slimk : init conf data error , Slimk can not open conf file or write configuration into conf file!");
                    }
                } else {
                    panic!("Slimk : can not create_command conf file!")
                }
            }
            ConfCheckResult::ConfParseError => {
                if !InitService::init_conf_data() {
                    panic!("Slimk : init conf data error , Slimk can not open conf file or write configuration into conf file!");
                }
            }
            ConfCheckResult::ConfCheckSuccess => (),
            ConfCheckResult::CheckError => println!("Slimk : check Error!")
        };
        // native and cache
        let conf = Conf::from_json();
        let mut update_strategy = conf.update_strategy().clone();
        //check native repo
        match get_env_path("repo").read_dir() {
            Ok(inner) => {
                println!("Slimk : {}", "Downloading | Updating default template...");
                let mut is_empty = true;
                for _item in inner {
                    is_empty = false;
                    if update_strategy.is_native_updated() {
                        let _ = update_strategy.update_native();
                    }
                    if update_strategy.is_cache_updated() {
                        let _ = update_strategy.update_cache();
                    }
                }
                if is_empty {
                    let _ = update_strategy.update_native();
                    let _ = update_strategy.update_cache();
                }
            }
            Err(e) => { panic!("{}", e) }
        }
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
                } else {
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
            if target_path.exists() {
                continue;
            } else {
                if create_dir(target_path.as_path()).is_err() {
                    return false;
                }
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
    pub fn init_native_cache() {
        let native = get_env_path("repo").join("slimk-binary");
        let mut up = Conf::from_json().update_strategy().clone();
        if native.exists() {
            //empty
            let mut flag = true;
            for _ in native.read_dir().expect("cannot read dir") {
                flag = false;
                break;
            }
            if flag {
                let _ = up.update_native();
                let _ = up.update_cache();
            }
        }
    }
}