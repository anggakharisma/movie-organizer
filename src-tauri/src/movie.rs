use std::fs;

use serde::{Deserialize, Serialize};

use crate::config::get_selected_movie_dir;

#[derive(Serialize, Debug, Deserialize)]
pub struct Movie {
    pub poster: String,
    pub year: usize,
    pub name: String,
    pub path: String,
    pub category: Option<String>,
}

impl Movie {
    fn new(
        poster: String,
        year: usize,
        name: String,
        path: String,
        category: Option<String>,
    ) -> Self {
        Movie {
            poster,
            year,
            name,
            path,
            category,
        }
    }

    fn check_cache(&self) -> Option<&Self> {
        None
    }
    fn write_to_cache() {}
    fn fetch_metadata() {}
    fn clean_string(&self) -> Option<&Self> {
        None
    }
}

#[tauri::command]
pub fn get_movie_list() -> Result<Vec<Movie>, &'static str> {
    if get_selected_movie_dir().is_empty() {
        return Err("Please set your movie directory");
    }

    let file_list_result = fs::read_dir(get_selected_movie_dir()).unwrap();

    let filtered_file_list: Vec<_> = file_list_result
        .map(|y| y.unwrap())
        .filter(|z| !z.file_name().to_str().unwrap().contains(".DS_Store"))
        .collect::<Vec<_>>();

    let mut movie_list: Vec<Movie> = vec![];
    for p in filtered_file_list {
        let m = Movie::new(
            String::from(""),
            2014,
            p.file_name().into_string().unwrap(),
            String::from(""),
            Default::default(),
        );
        movie_list.push(m);
    }

    //let a = reqwest::get(base_url)
    //    .await
    //    .unwrap()
    //    .json::<serde_json::Value>()
    //    .await
    //    .unwrap();

    // for movie in file_names {
    // let mut base_url = String::from("https://www.omdbapi.com/?apikey=<API_KEY>&t=");
    // base_url.push_str(movie.file_name().to_str().unwrap());
    //movie_list.push(Movie {
    //    poster: a["Poster"].as_str().unwrap().to_string(),
    //    year: 2014,
    //    path: String::from(movie.path().to_str().unwrap()),
    //    name: String::from(movie.file_name().to_str().unwrap()),
    //    category: "Supa action".into(),
    //})
    // }

    Ok(movie_list)
}

pub fn cache_movie_list(movie: &Movie) {}
pub fn get_movies_cache() {}
