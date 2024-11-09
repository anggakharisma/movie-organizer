use directories::BaseDirs;

use std::{fs::File, io::Read, path::PathBuf};

pub fn get_cache_movie_dir() -> PathBuf {
    let cache_dir = BaseDirs::new().unwrap().cache_dir().join("seirei");
    cache_dir
}

pub fn create_cache_movie_file() {
    let cache_dir = get_cache_movie_dir();
    std::fs::create_dir_all(&cache_dir).unwrap();

    let file_path = cache_dir.join("movie.json");

    // Check if file exists
    if let Ok(mut file) = File::open(&file_path) {
        // File exists, check if it's empty
        let mut contents = String::new();
        if file.read_to_string(&mut contents).unwrap() == 0 || contents.trim().is_empty() {
            // File is empty, write empty JSON array
            std::fs::write(&file_path, "[]").unwrap();
        }
    } else {
        File::create(&file_path)
            .and_then(|_| std::fs::write(&file_path, "[]"))
            .unwrap();
    }
}
