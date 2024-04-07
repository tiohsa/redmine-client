// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use tauri::{
    CustomMenuItem, LogicalSize, Manager, Size, SystemTray, SystemTrayEvent, SystemTrayMenu,
    SystemTrayMenuItem,
};
use tauri_plugin_positioner::{Position, WindowExt};

#[derive(Serialize, Deserialize, Debug)]
struct Issue {
    project_id: String,
    subject: String,
    description: Option<String>,
    tracker_id: String,
    status_id: String,
    priority_id: String,
    category_id: Option<String>,
    fixed_version_id: Option<i32>,
    assigned_to_id: Option<i32>,
    parent_issue_id: Option<i32>,
    custom_fields: Option<i32>,
    watcher_user_ids: Option<i32>,
    is_private: Option<bool>,
    estimated_hours: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Issues {
    issue: Vec<Issue>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Config {
    url: String,
    token: String,
    project_id: String,
}

// https://jonaskruckenberg.github.io/tauri-docs-wip/development/inter-process-communication.html#error-handling
#[derive(Debug, thiserror::Error)]
enum Error {
    #[error("Failed to read file: {0}")]
    Io(#[from] std::io::Error),
    #[error("File is not valid utf8: {0}")]
    Utf8(#[from] std::string::FromUtf8Error),
    #[error("Failed to request: {0}")]
    Reqwest(#[from] reqwest::Error),
    #[error("Failed to convert json: {0}")]
    SerdeJson(#[from] serde_json::Error),
    #[error("Failed : {0}")]
    Err(#[from] anyhow::Error),
}

// we must also implement serde::Serialize
impl serde::Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
async fn post_issues(issue: String, config: String) -> Result<String, Error> {
    let config_json: Config = serde_json::from_str(&config)?;
    let issue_json: serde_json::Value = serde_json::from_str(&issue)?;

    let resp = reqwest::Client::new()
        .post(format!("{}/issues.json", config_json.url))
        .header("X-Redmine-API-Key", config_json.token)
        .json(&issue_json)
        .send()
        .await?;
    println!("{}", &resp.status());
    Ok(resp.status().to_string())
}

#[tauri::command]
async fn get_issue_categories(config: String) -> Result<String, Error> {
    let config_json: Config = serde_json::from_str(&config)?;

    let resp = reqwest::Client::new()
        .get(format!(
            "{}/projects/{}/issue_categories.json",
            config_json.url, config_json.project_id
        ))
        .header("X-Redmine-API-Key", config_json.token)
        .send()
        .await?;
    println!("{}", &resp.status());
    let json = resp.json::<serde_json::Value>().await?;
    println!("{}", &json.to_string());
    Ok(json["issue_categories"].to_string())
}

const SETTING_FILE_NAME: &str = "redmine-client.json";

#[tauri::command]
fn save_config(config: String) -> Result<(), Error> {
    let dir = tauri::api::path::config_dir().ok_or(anyhow!("Could not find config directory"))?;
    let path = dir.join(SETTING_FILE_NAME);
    std::fs::write(path, config)?;
    Ok(())
}

#[tauri::command]
fn read_config() -> Result<String, Error> {
    let dir = tauri::api::path::config_dir().ok_or(anyhow!("Could not find config directory"))?;
    let path = dir.join(SETTING_FILE_NAME);
    let data = std::fs::read_to_string(path)?;
    Ok(data)
}

fn main() {
    // system tray
    // https://tauri.app/v1/guides/features/system-tray/
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let hide = CustomMenuItem::new("hide".to_string(), "Hide");
    let tray_menu = SystemTrayMenu::new()
        .add_item(quit)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(hide);
    let system_tray = SystemTray::new().with_menu(tray_menu);

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            post_issues,
            get_issue_categories,
            read_config,
            save_config
        ])
        .plugin(tauri_plugin_positioner::init())
        .setup(|app| {
            let window = app.get_window("main").unwrap();
            // window.hide().unwrap();
            let _ = window.move_window(Position::BottomRight);
            window
                .set_size(Size::Logical(LogicalSize {
                    width: 400.0,
                    height: 600.0,
                }))
                .unwrap();
            // btw don't use .unwrap() here, i'm just lazy.
            Ok(())
        })
        .system_tray(system_tray)
        .on_window_event(|event| match event.event() {
            tauri::WindowEvent::CloseRequested { api, .. } => {
                event.window().hide().unwrap();
                api.prevent_close()
            }
            _ => {}
        })
        .on_system_tray_event(|app, event| match event {
            SystemTrayEvent::MenuItemClick { id, .. } => {
                // let item_handle = app.tray_handle().get_item(&id);
                match id.as_str() {
                    "hide" => {
                        let window = app.get_window("main").unwrap();
                        window.hide().unwrap();
                    }
                    "quit" => {
                        std::process::exit(0);
                    }
                    _ => {}
                }
            }
            SystemTrayEvent::LeftClick { .. } => {
                let window = app.get_window("main").unwrap();
                let _ = window.move_window(Position::BottomRight);
                window.show().unwrap();
            }
            _ => {}
        })
        .build(tauri::generate_context!())
        .expect("error while running tauri application")
        .run(|_app_handle, event| match event {
            tauri::RunEvent::ExitRequested { api, .. } => {
                api.prevent_exit();
            }
            _ => {}
        });
}
