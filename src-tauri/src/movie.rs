use std::{error::Error, fs};

use regex::Regex;
use serde::{Deserialize, Serialize};

use crate::{cache::get_cache_movie_dir, config::get_selected_movie_dir};

#[derive(Serialize, Debug, Deserialize)]
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

    fn check_cache(&self) -> Option<Movie> {
        None
    }

    fn write_to_cache(&self) {
        let path = get_cache_movie_dir().join("movie.json");
    }

    // maybe be make this a module
    fn clean_name(&mut self) {
        let bracket_regex = Regex::new(
            r"\(([^()]+)\)|\[([^()]+)\]|(?i)(1080p|webrip|bluray|x264|x265|10bit|rarbg|ac3|6ch|-etrg|hevc|psa|\b(19|20)\d{2}\b)",
        )
        .unwrap();
        self.name = self.name.replace(".", " ");
        self.name = String::from(bracket_regex.replace_all(&self.name[..], ""));
    }

    async fn fetch_metdata(&mut self) -> Result<(), Box<dyn Error>> {
        let mut base_url = String::from("https://www.omdbapi.com/?apikey=<api_key>&t=");
        base_url.push_str(&self.name[..]);

        println!("{}", base_url);

        let a = reqwest::get(base_url)
            .await?
            .json::<serde_json::Value>()
            .await?;

        println!("REQUEST: {:?}", a.get("Poster"));

        match a.get("Poster") {
            Some(_) => {
                self.poster = Some(String::from(a["Poster"].as_str().unwrap()));
            }
            None => {}
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
        .take(5)
        .collect::<Vec<String>>();

    let mut movie_list: Vec<Movie> = vec![];

    for p in filtered_file_list {
        let mut m = Movie::new(p, None, None, None, None);
        m.clean_name();

        if let Some(x) = m.check_cache() {
            movie_list.push(x);
        } else {
            m.fetch_metdata().await.ok();
            m.write_to_cache();
            movie_list.push(m);
        }
    }

    Ok(movie_list)
}
