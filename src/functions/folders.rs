use fltk::{
    prelude::*,
    tree::{
        Tree,
        TreeSelect,
        TreeConnectorStyle,
        TreeReason,
    },
    enums::Color,
};
use crate::functions::{
    get_folders_roots,
    get_all_paths_in_directory
};
use std::{
    path::Path,
};

pub fn folders() -> (Tree, String) {
    let mut folders: Tree = Tree::new(0, 20, 200, 560, None);
    let (raw_path, is_the_repository_clear): (String, bool) = get_folders_roots();
    let root = Path::new(&raw_path);
    let mut split_path: Vec<&str> = raw_path.as_str().split('\\').collect();
    split_path.pop();
    let prefix: String = split_path.join("/");
    let paths: Vec<String> = get_all_paths_in_directory(&root, prefix.clone(), is_the_repository_clear);
    for path in &paths {
        folders.add(&path);
    }
    /*let _ = folders.close("RustyFlktIDE", true);
    let _ = folders.close("RustyFlktIDE/.git", true);
    let _ = folders.close("RustyFlktIDE/src", true);
    let _ = folders.close("RustyFlktIDE/target", true);*/
    folders.set_callback_reason(TreeReason::Closed);
    folders.set_show_root(false);
    folders.set_select_mode(TreeSelect::Multi);
    folders.set_connector_style(TreeConnectorStyle::Solid);
    folders.set_connector_color(Color::from_rgb(100,100,0));
    (folders, prefix)
}
