mod core;
mod api;
mod check;
mod create;
mod init;
mod constant;

use std::path::PathBuf;
use std::str::FromStr;
pub use constant::*;


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
}

pub fn get_env_path(patch: &str) -> PathBuf {
    let env = PathBuf::from_str(env!()).expect("Invalid directory");
    return env.join(patch);
}