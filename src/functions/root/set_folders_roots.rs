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
#[path="../event/center.rs"]
mod center;
#[path="./find_path.rs"]
mod find_path;

#[allow(dead_code)]
pub fn set_folders_roots(
    root: String
) -> Result<(), Error> {
    // Open the "folders_roots.rs" file with read and write access.
    let mut file: File = OpenOptions::new().read(true).write(true).open("src/constants/folders_roots.rs").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();// Read the current contents of the file into a string.
    if contents.len() == 0 { // If the file is empty, write the new root path to it.
        file.write_all(&(root + "\\-").as_bytes()).unwrap();
    } else {
        // Otherwise, split the file contents into individual paths and check if the new root path is already in the file.
        let paths: Vec<String> = contents
            .as_str()
            .split("\\-")
            .map(|s| s.to_string())
            .collect();
        if find_path::find_path(&paths, &root) {
            alert( // If the path is already in the file, show an error message.
                center::center().0 - 100,
                center::center().1 - 100,
                &format!("Error: The path is already in the directory\n")
            );
        } else {
            file.write_all(&(root + "\\-").as_bytes()).unwrap(); // Otherwise, write the new root path to the file.
        }
    }
    Ok(())
}
