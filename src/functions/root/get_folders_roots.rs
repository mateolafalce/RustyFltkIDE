use std::{
    fs::File,
    io::Read
};
use fltk::dialog::alert;
#[path="../event/center.rs"]
mod center;

#[allow(dead_code)]
pub fn get_folders_roots() -> (
    Vec<String>,
    bool
) {
    let mut file: File = File::open("src/constants/folders_roots.rs").unwrap();
    let mut contents: String = String::new();
    let mut is_the_repository_clear: bool = true;
    match file.read_to_string(&mut contents) {
        Ok(_) => {
            if contents.len() > 0 {
                is_the_repository_clear = false;
            }
        },
        Err(e) => {
            alert(center::center().0 - 100, center::center().1 - 100, &format!("Error: {}\n", e));
        }
    }
    let vector: Vec<String> = contents.as_str().split("\\-").map(|s| s.to_string()).collect();
    (vector, is_the_repository_clear)
}
