use std::{
    fs::{
        File,
        OpenOptions
    },
    io::{
        Error,
        Write,
        Read
    },
};
use fltk::dialog::alert;
use crate::functions::center;

pub fn set_folders_roots(
    root: String
) -> Result<(), Error> {
    let mut file: File = OpenOptions::new()
        .read(true)
        .write(true)
        .open("src/constants/folders_roots.rs")
        .unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    if contents.len() == 0 {
        contents.push_str(&(root + "\\-"));
        file.write_all(contents.as_bytes()).unwrap();
    } else {
        let paths: Vec<String> = contents.as_str().split("\\-").map(|s| s.to_string()).collect();
        if find_path(&paths, &root) {
            alert(
                center().0 - 100,
                center().1 - 100,
                &format!("Error: The path is already in the directory\n")
            );
        } else {
            contents.push_str(&(root + "\\-"));
            file.write_all(
                contents.as_bytes()
            ).unwrap();
        }
    }
    Ok(())
}

fn find_path(paths: &Vec<String>, target_path: &str) -> bool {
    for path in paths.iter() {
        if path == target_path {
            return true;
        }
    }
    false
}
