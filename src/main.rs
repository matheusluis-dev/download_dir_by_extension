use directories::UserDirs;
use std::fs;
use std::path::PathBuf;

fn main() {
    let user_dirs = match UserDirs::new() {
        Some(user_dirs) => Ok(user_dirs),
        None => Err("Failed to get user directories."),
    }
    .unwrap();

    let download_dir = match user_dirs.download_dir() {
        Some(download_dir) => Ok(download_dir),
        None => Err("Failed to get download directory."),
    }
    .unwrap();

    let files: Vec<PathBuf> = fs::read_dir(download_dir)
        .unwrap()
        .filter_map(Result::ok)
        .filter(|entry| entry.path().is_file())
        .map(|entry| entry.path())
        .collect();

    for file in files {
        let extension = file.extension().unwrap().to_string_lossy().to_string();
        let file_name = file.file_name().unwrap().to_string_lossy().to_string();
        let old_path = file.parent().unwrap().to_string_lossy().to_string();

        let new_path = format!("{}\\{}", &old_path, &extension);
        let new_file = format!("{}\\{}", &new_path, &file_name);

        fs::create_dir(&new_path).unwrap_or_else(|err| {
            match err.kind() {
                std::io::ErrorKind::AlreadyExists => { /* ignore */ }
                _ => {
                    eprintln!("Failed to create directory: {}", err);
                }
            }
        });

        match fs::rename(&file, &new_file) {
            Ok(_) => println!("File '{}' moved successfully.", file_name),
            Err(err) => {
                eprintln!("Failed to move the file: {}", err);
            }
        }
    }
}
