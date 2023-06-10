use fltk::prelude::*;

#[path="./root/get_folders_roots.rs"]
mod get_folders_roots;
#[path="./root/get_all_paths_in_directory.rs"]
mod get_all_paths_in_directory;
#[path="./folders_functions/redner_tree_when_exist.rs"]
mod redner_tree_when_exist;

pub fn folders() -> (fltk::tree::Tree, Vec<String>) {
    let mut folders: fltk::tree::Tree = fltk::tree::Tree::new(0, 20, 200, 580, None);
    let mut prefix: Vec<String> = vec![];
    let mut close_tree: Vec<String> = vec![];
    let (raw_path, is_the_repository_clear): (Vec<String>, bool) = get_folders_roots::get_folders_roots();
    // Check if there are multiple root directories
    if raw_path.len() > 1 {
        redner_tree_when_exist::redner_tree_when_exist(folders.clone(), close_tree, raw_path, is_the_repository_clear, prefix.clone());
    } else { // If there is only one root directory
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
    }
    // Set tree properties
    folders.set_callback_reason(fltk::tree::TreeReason::Closed);
    folders.set_show_root(false);
    folders.set_select_mode(fltk::tree::TreeSelect::Multi);
    folders.set_connector_style(fltk::tree::TreeConnectorStyle::Solid);
    folders.set_connector_color(fltk::enums::Color::from_rgb(100,100,0));
    (folders, prefix)
}
