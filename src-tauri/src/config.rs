use std::path::PathBuf;

use directories::ProjectDirs;

#[tauri::command]
pub fn get_user_config() -> PathBuf {
    let config_file = ProjectDirs::from("com", "dashdev", "seirei").expect("Can't get file");

    let config_file = ProjectDirs::from("com", "dashdev", "sushei")
        .expect("Can't get file")
        .config_dir()
        .join("config.toml");

    config_file
}
