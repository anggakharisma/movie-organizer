// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod cache;
mod config;
mod directory;
mod error;
mod movie;

use std::{fs::File, vec};

use cache::create_cache_movie_file;
use config::set_key_value;
use directories::ProjectDirs;
use movie::get_movie_list;
use tauri::Manager;

#[tauri::command]
fn initialize_config() {
    let proj_dirs = ProjectDirs::from("com", "dashdev", "seirei").expect("Can't get file");

    // Create directory for config, OS based configuration file
    let path = std::path::Path::new(proj_dirs.config_dir());

    create_cache_movie_file();

    // Prevent directory from recreating
    if !path.exists() || !path.join("config.toml").exists() {
        std::fs::create_dir_all(path).unwrap();
        File::create(path.join("config.toml")).unwrap();
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    initialize_config();

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            get_movie_list,
            set_key_value,
            config::get_user_config
        ])
        .setup(|app| {
            #[cfg(debug_assertions)]
            app.get_window("main").unwrap().open_devtools();
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    Ok(())
}
