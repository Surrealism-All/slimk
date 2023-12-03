use std::collections::HashMap;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::path::Path;
use std::process::Command;
use serde::{Serialize, Deserialize};
use std::fs::{copy, File, read_dir, remove_dir, remove_dir_all, remove_file};
use std::io::Write;
use super::constant::{REMOTE_REPO, TEMPLATE_NOTE, CONF_FILE_PATH};
use super::{unzip_file, get_env_path, copy_dir};
use std::time::{SystemTime, UNIX_EPOCH};
use figment::Figment;
use figment::providers::{Json, Format};
use serde_json::Value;


///Slimk配置类
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Conf {
    /// 用户名
    user: Option<String>,
    /// 邮件
    email: Option<String>,
    /// 远程模板仓库地址
    remotes: HashMap<String, Template>,
    /// 本地模板仓库地址
    natives: HashMap<String, Template>,
    /// 创建策略
    create_strategy: CreateStrategy,
    /// 更新策略
    update_strategy: UpdateStrategy,
}

impl Default for Conf {
    fn default() -> Self {
        let tags = get_releases_tags_from_github();
        // get latest release
        let url = get_release_url_from_github(Some(&tags[0])).unwrap();
        //new a template
        let mut template = Template::default_template();
        template.set_url(&url);
        let mut remotes: HashMap<String, Template> = HashMap::new();
        let _ = remotes.insert(String::from("slimk-binary"), template);
        Conf {
            user: None,
            email: None,
            remotes,
            natives: Default::default(),
            create_strategy: CreateStrategy::default(),
            update_strategy: UpdateStrategy::default(),
        }
    }
}

#[allow(dead_code)]
impl Conf {
    pub fn new() -> Conf {
        Conf::default()
    }
    /// add native repository
    pub fn add_native(&mut self, name: &str, url: &str, note: Option<&str>) {
        let _ = self.natives.insert(String::from(name), Template::new(url, note));
    }
    /// add native repository
    pub fn add_remote(&mut self, name: &str, url: &str, note: Option<&str>) {
        let _ = self.remotes.insert(String::from(name), Template::new(url, note));
    }
    pub fn user(&self) -> &Option<String> {
        &self.user
    }
    pub fn set_user(&mut self, user: &str) {
        let _ = self.user.replace(String::from(user));
    }
    pub fn email(&self) -> &Option<String> {
        &self.email
    }
    pub fn set_email(&mut self, email: &str) {
        let _ = self.email.replace(String::from(email));
    }
    pub fn natives(&self) -> &HashMap<String, Template> {
        &self.natives
    }
    pub fn remotes(&self) -> &HashMap<String, Template> {
        &self.remotes
    }
    pub fn native_insert(&mut self, name: &str, url: &str, note: Option<&str>) {
        self.natives.insert(String::from(name), Template::new(url, note));
    }
    pub fn set_update_strategy(&mut self, update_strategy: UpdateStrategy) {
        self.update_strategy = update_strategy;
    }
    pub fn update_strategy(&self) -> &UpdateStrategy {
        &self.update_strategy
    }
    pub fn create_strategy(&self) -> &CreateStrategy {
        &self.create_strategy
    }
    pub fn display_natives(&self) -> String {
        self.natives().iter().map(|(key, value)| format!("{}:\n{}", key, value.to_string())).collect::<String>()
    }
    pub fn display_remotes(&self) -> String {
        self.remotes().iter().map(|(key, value)| format!("{}:\n{}", key, value.to_string())).collect::<String>()
    }
    pub fn display_update_strategy(&self) -> String {
        self.update_strategy().to_string()
    }
    pub fn display_create_strategy(&self) -> String {
        self.create_strategy().to_string()
    }
    pub fn to_json(&self) -> String {
        serde_json::to_string_pretty(self).unwrap()
    }
    /// get configuration from slimk.json
    pub fn from_json() -> Conf {
        let conf_path = get_env_path(CONF_FILE_PATH);
        Figment::from(<Json as Format>::file(conf_path.as_path())).extract::<Conf>().unwrap()
    }
    pub fn write_back(&self) {
        let conf_path = get_env_path(CONF_FILE_PATH);
        let _ = File::create(conf_path).unwrap().write_all(self.to_json().as_bytes());
    }
}


/// 更新策略只针对本地仓库和缓存
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UpdateStrategy {
    /// 更新本地仓库间隔
    /// - n == 0 : 每次下载都更新本地仓库
    /// - n <= -1: 永远不更新本地仓库
    /// - n > 0 : n天进行一次更新（创建项目时进行检测，不创建不更新）
    native: i32,
    /// cache策略
    /// - <= -1 : 不启用cache
    /// - 0 ：每次本地仓库更新都生成cache
    cache: i32,
    native_timestamp: usize,
    cache_timestamp: usize,
}

