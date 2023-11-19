mod core;
mod api;
mod check;
mod create;
mod init;
mod constant;

use std::path::PathBuf;
use std::str::FromStr;
use std::env::current_dir;
pub use constant::*;
pub use core::Conf;
pub use init::InitService;
pub use check::CheckService;


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
    let env = current_dir().expect("can not operate the target directory");
    return env.join(patch);
}