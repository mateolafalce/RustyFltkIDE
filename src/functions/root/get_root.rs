use std::io::{Write, Read};

pub fn get_root() -> String {
    // Open the file in read mode
    let mut file: std::fs::File = std::fs::File::open("src/constants/root.rs").unwrap();
    let mut path: String = String::new();
    // Read the contents of the file into the `path` variable
    file.read_to_string(&mut path).unwrap();
    // Check if the path exists
    if std::path::Path::new(&path).exists() {
        // Append a target emoji to the path
        path.push_str(r"\🎯");
        return path;
    } else {
        // Create a new file for writing the root directory path
        let mut path: String = String::new();
        let mut root_dir: std::fs::File = std::fs::OpenOptions::new()
            .write(true)
            .create(false)
            .truncate(true)
            .open("src/constants/root.rs")
            .unwrap();
        // Write the current directory path to the file
        root_dir
            .write_all(std::env::current_dir().unwrap().display().to_string().as_bytes())
            .unwrap();
        // Read the contents of the file into the `path` variable
        std::fs::read_to_string(&mut path).unwrap();
        // Append a target emoji to the path
        path.push_str(r"\🎯");
        return path;
    }
}
