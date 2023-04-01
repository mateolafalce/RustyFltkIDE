use std::{
    fs::{
        File,
        OpenOptions
    },
    io::{
        Error,
        Write,
        Read
    }
};

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
    let root: String = root + "\\-";
    contents.push_str(&root);
    file.write_all(contents.as_bytes()).unwrap();
    Ok(())
}
