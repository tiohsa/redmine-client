// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use tauri::{utils::config::AppUrl, window::WindowBuilder, WindowUrl};
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
    tracker_id: i32,
    status_id: i32,
    priority_id: i32,
    category_id: Option<i32>,
    fixed_version_id: Option<i32>,
    assigned_to_id: Option<i32>,
    parent_issue_id: Option<i32>,
    custom_fields: Option<i32>,
    watcher_user_ids: Option<i32>,
    is_private: Option<bool>,
    estimated_hours: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Config {
    url: String,
    token: String,
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
//$result = sendPostRequest($url, [
//   'issue' => [
//     'project_id' => 1,          // プロジェクト
//     'subject'    => 'APIテスト',  // 題名
//     // 以下は任意で指定する
//     //'description'      => '',  // 説明文
//     //'tracker_id'       => '',  // トラッカー
//     //'status_id'        => '',  // ステータス
//     //'priority_id'      => '',  // 優先度
//     //'category_id'      => '',  // カテゴリ
//     //'fixed_version_id' => '',  // 対象バージョン
//     //'assigned_to_id'   => '',  // 担当者
//     //'parent_issue_id'  => '',  // 親チケット
//     //'custom_fields'    => '',  // カスタムフィールド
//     //'watcher_user_ids' => '',  // ウォッチャー(ユーザーID) v2.3.0〜
//     //'is_private'       => '',  // プライベートチケットにするか true or false
//     //'estimated_hours'  => ''   // 予定工数
//   ]
#[tauri::command]
async fn issue(json: String, config: String) -> Result<String, Error> {
    let config_json: Config = serde_json::from_str(&config)?;
    let issue_json: serde_json::Value = serde_json::from_str(&json)?;

    println!("{}", format!("{}/issues.json", config_json.url));
    let resp = reqwest::Client::new()
        .post(format!("{}/issues.json", config_json.url))
        .header("X-Redmine-API-Key", config_json.token)
        .json(&issue_json)
        .send()
        .await?;
    println!("{}", &resp.status());
    Ok(resp.status().to_string())
}

const SETTING_FILE_NAME: &str = "redmine-client.json";

#[tauri::command]
fn save(json: String) -> Result<(), Error> {
    let dir = tauri::api::path::config_dir().ok_or(anyhow!("Could not find config directory"))?;
    let path = dir.join(SETTING_FILE_NAME);
    std::fs::write(path, json)?;
    Ok(())
}

#[tauri::command]
fn read() -> Result<String, Error> {
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

    // let port = portpicker::pick_unused_port().expect("failed to find unused port");
    // let mut context = tauri::generate_context!();
    // let url = format!("http://localhost:{}", port).parse().unwrap();
    // let window_url = WindowUrl::External(url);
    // // rewrite the config so the IPC is enabled on this URL
    // context.config_mut().build.dist_dir = AppUrl::Url(window_url.clone());

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![issue, read, save])
        .plugin(tauri_plugin_positioner::init())
        // .plugin(tauri_plugin_localhost::Builder::new(port).build())
        .setup(|app| {
            // WindowBuilder::new(
            //     app,
            //     "main".to_string(),
            //     if cfg!(dev) {
            //         Default::default()
            //     } else {
            //         window_url
            //     },
            // )
            // .build()?;
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
