use directories::BaseDirs;

use std::{fs::File, io::Read, path::PathBuf};

pub fn get_cache_dir() -> PathBuf {
    let cache_dir = BaseDirs::new().unwrap().cache_dir().join("seirei");
    cache_dir
}

pub fn create_cache_file(file_name: String) {
    let cache_dir = get_cache_dir();
    std::fs::create_dir_all(&cache_dir).unwrap();

    let file_path = cache_dir.join(format!("{}.json", file_name));
    match File::open(&file_path) {
        Ok(mut file) => {
            let mut contents = String::new();
            if file.read_to_string(&mut contents).unwrap() == 0 || contents.trim().is_empty() {
                // File is empty, write empty JSON array
                std::fs::write(&file_path, "[]").unwrap();
            }
        }
        Err(_) => {
            File::create(&file_path)
                .and_then(|_| std::fs::write(&file_path, "[]"))
                .unwrap();
        }
    }
}
