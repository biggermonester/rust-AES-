use std::path::PathBuf;
use walkdir::WalkDir;

pub fn list_files(p: PathBuf) -> Vec<PathBuf> {

    let mut file_path: Vec<PathBuf> = Vec::new();
    for entry in WalkDir::new(p).into_iter().filter_map(|e| e.ok()) {
        file_path.push(entry.path().to_path_buf());
    }
    file_path
}
