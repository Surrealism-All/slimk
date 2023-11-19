# Slimk

A package manager for creating Slint with SurrealismUI

- author : syf20020816@outlook.com
- createDate : 20231115
- updateDate : 20231119
- version : 0.0.1

## Command

### Create Slint Project

```bash
# use default strategy to create a new project
> slimk create hello 
# create a new project with a template
> slimk create hello --template slimk
> slimk create hello -t slimk
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
```

## Goals

- [x] : create command
- [ ] : init command
- [ ] : list command
- [ ] : update command
- [x] : --list options