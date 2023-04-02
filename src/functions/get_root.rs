use std::{
    fs::File,
    io::{
        Error,
        Read
    }
};

pub fn root() -> Result<String, Error> {
    let mut file: File = File::open("src/constants/root.rs").expect("Error");
    let mut contents: String = String::new();
    match file.read_to_string(&mut contents) {
        Ok(_) => {
            contents.push_str(r"\🎯");
            Ok(contents)
        },
        Err(_) => {
            Ok("root".to_string())
        }
    }
}
