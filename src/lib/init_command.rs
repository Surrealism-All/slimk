use std::collections::HashMap;
use std::fs::{create_dir, File};
use std::io::Write;
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};
use clap::Args;
use super::{get_work_path, format_dir_name, create_files, create_dirs, combine_to_map};
use super::constant::{
    BUILD_RS, README_MD, UI_FILES, UI_FILES_VIEWS, UI_DIRS, MAIN_SLINT_VIEWS, UI_DIRS_ASSETS, UI_FILES_COMPONENTS,
    HELLO_SLINT_COMPONENTS, INDEX_SLINT, INDEX_SLINT_COMPONENTS, INDEX_SLINT_VIEWS, APP_SLINT, GLOBAL_SLINT,
};

#[derive(Args, Debug, Clone)]
pub struct InitCommand {
    name: Option<String>,
}

#[allow(dead_code)]
impl InitCommand {
    fn name(&self) -> &str {
        return if let Some(name) = &self.name {
            name
        } else {
            ""
        };
    }
    pub fn cargo_new(&self) {
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
        let name = String::from(self.name());
        let cmd = Command::new("cargo").args(&["init", &name]).output().expect("cargo init fail, please check your environment or rust toolchain is not installed in your computer!");
        if cmd.status.success() {
            println!("Slimk - [{}] : cargo init ---> (success)", timestamp);
        } else {
            eprintln!("Slimk - [{}] : {} ---> (fail)", timestamp, String::from_utf8_lossy(&cmd.stderr));
        }
    }
    pub fn cargo_add_slint(&self) {
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
        let path = get_work_path(self.name());
        // add slint
        let slint = Command::new("cargo").args(&["add", "slint"]).current_dir(path.as_path()).output().expect("cargo add slint fail, please check your network!");
        if slint.status.success() {
            println!("Slimk - [{}] : cargo add slint ---> (success)", timestamp);
        } else {
            eprintln!("Slimk - [{}] : {} ---> (fail)", timestamp, String::from_utf8_lossy(&slint.stderr));
        }
        // add slint-build
        let slint_build = Command::new("cargo").args(&["add", "slint-build", "--build"]).current_dir(path.as_path()).output().expect("cargo add slint-build fail, please check your network!");
        if slint_build.status.success() {
            println!("Slimk - [{}] : cargo add slint-build --build ---> (success)", timestamp);
        } else {
            eprintln!("Slimk - [{}] : {} ---> (fail)", timestamp, String::from_utf8_lossy(&slint_build.stderr));
        }
    }
    pub fn add_build_rs(&self) {
        let build_rs = format_dir_name(self.name(), "build.rs");
        match File::create(build_rs.as_path()).unwrap().write_all(BUILD_RS.as_bytes()) {
            Ok(_) => println!("Slimk : Add build.rs ---> (success)"),
            Err(e) => panic!("Slimk : Add build.rs ---> (fail) => reason :\n{}", e),
        };
    }
    pub fn add_readme(&self) {
        let readme = format_dir_name(self.name(), "README.md");
        match File::create(readme.as_path()).unwrap().write_all(README_MD.as_bytes()) {
            Ok(_) => println!("Slimk : Add README.md ---> (success)"),
            Err(e) => panic!("Slimk : Add README.md ---> (fail) => reason :\n{}", e),
        };
    }
    pub fn create_ui_dir(&self) {
        let ui_path = format_dir_name(self.name(), "ui");
        let _ = create_dir(&ui_path.as_path());
        let _ = create_dirs(&ui_path, UI_DIRS.to_vec());
        let assets_path = &ui_path.join("assets");
        let _ = create_dirs(assets_path, UI_DIRS_ASSETS.to_vec());
        let mut map1: HashMap<String, String> = HashMap::new();
        let _ = combine_to_map(&mut map1, UI_FILES.to_vec(), vec![INDEX_SLINT, GLOBAL_SLINT, APP_SLINT]);
        let _ = create_files(&ui_path, map1);
        let components_path = &ui_path.join("components");
        let mut map2: HashMap<String, String> = HashMap::new();
        let _ = combine_to_map(&mut map2, UI_FILES_COMPONENTS.to_vec(), vec![HELLO_SLINT_COMPONENTS, INDEX_SLINT_COMPONENTS]);
        let _ = create_files(components_path, map2);
        let views_path = &ui_path.join("views");
        let mut map3: HashMap<String, String> = HashMap::new();
        let _ = combine_to_map(&mut map3, UI_FILES_VIEWS.to_vec(), vec![MAIN_SLINT_VIEWS, INDEX_SLINT_VIEWS]);
        let _ = create_files(views_path, map3);
    }
}
