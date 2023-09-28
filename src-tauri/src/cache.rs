use crate::movie::Movie;

use std::fs::File;

pub fn cache_movies(movies: &mut Vec<Movie>) {
    let file = File::create("output.json").unwrap();
    serde_json::to_writer(file, &movies).unwrap();
}

pub fn get_cache_movies(movies: &mut Vec<Movie>) -> Vec<Movie> {
    let movie_list = vec![];
    movie_list
}

pub fn get_cache_dir() {}
