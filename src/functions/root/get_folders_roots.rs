use std::io::Read;
#[path="../event/error.rs"]
mod error;
#[path="../event/center.rs"]
mod center;

#[allow(dead_code)]
pub fn get_folders_roots() -> (
    Vec<String>,
    bool
) {
    let mut file: std::fs::File = std::fs::File::open("src/constants/folders_roots.rs").unwrap();
    let mut contents: String = String::new();
    let mut is_the_repository_clear: bool = true;
    match file.read_to_string(&mut contents) {
        Ok(_) => {
            if contents.len() > 0 {
                is_the_repository_clear = false;
            }
        },
        Err(e) => {
            error::error(&e.to_string());
        }
    }
    let vector: Vec<String> = contents.as_str().split("\\-").map(|s| s.to_string()).collect();
    (vector, is_the_repository_clear)
}
