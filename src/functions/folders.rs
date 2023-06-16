use fltk::prelude::*;
#[path="./root/get_folders_roots.rs"]
mod get_folders_roots;
#[path="./root/get_all_paths_in_directory.rs"]
mod get_all_paths_in_directory;
#[path="./folders_functions/render_tree_when_exist.rs"]
mod render_tree_when_exist;
#[path="./folders_functions/render_tree_when_doesnt_exist.rs"]
mod render_tree_when_doesnt_exist;

pub fn folders() -> (fltk::tree::Tree, Vec<String>) {
    let mut folders: fltk::tree::Tree = fltk::tree::Tree::new(0, 20, 200, 580, None);
    let prefix: Vec<String> = vec![];
    let close_tree: Vec<String> = vec![];
    let (raw_path, is_the_repository_clear): (Vec<String>, bool) = get_folders_roots::get_folders_roots();
    // Check if there are multiple root directories
    if raw_path.len() > 1 {
        render_tree_when_exist::render_tree_when_exist(folders.clone(), close_tree, raw_path, is_the_repository_clear, prefix.clone());
    } else {
        // If there is only one root directory
        render_tree_when_doesnt_exist::render_tree_when_doesnt_exist(folders.clone(), close_tree, raw_path, is_the_repository_clear, prefix.clone());
    }
    // Set tree properties
    folders.set_callback_reason(fltk::tree::TreeReason::Closed);
    folders.set_show_root(false);
    folders.set_select_mode(fltk::tree::TreeSelect::Multi);
    folders.set_connector_style(fltk::tree::TreeConnectorStyle::Solid);
    folders.set_connector_color(fltk::enums::Color::from_rgb(100,100,0));
    (folders, prefix)
}
