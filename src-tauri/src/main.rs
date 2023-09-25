// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod config;
mod directory;
mod movie;

use std::{
    fs::{self, File},
    vec,
};

use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tauri::Manager;

#[derive(Serialize, Debug, Deserialize)]
struct Config {
    movie_dir: String,
}

#[derive(Serialize, Debug, Deserialize)]
struct Movie {
    poster: String,
    year: usize,
    name: String,
    path: String,
    category: String,
}

#[tauri::command]
fn cache_movies(movies: &mut Vec<Movie>) {
    //println!("{:?}", movies[0]);
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn get_selected_dir() -> String {
    let user_config = config::get_user_config();
    let f: String = fs::read_to_string(user_config).unwrap().parse().unwrap();

    if f.is_empty() {
        return "".to_string();
    }

    let config: Config = toml::from_str(&f).unwrap();

    if config.movie_dir.is_empty() {
        return "".to_string();
    }

    config.movie_dir
}

#[tauri::command]
fn set_selected_dir(dir: String) {
    let user_config = config::get_user_config();

    let config = Config { movie_dir: dir };

    let toml = toml::to_string(&config).unwrap();
    fs::write(user_config, &toml).unwrap();
}

#[tauri::command]
fn initialize_config() {
    let proj_dirs = ProjectDirs::from("com", "dashdev", "sushei").expect("Can't get file");

    // Create directory for config, OS based configuration file, to know more check directories crates
    let path = std::path::Path::new(proj_dirs.config_dir())
        .parent()
        .unwrap();

    if path.exists() {
        return;
    }

    // Then create the actual config file
    std::fs::create_dir_all(path).unwrap();

    File::create(proj_dirs.config_dir().with_file_name("config.toml"))
        .expect("Can't create folder");
}

#[tauri::command]
async fn get_movie_list() -> Result<Vec<Movie>, &'static str> {
    if get_selected_dir().is_empty() {
        return Err("Please set your movie directory");
    }

    let file_list_result = fs::read_dir(get_selected_dir()).unwrap();
    let file_names = file_list_result.filter_map(Result::ok);

    let mut movie_list: Vec<Movie> = vec![];
    let mut base_url = String::from("https://www.omdbapi.com/?apikey=eebff1e2&t=pulp");
    base_url.push_str("pulp fiction");

    let movie_info_url = "https://www.omdbapi.com/?apikey=eebff1e2&t=pulp+fiction".to_string();
    let a = reqwest::get(movie_info_url)
        .await
        .unwrap()
        .json::<serde_json::Value>()
        .await
        .unwrap();

    for movie in file_names {
        movie_list.push(Movie {
            poster: a["Poster"].as_str().unwrap().to_string(),
            year: 2014,
            path: String::from(movie.path().to_str().unwrap()),
            name: String::from(movie.file_name().to_str().unwrap()),
            category: "Supa action".into(),
        })
    }

    cache_movies(&mut movie_list);

    Ok(movie_list)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    initialize_config();

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            greet,
            get_selected_dir,
            set_selected_dir,
            get_movie_list,
            config::get_user_config
        ])
        .setup(|app| {
            #[cfg(debug_assertions)]
            app.get_window("main").unwrap().open_devtools(); // `main` is the first window from tauri.conf.json without an explicit label
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    Ok(())
}
