<div align="center">
  <img src="https://github.com/1111mp/nvm-desktop/assets/31227919/67132758-8aa9-4b05-b987-18fdd5980936"/>
</div>

# Node Version Manager Desktop

`nvm-desktop` 是一个以可视化界面操作方式管理多个 Node 版本的桌面应用，使用 [Tauri](https://v2.tauri.app/) 构建（支持 `macOS`、`Windows` 以及 `Linux` 系统）。通过该应用，您可以快速安装、管理和使用不同版本的 Node。

完美支持为不同的项目单独设置和切换 Node 版本，不依赖操作系统的任何特定功能和 shell。

智能（快速）识别正确 Node 引擎版本的功能由另一个项目：[nvmd-command](https://github.com/1111mp/nvmd-command) 提供支持，它是一个单一、快速的本机可执行文件，没有外部依赖项，使用 Rust 构建。

[English](https://github.com/1111mp/nvm-desktop/blob/tauri/README.md) | 简体中文

## 目录

- [截图](#截图)
- [一些或许你需要知道的事情](#一些或许你需要知道的事情)
- [命令行工具](#命令行工具)
- [安装](#安装)
  - [下载](#下载)
- [卸载](#卸载)
  - [macOS 卸载](#macOS-卸载)
  - [Windows 卸载](#Windows-卸载)
- [开发和构建](#开发和构建)
  - [开发](#开发)
  - [构建生产包](#构建生产包)
- [管理您的项目](#管理您的项目)
- [功能](#功能)

## 截图

<img width="1029" alt="Screenshot 2024-10-05 at 10 09 27" src="https://github.com/user-attachments/assets/1103871f-5e47-4f96-b71c-3805fdfd694f">

<img width="1030" alt="Screenshot 2024-10-05 at 10 08 31" src="https://github.com/user-attachments/assets/d8005347-a671-4c25-a776-658b258fe06e">

## 一些或许你需要知道的事情

所有与`nvm-desktop`相关的文件都保存在`"$HOME/.nvmd/"`目录下：

- `"bin/"`(文件夹) **保存着所有 Node.js 可执行文件的垫片**。 目录 `"$HOME/.nvmd/bin` 需要添加到系统的环境变量里面。

  | macOS        | Windows                      |
  | :---:        | :---:                        |
  | `nvmd`       | `nvmd.exe`                   |
  | `node`       | `node.exe`                   |
  | `npm`        | `npm.exe npm.cmd`            |
  | `npx`        | `npx.exe npx.cmd`            |
  | `corepack`   | `corepack.exe corepack.cmd`  |

- `"versions/"`(文件夹) **保存着所有下载的 Node.js 版本的文件，文件名一般以 Node.js 的版本号为**。 例如： `"$HOME/.nvmd/versions/21.2.0"`.
- `"default"`(文件) **文件内容为全局设置 Node.js 版本的版本号**， 例如： `21.2.0`.
- `"migration"`(文件) `nvm-desktop` 每次升级时都会根据这个文件来控制脚本代码的执行。
- `"setting.json"`(文件) **保存着 `nvm-desktop` 设置中心的设置信息**， 比如 `Theme, Language, Mirror Url` 等。
  ```json
  {
    "locale": "en",
    "theme": "system",
    "closer": "minimize",
    "directory": "/Users/********/.nvmd/versions",
    "mirror": "https://nodejs.org/dist"
  }
  ```
- `"projects.json"`(文件) **保存着所有添加过的项目的信息**。
  ```json
  [
    {
      "name": "nvm-desktop",
      "path": "/Users/********/Documents/Electron/nvm-desktop",
      "version": "20.6.1",
      "active": true,
      "createAt": "2023-11-25T04:07:43.012Z",
      "updateAt": "2023-11-25T04:07:44.931Z"
    },
    {
      "name": "electron_client",
      "path": "/Users/********/Documents/projects/electron_client",
      "version": "20.6.1",
      "active": true,
      "createAt": "2023-11-25T04:07:35.172Z",
      "updateAt": "2023-11-25T04:07:37.234Z"
    }
  ]
  ```
- `"packages.json"`(文件) **保存着 npm 全局安装的包的相关的信息**。 更多信息请查看 [how-does-it-work](https://github.com/1111mp/nvmd-command#how-does-it-work).
- `"versions.json"`(文件) 保存着从 `"https://nodejs.org/dist"`（默认） 请求过来的所有的 Node.js  的版本的信息。
  ```json
  [
    {
      "version": "v21.2.0",
      "date": "2023-11-14",
      "files": [
        "aix-ppc64",
        "headers",
        "linux-arm64",
        "linux-armv7l",
        "linux-ppc64le",
        "linux-s390x",
        "linux-x64",
        "osx-arm64-tar",
        "osx-x64-pkg",
        "osx-x64-tar",
        "src",
        "win-arm64-7z",
        "win-arm64-zip",
        "win-x64-7z",
        "win-x64-exe",
        "win-x64-msi",
        "win-x64-zip",
        "win-x86-7z",
        "win-x86-exe",
        "win-x86-msi",
        "win-x86-zip"
      ],
      "npm": "10.2.3",
      "v8": "11.8.172.17",
      "uv": "1.46.0",
      "zlib": "1.2.13.1-motley",
      "openssl": "3.0.12+quic",
      "modules": "120",
      "lts": false,
      "security": false
    },
  ]
  ```

## 命令行工具

您可以直接在终端中输入命令行管理所有 Node 版本。 `nvmd` 不提供 Node 的下载安装功能，如果您需要下载安装新版本的 Node，请打开 `nvm-desktop` 客户端。

`nvmd` 允许您通过命令行快速管理不同版本的 Node：

```shell
$ nvmd use 18.17.1
Now using node v18.17.1
$ node -v
v18.17.1
$ nvmd use v20.5.1 --project
Now using node v20.5.1
$ node -v
v20.5.1
$ nvmd ls
v20.6.1
v20.5.1 (currently)
v18.17.1
$ nvmd current
v20.5.1
```

`nvmd --help`：

```shell
$ nvmd --help
nvmd (2.2.0)
command tools for nvm-desktop

Usage: nvmd [COMMAND]

Commands:
  current  Get the currently used version
  list     List the all installed versions of Node.js
  ls       List the all installed versions of Node.js
  use      Use the installed version of Node.js (default is global)
  which    Get the path to the executable to where Node.js was installed
  help     Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version

Please download new version of Node.js in nvm-desktop.
```

在你通过`nvmd use` 命令行切换Node版本之后，请点击刷新按钮让`nvm-desktop`同步最新的数据。

更多详情请查看此文档: [command-tools-intro](https://github.com/1111mp/nvmd-command#command-tools-intro) .

## 安装

### 下载

您可以下载源代码并自行构建，或者从以下链接下载最新构建的版本：

- [nvmd-desktop 下载页面 (GitHub release)](https://github.com/1111mp/nvm-desktop/releases)

应用程序的自动检查更新功能从 `v4.0.0` 版本开始已支持全平台。

## 卸载

### macOS 卸载

- 卸载 `nvm-desktop` 应用程序
- `rm -rf ~/.nvmd`
- 从 `shell` 的配置文件中删除关于 `nvmd` 的两行：

  ```shell
  export NVMD_DIR="$HOME/.nvmd"
  export PATH="$NVMD_DIR/bin:$PATH"
  ```
  默认的文件可能是：
    - .zshrc
    - .bashrc
    - .bash_profile
    - .profile

### Windows 卸载

- 卸载 `nvm-desktop` 应用程序
- 删除 `%HOMEPATH%\.nvmd` 文件
- 移除系统环境变量：`%HOMEPATH%\.nvmd\bin`（从 `v4.0.0` 开始卸载时会自动移除）

## 开发和构建

`nvm-desktop` 依赖 `nvmd-command` 提供智能识别正确 Node 引擎版本的功能，所以你需要在本地提前为 `nvm-desktop` 构建一个可执行文件。关于如何构建 `nvmd-command` 的可执行文件，请查看此文档： [build-nvmd-command](https://github.com/1111mp/nvmd-command#build-nvmd-command)。

- 首先提前为 `nvm-desktop` 构建一个可执行文件
- 将这个可执行文件复制到 `nvm-desktop` 的指定目录下：
  - macOS `"./assets/sources/nvmd"`
  - Windows `"./assets/sources/{arch}.exe"`, 例如: `"./assets/sources/x64.exe"` & `"./assets/sources/arm64.exe"`
- 在 Windows 平台，你还需要添加一个名为 `temp.cmd` 的文件到指定目录： `./assets/sources/temp.cmd`， `temp.cmd` 文件的内容为:

```shell
@echo off
"%~dpn0.exe" %*
```

然后你就可以开始在本地运行和构建 `nvm-desktop` 了。

从 `v4.0.0` 版本开始，已经迁移到 `tauri`，所以已经不需要上述操作了，直接运行 `pnpm check` 命令即可。

### 开发

- 首先，你应该在本地安装 `Rust` 运行环境。请阅读官方指南：[rust get started](https://www.rust-lang.org/learn/get-started)
- 其次，确保你本地已经安装过 [Node.js](https://nodejs.org/) 了
- 去到项目的根目录，然后在终端运行：`pnpm install` 命令为项目安装依赖

有两种方式启动开发服务器：

- 使用 `pnpm dev` 命令
- `F5` 按键一键启动（Debug 模式）

### 构建生产包

- 去到项目根目录
- 执行 `pnpm build` 命令， 如果一切工作都正常运行的话，你可以在 `./src-tauri/target/release/bundle` 目录下找到构建好的包文件

## 管理您的项目

现在，您可以为您的项目单独选择不同的 Node 版本，无需任何其他依赖项和额外工作。

此功能依赖 `nvmd-command` 的底层支持。

更多详情，请查看：[nvmd-command](https://github.com/1111mp/nvmd-command) 项目代码。

<img width="1660" alt="image" src="https://github.com/1111mp/nvm-desktop/assets/31227919/ac8653c4-5b40-447f-b10c-557907d101df">

在你项目的根目录下会添加一个文件：`.nvmdrc`，文件的内容为你在 `nvm-desktop` 界面中为该项目选择的 Node 引擎的版本号。`nvm-desktop` 检测此文件以识别你项目的 Node 版本。

## 功能

- [x] 支持为系统全局和项目单独设置Node引擎版本
- [x] 管理Node的命令行工具
- [x] 支持英文和简体中文
- [x] 支持自定义下载镜像地址 (默认是 https://nodejs.org/dist)
- [x] 自动检查更新
