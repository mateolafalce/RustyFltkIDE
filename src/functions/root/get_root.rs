use std::{
    fs::{
        File,
        OpenOptions
    },
    io::{
        Read,
        Write
    },
    env,
    path::Path,
};

pub fn root() -> String {
    let mut file: File = File::open("src/constants/root.rs").unwrap();
    let mut path: String = String::new();
    file.read_to_string(&mut path).unwrap();
    if Path::new(&path).exists() {
        path.push_str(r"\🎯");
        return path
    } else {
        let mut path: String = String::new();
        let mut root_dir: File = OpenOptions::new()
        .write(true)
        .create(false)
        .truncate(true)
        .open("src/constants/root.rs")
        .unwrap();
        root_dir.write_all(env::current_dir().unwrap().display().to_string().as_bytes()).unwrap();
        root_dir.read_to_string(&mut path).unwrap();
        path.push_str(r"\🎯");
        return path
    }
}
