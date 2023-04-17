use std::{
    fs::File,
    io::{
        Error,
        Write
    }
};

pub fn set_root(root: String) -> Result<(), Error> {
    let mut file: File = File::create("src/constants/root.rs")?;
    file.write_all(root.as_bytes())?;
    Ok(())
}
