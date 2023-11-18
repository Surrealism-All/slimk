/// conf 作为配置文件目录
/// repo 作为存储库目录
/// log 作为日志目录
/// cache 作为存储库的缓存目录（15天清理）
pub const DIRS: Vec<&str> = vec!["conf", "repo", "cache", "log"];
/// 配置文件:slimk.json
pub const CONF_FILE: &str = "slimk.json";
/// author
pub const AUTHOR: &str = "syf20020816@outlook.com";
/// standard remote repository
pub const REMOTE_REPO: &str = "https://github.com/Surrealism-All/slimk-template";
/// standard template note
pub const TEMPLATE_NOTE: &str = "Standard Template For Slint With SurrealismUI";