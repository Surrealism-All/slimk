use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use serde::{Serialize, Deserialize};
use super::{REMOTE_REPO, TEMPLATE_NOTE};
use std::time::{SystemTime, UNIX_EPOCH};

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
        let mut remotes: HashMap<String, Template> = HashMap::new();
        let _ = remotes.insert(String::from("slimk"), Template::default_template());
        Conf {
            user: None,
            email: None,
            remotes,
            natives:Default::default(),
            create_strategy: CreateStrategy::default(),
            update_strategy: UpdateStrategy::default(),
        }
    }
}

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
    pub fn email(&self) -> &Option<String> {
        &self.email
    }
    pub fn natives(&self) -> &HashMap<String, Template> {
        &self.natives
    }
    pub fn remotes(&self) -> &HashMap<String, Template> {
        &self.remotes
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
}


/// 更新策略只针对本地仓库和缓存
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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

impl Default for UpdateStrategy {
    fn default() -> Self {
        UpdateStrategy {
            native: 15,
            cache: 7,
        }
    }
}

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
    pub fn set_native(&mut self, native: i32) {
        self.native = native;
    }
    pub fn set_cache(&mut self, cache: i32) {
        self.cache = cache
    }
}

impl Display for UpdateStrategy {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "update strategy => native: {} cache: {}", self.native(), self.cache())
    }
}

/// 创建策略
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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

impl Default for CreateStrategy {
    fn default() -> Self {
        CreateStrategy {
            remote: true,
            default: "slimk".to_string(),
            secondary: "".to_string(),
            define: false,
        }
    }
}

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
        write!(f, "create strategy =>\nAttempting to create a template using a remote repository : {}\ndefault template: {}\nsecondary template: {}\n define create: {}", self.remote(), self.default_repo(), self.secondary_repo(), self.define())
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