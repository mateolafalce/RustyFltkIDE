use std::io::Read;

// Importing the error module from the specified path
#[path="../event/error.rs"]
mod error;

// Importing the center module from the specified path
#[path="../event/center.rs"]
mod center;

#[allow(dead_code)]
pub fn get_folders_roots() -> (Vec<String>, bool) {
    let mut file: std::fs::File = std::fs::File::open("src/constants/folders_roots.rs").unwrap();
    let mut contents: String = String::new();
    let mut is_the_repository_clear: bool = true;
    // Read the contents of the file into the 'contents' variable
    match file.read_to_string(&mut contents) {
        Ok(_) => {
            if contents.len() > 0 {
                is_the_repository_clear = false;
            }
        },
        Err(e) => {
            error::error(&e.to_string()); // Call the error function from the error module
        }
    }
    // Split the contents into individual strings using the delimiter '\-'
    let vector: Vec<String> = contents.as_str().split("\\-").map(|s| s.to_string()).collect();
    // Return the vector of folder roots and the flag indicating if the repository is clear
    (vector, is_the_repository_clear)
}