impl Default for UpdateStrategy {
    fn default() -> Self {
        UpdateStrategy {
            native: 15,
            cache: 7,
            native_timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as usize,
            cache_timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as usize,
        }
    }
}

#[allow(dead_code)]
impl UpdateStrategy {
    pub fn new() -> Self {
        UpdateStrategy::default()
    }
    pub fn native(&self) -> i32 {
        self.native
    }
    pub fn cache(&self) -> i32 {
        self.cache
    }
    pub fn native_timestamp(&self) -> usize {
        self.native_timestamp
    }
    pub fn set_native_timestamp(&mut self, timestamp: usize) {
        self.native_timestamp = timestamp
    }
    pub fn cache_timestamp(&self) -> usize {
        self.cache_timestamp
    }
    pub fn set_cache_timestamp(&mut self, timestamp: usize) {
        self.cache_timestamp = timestamp
    }
    pub fn set_native(&mut self, native: i32) {
        self.native = native;
    }
    pub fn set_cache(&mut self, cache: i32) {
        self.cache = cache
    }
    pub fn is_native_updated(&self) -> bool {
        UpdateStrategy::is_updated(self.native(), self.native_timestamp())
    }
    pub fn is_cache_updated(&self) -> bool {
        UpdateStrategy::is_updated(self.cache(), self.cache_timestamp())
    }
    pub fn is_updated(target: i32, timestamp: usize) -> bool {
        return if target == 0 {
            // update each
            true
        } else if target <= -1 {
            // never update
            false
        } else {
            let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as isize;
            let update = timestamp + (target as usize) * 86400000_usize;
            if now - (update as isize) < 0 {
                // update time has not arrived
                return false;
            }
            true
        };
    }
    pub fn update_native(&mut self) {
        if ping_github() {
            let repo = get_env_path("repo");
            if let Some(url) = get_release_url_from_github(Some("latest")) {
                let curl = Command::new("curl").args([
                    "-L", "-H", "Accept: application/vnd.github+json", "-H", "X-GitHub-Api-Version: 2022-11-28", "-o", "slimk-binary.zip", &url
                ]).current_dir(Path::new(&repo)).output().unwrap();
                if curl.status.success() {
                    //unzip
                    match unzip_file(repo.join("slimk-binary.zip").as_path(), repo.join("slimk-binary").as_path()) {
                        Ok(_) => {
                            remove_file(repo.join("slimk-binary.zip").as_path());
                            //update timestamp
                            let _ = self.set_native_timestamp(SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as usize);
                            let mut conf = Conf::from_json();
                            let _ = conf.set_update_strategy(self.clone());
                            let _ = conf.native_insert("slimk-binary", repo.join("slimk-binary").to_str().unwrap(), Some("this is a native default template use Slint with SurrealismUI"));
                            let conf_path = get_env_path(CONF_FILE_PATH);
                            let _ = File::create(conf_path).unwrap().write_all(conf.to_json().as_bytes());
                        }
                        Err(e) => {
                            panic!("Slimk : {}", e)
                        }
                    }
                } else {
                    panic!("{}", "download failed, please check your network settings!");
                }
            }
        }
    }
    pub fn update_cache(&mut self) {
        let cache = get_env_path("cache");
        let cache_dir = cache.read_dir().unwrap();
        for item in cache_dir {
            let _ = remove_dir_all(item.unwrap().path().as_path());
        }
        let native = get_env_path("repo");
        copy_dir(native.as_path(), cache.as_path());
        let _ = self.set_cache_timestamp(SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as usize);
        let mut conf = Conf::from_json();
        let _ = conf.set_update_strategy(self.clone());
        let conf_path = get_env_path(CONF_FILE_PATH);
        let _ = File::create(conf_path).unwrap().write_all(conf.to_json().as_bytes());
    }
}

///check connection to github
pub fn ping_github() -> bool {
    let ping = Command::new("ping").arg("github.com").output().expect("Slimk : can not connect to github , please check your network settings!");
    return if ping.status.success() {
        true
    } else {
        false
    };
}

