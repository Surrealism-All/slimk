/// conf 作为配置文件目录
/// repo 作为存储库目录
/// log 作为日志目录
/// cache 作为存储库的缓存目录（15天清理）
pub const DIRS: [&str; 3] = ["conf", "repo", "cache"];
/// index.slint : global components in project
/// main.slint : then enter point
/// global.slint : global singleton
pub const UI_DIRS: [&str; 4] = ["assets", "components", "modules", "views"];
pub const UI_DIRS_ASSETS: [&str; 4] = ["font", "icons", "images", "others"];
pub const UI_FILES: [&str; 3] = ["index.slint", "global.slint", "app.slint"];
pub const UI_FILES_COMPONENTS: [&str; 2] = ["hello.slint", "index.slint"];
pub const UI_FILES_VIEWS: [&str; 2] = ["main.slint", "index.slint"];
/// 配置文件:slimk.json
#[allow(dead_code)]
pub const CONF_FILE: &str = "slimk.json";
pub const CONF_FILE_PATH: &str = "conf/slimk.json";
/// author
#[allow(dead_code)]
pub const AUTHOR: &str = "syf20020816@outlook.com";
/// standard remote repository: owner/repo
pub const REMOTE_REPO: &str = "https://github.com/Surrealism-All/slimk-template";
/// standard template note
pub const TEMPLATE_NOTE: &str = "Standard Template For Slint With SurrealismUI";

pub const BUILD_RS: &str = r#"fn main() {
    slint_build::compile("ui/main.slint").unwrap();
}"#;

pub const README_MD: &str = r#"# Slint + SurrealismUI + Rust

This template should help get you started developing with `Slint` and `Rust`. And this template use `SurrealismUI` as default Component Library.

## Recommand IDE

VSCode

## Recommand Plugin

- Slint
- rust
- rust-analyzer

## About Slimk Commands

### Create Slint Project

create_command a project by selecting configuration items

```bash
# use default strategy to create_command a new project
> slimk create_command hello
# create_command a new project with a template
> slimk create_command hello --template slimk
> slimk create_command hello -t slimk
```
### Init an empty Slint Project

this command creates a new project but use the default strategy with no template , you will get an empty slint project

> you do not need to name the project , this way will use your root directory

```bash
> slimk init
```

### Select Templates(Native,Remote)

```bash
# native
> slimk list -n
# remote
> slimk list -r
# both
> slimk list -a
```"#;

pub const INDEX_SLINT: &str = r#"//export views
export * from "./views/index.slint";"#;

pub const GLOBAL_SLINT: &str = r#"export global ROOT_GLOBAL {
  in-out property <length> window-height : 600px;
  in-out property <length> window-width : 800px;
  in-out property <length> font-size : 16px;
  in-out property <length> padding : 0px;
}"#;

pub const APP_SLINT: &str = r#"import {MainView} from "./index.slint";
import {ROOT_GLOBAL} from "./global.slint";

export component App inherits Window {
  height: ROOT-GLOBAL.window-height;
  width: ROOT-GLOBAL.window-width;
  title: @tr("Slint + SurrealismUI + Rust");
  MainView {}
}"#;

pub const MAIN_SLINT_VIEWS: &str = r#"import {Hello} from "../components/index.slint";

export component MainView {
  height: 100%;
  width: 100%;
  VerticalLayout {
    Hello{}
    Rectangle {
      Button {
        theme:Dark;
        text: "Click Me!";
      }
    }
  }
}"#;
pub const INDEX_SLINT_VIEWS: &str = r#"import { MainView } from "./main.slint";

export { MainView }"#;
pub const HELLO_SLINT_COMPONENTS: &str = r#"import { AboutSlint } from "std-widgets.slint";
export component HelloComponent {
  Rectangle {
    layout := VerticalLayout {
      spacing: 16px;
      AboutSlint {
        height: 260px;
      }
      Text {
        horizontal-alignment: center;
        vertical-alignment: center;
        font-size: 42px;
        font-weight: 700;
        color: #2379F4;
        text: "Slint + SurrealismUI + Rust";
      }
      Text {
        horizontal-alignment: center;
        vertical-alignment: center;
        font-size: 36px;
        font-weight: 700;
        color: #2a6dcb;
        text: "Made By Slimk";
      }
    }
  }
}"#;
pub const INDEX_SLINT_COMPONENTS: &str = r#"import { HelloComponent } from "./hello.slint";

export {
  HelloComponent as Hello
}"#;
