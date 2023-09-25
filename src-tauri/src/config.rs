use std::path::PathBuf;

use directories::ProjectDirs;

#[tauri::command]
pub fn get_user_config() -> PathBuf {
    let config_file = ProjectDirs::from("com", "dashdev", "sushei")
        .expect("Can't get file")
        .config_dir()
        .with_file_name("config.toml");

    config_file
}
