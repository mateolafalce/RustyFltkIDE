use std::{
    path::Path,
    fs
};

pub fn get_all_paths_in_directory(
    path: &Path,
    prefix: String
) -> Vec<String> {
    let mut paths = Vec::new();
    for entry in fs::read_dir(path).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        let mut path_str = path.to_str().unwrap().to_string().replace("\\", "/");
        if path.is_dir() {
            path_str.push('/');
            let sub_paths = get_all_paths_in_directory(&path, prefix.clone());
            paths.extend(sub_paths);
        }
        paths.push(path_str);
    }
    for i in 0..paths.len() {
        if paths[i].starts_with(&prefix) {
            paths[i] = paths[i].replace(&prefix, "");
        }
    }
    paths
}
