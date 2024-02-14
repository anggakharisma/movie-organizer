use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};

use directories::ProjectDirs;

#[derive(Serialize, Debug, Deserialize)]
pub struct Config {
    pub movie_dir: String,
}

#[tauri::command]
pub fn get_user_config() -> PathBuf {
    let config_file = ProjectDirs::from("com", "dashdev", "seirei")
        .expect("Can't get file")
        .config_dir()
        .join("config.toml");
    config_file
}

#[tauri::command]
pub fn get_selected_movie_dir() -> String {
    let user_config = get_user_config();
    let f: String = fs::read_to_string(user_config).unwrap().parse().unwrap();

    if f.is_empty() {
        return String::from("");
    }

    let config: Config = toml::from_str(&f).unwrap();

    if config.movie_dir.is_empty() {
        return String::from("");
    }

    config.movie_dir
}

#[tauri::command]
pub fn set_selected_movie_dir(dir: String) {
    let user_config = get_user_config();

    let config = Config { movie_dir: dir };

    let toml = toml::to_string(&config).unwrap();
    fs::write(user_config, &toml).unwrap();
}