pub fn get_releases_from_github() -> Option<Vec<Value>> {
    let curl = Command::new("curl").args([
        "-L", "-H", "Accept: application/vnd.github+json", "-H", "X-GitHub-Api-Version: 2022-11-28",
        "https://api.github.com/repos/Surrealism-All/slimk-template/releases"
    ]).output().unwrap();

    if curl.status.success() {
        // stdout -> json
        let stdout_str = String::from_utf8(curl.stdout).unwrap();
        let json: Value = serde_json::from_str(&stdout_str).unwrap();
        if let Some(array) = json.as_array() {
            return Some(array.clone());
        }
    } else {
        panic!("Slimk : {:#?}", String::from_utf8(curl.stderr));
    }
    return None;
}

pub fn get_releases_tags_from_github() -> Vec<String> {
    if let Some(releases) = get_releases_from_github() {
        if !releases.is_empty() {
            let mut tags = Vec::new();
            for item in releases {
                if let Some(map) = item.as_object() {
                    tags.push(map.get("tag_name").unwrap().as_str().unwrap().to_string());
                }
            }
            return tags;
        }
    }
    Vec::new()
}

/// get release download url form github
pub fn get_release_url_from_github(tag: Option<&str>) -> Option<String> {
    if let Some(version) = tag {
        let mut url = String::from("https://api.github.com/repos/Surrealism-All/slimk-template/releases/");
        if version.eq("latest") {
            url.push_str(version);
        } else {
            url.push_str(format!("tags/{}", version).as_str());
        }
        let curl = Command::new("curl").args([
            "-L", "-H", "Accept: application/vnd.github+json", "-H", "X-GitHub-Api-Version: 2022-11-28", &url
        ]).output().unwrap();
        if curl.status.success() {
            let stdout_str = String::from_utf8(curl.stdout).unwrap();
            let json: Value = serde_json::from_str(&stdout_str).unwrap();
            if let Some(map) = json.as_object() {
                let value = map.get("assets").unwrap();
                if let Some(assets) = value.as_array() {
                    return Some(assets[0].as_object().unwrap().get("browser_download_url").unwrap().as_str().unwrap().to_string());
                }
            }
        }
    }
    None
}

impl Display for UpdateStrategy {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "update strategy => native: {} cache: {}", self.native(), self.cache())
    }
}

/// 创建策略
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateStrategy {
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

impl Default for CreateStrategy {
    fn default() -> Self {
        CreateStrategy {
            remote: true,
            default: "slimk-binary".to_string(),
            secondary: "".to_string(),
            define: false,
        }
    }
}

#[allow(dead_code)]
impl CreateStrategy {
    pub fn new() -> Self {
        CreateStrategy::default()
    }
    pub fn remote(&self) -> bool {
        self.remote
    }
    pub fn default_repo(&self) -> &str {
        &self.default
    }
    pub fn secondary_repo(&self) -> &str {
        &self.secondary
    }
    pub fn define(&self) -> bool {
        self.define
    }
    pub fn set_remote(&mut self, remote: bool) {
        self.remote = remote;
    }
    pub fn set_default_repo(&mut self, name: &str) {
        self.default = String::from(name);
    }
    pub fn set_secondary_repo(&mut self, name: &str) {
        self.secondary = String::from(name);
    }
    pub fn set_define(&mut self, define: bool) {
        self.define = define;
    }
}

impl Display for CreateStrategy {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "create_command strategy =>\nAttempting to create_command a template using a remote repository : {}\ndefault template: {}\nsecondary template: {}\ndefine create_command: {}", self.remote(), self.default_repo(), self.secondary_repo(), self.define())
    }
}

/// Template for creating
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Template {
    /// The id of the repository
    id: usize,
    /// The url of the repository
    url: String,
    /// the note of the repository
    note: String,
}

impl Default for Template {
    fn default() -> Self {
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
        Template {
            id: timestamp as usize,
            url: "".to_string(),
            note: "".to_string(),
        }
    }
}

#[allow(dead_code)]
impl Template {
    pub fn new(url: &str, note: Option<&str>) -> Self {
        let mut template = Template::default();
        let _ = template.set_url(url);
        if let Some(note_text) = note {
            let _ = template.set_note(note_text);
        }
        template
    }

    pub fn set_url(&mut self, url: &str) {
        self.url = String::from(url);
    }
    pub fn set_note(&mut self, note: &str) {
        self.note = String::from(note);
    }
    pub fn url(&self) -> &str {
        &self.url
    }
    pub fn note(&self) -> &str {
        &self.note
    }
    pub fn default_template() -> Template {
        let mut template = Template::default();
        template.set_url(REMOTE_REPO);
        template.set_note(TEMPLATE_NOTE);
        template
    }
}

impl Display for Template {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "repo_id: {}\nrepo_url: {}\nnote: {}", self.id, self.url, self.note)
    }
}