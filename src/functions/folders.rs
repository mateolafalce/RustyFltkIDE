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
    root,
    get_all_paths_in_directory
};
use std::{
    path::Path,
};

pub fn folders() -> (Tree, String) {
    let mut folders: Tree = Tree::new(0, 20, 200, 560, None);
    let mut raw_path: String = root().unwrap();
    raw_path.pop();
    raw_path.pop();
    let root = Path::new(&raw_path);
    let mut split_path: Vec<&str> = raw_path.as_str().split('\\').collect();
    split_path.pop();
    let prefix: String = split_path.join("/");
    let paths: Vec<String> = get_all_paths_in_directory(&root, prefix.clone());
    for path in &paths {
        folders.add(&path);
    }
    let _ = folders.close("RustyFlktIDE", true);
    let _ = folders.close("RustyFlktIDE/.git", true);
    let _ = folders.close("RustyFlktIDE/src", true);
    let _ = folders.close("RustyFlktIDE/target", true);
    folders.set_callback_reason(TreeReason::Closed);
    folders.set_show_root(false);
    folders.set_select_mode(TreeSelect::Multi);
    folders.set_connector_style(TreeConnectorStyle::Solid);
    folders.set_connector_color(Color::from_rgb(100,100,0));
    (folders, prefix)
}
