#[allow(dead_code)]
pub fn find_path(
    paths: &Vec<String>,
    target_path: &str
) -> bool { // This function checks if a given path exists in a vector of paths.
    for path in paths.iter() { // Iterate through each path in the vector and compare it to the target path.
        if path == target_path {
            return true;
        }
    }
    false
}
