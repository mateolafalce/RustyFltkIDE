use std::io::Write;

pub fn set_root(root: String) -> Result<(), std::io::Error> {
    let mut file: std::fs::File = std::fs::File::create("src/constants/root.rs")?;
    file.write_all(root.as_bytes())?;
    Ok(())
}
