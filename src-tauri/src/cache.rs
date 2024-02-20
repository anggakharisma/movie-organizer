use directories::BaseDirs;

use std::{fs::File, path::PathBuf};

pub fn get_cache_movie_dir() -> PathBuf {
    let cache_dir = BaseDirs::new().unwrap().cache_dir().join("seirei");
    cache_dir
}

pub fn create_cache_movie_file() {
    let cache_dir = get_cache_movie_dir();
    let path = std::path::Path::new(&cache_dir);

    if path.exists() {
        return;
    }

    std::fs::create_dir_all(&cache_dir).unwrap();
    File::create(cache_dir.join("movie.json")).unwrap();
}
