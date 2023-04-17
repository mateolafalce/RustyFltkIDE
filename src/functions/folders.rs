use fltk::{
    prelude::*,
    tree::{
        Tree,
        TreeSelect,
        TreeConnectorStyle,
        TreeReason,
    },
    enums::{
        Color,
    },
};
#[path="./root/get_folders_roots.rs"]
mod get_folders_roots;
#[path="./root/get_all_paths_in_directory.rs"]
mod get_all_paths_in_directory;

use std::{
    path::Path,
};

pub fn folders() -> (Tree, Vec<String>) {
    let mut folders: Tree = Tree::new(0, 20, 200, 580, None);
    let mut prefix: Vec<String> = vec![];
    let mut close_tree: Vec<String> = vec![];
    let (raw_path, is_the_repository_clear): (Vec<String>, bool) = get_folders_roots::get_folders_roots();
    if raw_path.len() > 1 {
        for i in 0..raw_path.len() - 1 {
            let mut split_path: Vec<&str> = raw_path[i].as_str().split('\\').collect();
            close_tree.push(split_path.last().unwrap().to_string());
            split_path.pop();
            prefix.push(split_path.join("/"));
            let paths: Vec<String> = get_all_paths_in_directory::get_all_paths_in_directory(
                &Path::new(&raw_path[i]),
                prefix[i].clone(),
                is_the_repository_clear
            );
            for path in &paths {
                folders.add(&path);
            }
        }
        for i in 0..raw_path.len() - 1 {
            let _ = folders.close(&close_tree[i], true);
        }
    } else {
        for i in 0..raw_path.len() {
            let mut split_path: Vec<&str> = raw_path[i].as_str().split('\\').collect();
            close_tree.push(split_path.last().unwrap().to_string());
            split_path.pop();
            prefix.push(split_path.join("/"));
            let paths: Vec<String> = get_all_paths_in_directory::get_all_paths_in_directory(
                &Path::new(&raw_path[i]),
                prefix[i].clone(),
                is_the_repository_clear
            );
            for path in &paths {
                folders.add(&path);
            }
        }
        for i in 0..raw_path.len() {
            let _ = folders.close(&close_tree[i], true);
        }
    }
    folders.set_callback_reason(TreeReason::Closed);
    folders.set_show_root(false);
    folders.set_select_mode(TreeSelect::Multi);
    folders.set_connector_style(TreeConnectorStyle::Solid);
    folders.set_connector_color(Color::from_rgb(100,100,0));
    (folders, prefix)
}
