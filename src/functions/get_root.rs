use std::{
    fs::File,
    io::{
        Read,
        Write
    },
    env
};

pub fn root() -> String {
    let mut file: File = File::open("src/constants/root.rs").expect("Error");
    let mut contents: String = String::new();
    match file.read_to_string(&mut contents) {
        Ok(_) => {
            contents.push_str(r"\🎯");
            return contents
        },
        Err(_) => {
            file.write_all(env::current_dir().unwrap().display().to_string().as_bytes()).unwrap();
            contents.push_str(r"\🎯");
            return contents
        }
    }
}
