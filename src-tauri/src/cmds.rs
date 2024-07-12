use crate::{
    config::{Config, Group, ISettings, NVersion, Project},
    core::{group, node, project},
    ret_err, wrap_err,
};

use anyhow::Result;

type CmdResult<T = ()> = Result<T, String>;

/// get current version
#[tauri::command]
pub fn current(fetch: Option<bool>) -> CmdResult<Option<String>> {
    wrap_err!(node::get_current(fetch))
}

/// set current version
#[tauri::command]
pub async fn set_current(version: Option<String>) -> CmdResult<()> {
    wrap_err!(node::set_current(version).await)
}

/// fetch node version list
#[tauri::command]
pub async fn version_list(fetch: Option<bool>) -> CmdResult<Option<Vec<NVersion>>> {
    wrap_err!(node::get_version_list(fetch).await)
}

/// read node installed version list
#[tauri::command]
pub async fn installed_list(fetch: Option<bool>) -> CmdResult<Option<Vec<String>>> {
    wrap_err!(node::get_installed_list(fetch).await)
}

/// read settings
#[tauri::command]
pub async fn read_settings() -> CmdResult<ISettings> {
    Ok(Config::settings().data().clone())
}

/// update settings
#[tauri::command]
pub async fn update_settings(settings: ISettings) -> CmdResult<()> {
    Config::settings().apply();
    wrap_err!({ Config::settings().data().patch_settings(settings) })?;

    Ok(())
}

/// install node
#[tauri::command]
pub async fn install_node(
    window: tauri::Window,
    version: Option<String>,
    arch: Option<String>,
) -> CmdResult<String> {
    wrap_err!(node::install_node(window, version, arch).await)
}

/// install node
#[tauri::command]
pub async fn install_node_cancel() -> CmdResult<()> {
    wrap_err!(node::install_node_cancel().await)
}

/// uninstall node
#[tauri::command]
pub async fn uninstall_node(version: Option<String>, current: Option<bool>) -> CmdResult<()> {
    if version.is_none() {
        ret_err!("version should not be null");
    }

    let version = version.unwrap();
    wrap_err!(node::uninstall_node(version, current).await)
}

/// get project list
#[tauri::command]
pub async fn project_list(fetch: Option<bool>) -> CmdResult<Option<Vec<Project>>> {
    wrap_err!(project::project_list(fetch).await)
}

/// add projects
#[tauri::command]
pub async fn add_projects(app_handle: tauri::AppHandle) -> CmdResult<Option<Vec<project::PInfo>>> {
    wrap_err!(project::add_projects(app_handle).await)
}

/// get group list
#[tauri::command]
pub async fn group_list(fetch: Option<bool>) -> CmdResult<Option<Vec<Group>>> {
    wrap_err!(group::group_list(fetch).await)
}

/// exit app
#[tauri::command]
pub fn exit_app(app_handle: tauri::AppHandle) {
    app_handle.exit(0);
    std::process::exit(0);
}
