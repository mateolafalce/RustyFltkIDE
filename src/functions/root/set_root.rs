use std::io::Write;

pub fn set_root(root: String) -> Result<(), std::io::Error> {
    // Create a file named "src/constants/root.rs"
    let mut file: std::fs::File = std::fs::File::create("src/constants/root.rs")?;
    // Write the contents of the 'root' string as bytes to the file
    file.write_all(root.as_bytes())?;
    Ok(())
}
