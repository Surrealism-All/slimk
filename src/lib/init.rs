use super::{ConfCheckResult, DIRS, CONF_FILE, get_env_path};
use std::fs::{create_dir, create_dir_all};
use std::path::Path;
use std::fs::File;
use std::io;
use std::io::Write;
use crate::lib::core::Conf;

pub struct InitService;

impl InitService {
    pub fn init(result: ConfCheckResult) {
        match result {
            ConfCheckResult::DirUnCompleted => InitService::init_dirs(),
            ConfCheckResult::ConfFileNotFound => InitService::init_conf(),
            ConfCheckResult::ConfParseError => InitService::init_conf_data(),
            ConfCheckResult::ConfCheckSuccess => ()
        };
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
    fn init_conf() -> io::Result<File> {
        let conf_path = get_env_path(CONF_FILE);
        return File::create(conf_path.as_path());
    }
    // 创建配置文件中的数据
    fn init_conf_data() {
        let conf_path = get_env_path(CONF_FILE);
        if conf_path.exists(){
            File::write_all()
        }
    }
}