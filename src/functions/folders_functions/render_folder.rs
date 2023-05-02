use fltk::{
    app::App,
    tree::Tree,
    text::{
        TextBuffer,
    }
};
use crate::functions::{
    render_file::render_file,
};
#[path="../root/get_all_paths_in_directory.rs"]
mod get_all_paths_in_directory;
#[path="../root/set_folders_roots.rs"]
mod set_folders_roots;
#[path="../root/get_folders_roots.rs"]
mod get_folders_roots;
use std::path::Path;

pub fn render_folder(
    app: App,
    mut folders: Tree,
    text_buffer: TextBuffer,
) {
    let mut prefix: Vec<String> = vec![];
    let mut close_tree: Vec<String> = vec![];
    let (raw_path, is_the_repository_clear): (Vec<String>, bool) = get_folders_roots::get_folders_roots();
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
            if !path.contains("target") {
                folders.add(&path);
            }
        }
    }
    for i in 0..raw_path.len() - 1 {
        let _ = folders.close(&close_tree[i], true);
    }
    if prefix.len() == 1 {
        for i in 0..prefix.len() {
            render_file(
                folders.clone(),
                text_buffer.clone(),
                prefix[i].clone()
            );
        }
    } else if prefix.len() > 1 {
        for i in 0..prefix.len() - 1 {
            render_file(
                folders.clone(),
                text_buffer.clone(),
                prefix[i].clone()
            );
        }
    }
    app.redraw();
}
