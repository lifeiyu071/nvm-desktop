// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod cmds;
mod config;
mod core;
mod utils;

use config::Config;
use utils::resolve;

fn main() -> tauri::Result<()> {
    let builder = tauri::Builder::default()
        .setup(|app| {
            resolve::resolve_setup(app)?;
            Ok(())
        })
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            // settings
            cmds::read_settings,
            cmds::update_settings,
            // node
            cmds::current,
            cmds::set_current,
            cmds::version_list,
            cmds::installed_list,
            cmds::install_node,
            cmds::uninstall_node,
            cmds::install_node_cancel,
            // projects
            cmds::project_list,
            cmds::add_projects,
            cmds::update_projects,
            cmds::sync_project_version,
            cmds::batch_update_project_version,
            // groups
            cmds::group_list,
            cmds::update_groups,
        ]);

    let app = builder.build(tauri::generate_context!())?;

    app.run(|app_handle, err| match err {
        tauri::RunEvent::ExitRequested { api, .. } => {
            let closer = Config::settings()
                .data()
                .get_closer()
                .unwrap_or("minimize".to_string());

            if closer == "minimize" {
                api.prevent_exit();
            }
        }
        tauri::RunEvent::WindowEvent { label, event, .. } => {
            if label == "main" {
                match event {
                    tauri::WindowEvent::Destroyed => {
                        // Destroyed Event
                    }
                    tauri::WindowEvent::CloseRequested { api, .. } => {
                        // CloseRequested Event
                    }
                    _ => {}
                }
            }
        }
        _ => {}
    });

    Ok(())
}
