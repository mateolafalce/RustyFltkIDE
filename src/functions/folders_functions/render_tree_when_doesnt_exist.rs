#[path="../root/get_all_paths_in_directory.rs"]
mod get_all_paths_in_directory;

pub fn render_tree_when_doesnt_exist(
    mut folders: fltk::tree::Tree,
    mut close_tree: Vec<String>,
    raw_path: Vec<String>,
    is_the_repository_clear: bool,
    mut prefix: Vec<String>,
) -> bool {
    for i in 0..raw_path.len() {
        // Split the path and get the last element
        let mut split_path: Vec<&str> = raw_path[i].as_str().split('\\').collect();
        close_tree.push(split_path.last().unwrap().to_string()); // Add the last element to the close_tree vector
        split_path.pop(); // Remove the last element from the split_path vector
        prefix.push(split_path.join("/")); // Join the split_path elements with "/" and add to the prefix vector
        // Get all paths in the directory and iterate over them
        let paths: Vec<String> = get_all_paths_in_directory::get_all_paths_in_directory(
            &std::path::Path::new(&raw_path[i]),
            prefix[i].clone(),
            is_the_repository_clear
        );
        for path in &paths {
            folders.add(&path); // Add the path to the tree
        }
    }
    // Close the tree for each close tree path
    for i in 0..raw_path.len() {
        let _ = folders.close(&close_tree[i], true);
    }
    true
}
