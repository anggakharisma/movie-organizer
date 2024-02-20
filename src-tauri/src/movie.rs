use std::{error::Error, fs};

use regex::Regex;
use serde::{Deserialize, Serialize};

use crate::{cache::get_cache_movie_dir, config::get_selected_movie_dir};

#[derive(Serialize, Debug, Deserialize, Clone)]
pub struct Movie {
    pub name: String,
    pub poster: Option<String>,
    pub year: Option<usize>,
    pub path: Option<String>,
    pub category: Option<String>,
}

impl Movie {
    fn new(
        name: String,
        poster: Option<String>,
        year: Option<usize>,
        path: Option<String>,
        category: Option<String>,
    ) -> Self {
        Movie {
            name,
            poster,
            year,
            path,
            category,
        }
    }

    fn check_cache(t: &Movie) -> Option<Movie> {
        let cache = get_cache_movie_dir().join("movie.json");
        let cache_file = fs::read_to_string(&cache).unwrap();

        let movie_cache: serde_json::Value = serde_json::from_str(&cache_file).unwrap();
        let movie_json: Vec<Movie> = serde_json::from_value(movie_cache).unwrap();
        if let Some(m) = movie_json.iter().find(|f| t.name == f.name) {
            return Some(m.clone());
        }
        None
    }

    fn write_to_cache(&self) {
        let cache = get_cache_movie_dir().join("movie.json");

        let data = fs::read_to_string(&cache).unwrap();
        let mut movies: Vec<Movie> = vec![];

        if fs::metadata(&cache).unwrap().len() != 0 {
            movies = serde_json::from_str(&data).unwrap();
        }

        movies.push(self.clone());
        let json_cache = serde_json::to_string(&movies).unwrap();
        fs::write(cache, &json_cache).unwrap();
    }

    // maybe be make this a module?
    fn clean_name(&mut self) {
        let bracket_regex = Regex::new(
            r"\(([^()]+)\)|\[([^()]+)\]|(?i)(1080p|webrip|bluray|x264|x265|10bit|rarbg|ac3|6ch|-etrg|hevc|psa|\b(19|20)\d{2}\b)",
        )
        .unwrap();
        self.name = self.name.replace(".", " ");
        self.name = String::from(bracket_regex.replace_all(&self.name[..], ""));
    }

    async fn fetch_metdata(&mut self) -> Result<(), Box<dyn Error>> {
        let base_url = format!(
            "https://www.omdbapi.com/?apikey={}&t={}",
            "eecc",
            &self.name[..]
        )
        .to_string();

        println!("{}", base_url);

        let a = reqwest::get(base_url)
            .await?
            .json::<serde_json::Value>()
            .await?;

        match a.get("Poster") {
            Some(_) => {
                self.poster = Some(String::from(a["Poster"].as_str().unwrap()));
            }
            None => {
                self.poster = Some("placeholder image".to_string());
            }
        };
        Ok(())
    }
}

#[tauri::command]
pub async fn get_movie_list() -> Result<Vec<Movie>, &'static str> {
    if get_selected_movie_dir().is_empty() {
        return Err("Please set your movie directory");
    }

    let file_list_result = fs::read_dir(get_selected_movie_dir()).unwrap();

    let filtered_file_list: Vec<String> = file_list_result
        .map(|y| y.unwrap())
        .filter(|z| !z.file_name().to_str().unwrap().contains(".DS_Store"))
        .map(|y| String::from(y.path().file_name().unwrap().to_str().unwrap()))
        .take(20)
        .collect::<Vec<String>>();

    let mut movie_list: Vec<Movie> = vec![];

    for p in filtered_file_list {
        let mut m = Movie::new(p, None, None, None, None);
        m.clean_name();

        if let Some(x) = Movie::check_cache(&m) {
            movie_list.push(x);
        } else {
            m.fetch_metdata().await.ok();
            m.write_to_cache();
            movie_list.push(m);
        }
    }

    Ok(movie_list)
}
