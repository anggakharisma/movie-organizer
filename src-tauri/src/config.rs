use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};

use directories::ProjectDirs;

#[derive(Serialize, Debug, Deserialize, Clone)]
pub struct Config {
    pub movie_dir: Option<String>,
    pub api_key: Option<String>,
}

impl Config {
    pub fn get_key(self, key: &str) -> Result<String, String> {
        match key {
            "movie_dir" => Ok(self.movie_dir.unwrap()),
            "api_key" => Ok(self.api_key.unwrap()),
            _ => Err(format!("Err")),
        }
    }

    pub fn set_key(&mut self, key: &str, val: &str) -> Result<(), String> {
        if val.is_empty() {
            return Err(format!("Err"));
        }

        match key {
            "movie_dir" => {
                self.movie_dir = Some(val.to_string());
                Ok(())
            }
            "api_key" => {
                self.api_key = Some(val.to_string());
                Ok(())
            }
            _ => Err(format!("Err")),
        }
    }
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

    let config_from_toml: Config = toml::from_str(&f).unwrap();
    if config_from_toml.movie_dir.is_none() {
        return String::from("");
    }

    config_from_toml.movie_dir.expect("wrong")
}

#[tauri::command]
pub fn set_key_value(key: &str, val: &str) {
    let user_config = get_user_config();
    let mut config = Config {
        movie_dir: Default::default(),
        api_key: Default::default(),
    };

    config.set_key(key, val).ok();
    let toml = toml::to_string(&config).unwrap();
    fs::write(user_config, &toml).unwrap();
}
