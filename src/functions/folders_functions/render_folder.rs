#[path="../root/get_all_paths_in_directory.rs"]
mod get_all_paths_in_directory;
#[path="../root/set_folders_roots.rs"]
mod set_folders_roots;
#[path="../root/get_folders_roots.rs"]
mod get_folders_roots;
#[path="../render_file.rs"]
mod render_file;

pub fn render_folder(
    app: fltk::app::App,
    mut folders: fltk::tree::Tree,
    text_buffer: fltk::text::TextBuffer,
) {
    let mut prefix: Vec<String> = vec![];
    let mut close_tree: Vec<String> = vec![];
    // Get root paths and check if the repository is clear
    let (raw_path, is_the_repository_clear): (Vec<String>, bool) = get_folders_roots::get_folders_roots();
    // Iterate through root paths
    for i in 0..raw_path.len() - 1 {
        // Split path and prepare for closing tree
        let mut split_path: Vec<&str> = raw_path[i].as_str().split('\\').collect();
        close_tree.push(split_path.last().unwrap().to_string());
        split_path.pop();
        prefix.push(split_path.join("/"));
        // Get all paths in the directory
        let paths: Vec<String> = get_all_paths_in_directory::get_all_paths_in_directory(
            &std::path::Path::new(&raw_path[i]),
            prefix[i].clone(),
            is_the_repository_clear
        );
        // Add paths to the tree
        for path in &paths {
            if !path.contains("target") {
                folders.add(&path);
            }
        }
    }
    // Close tree nodes
    for i in 0..raw_path.len() - 1 {
        let _ = folders.close(&close_tree[i], true);
    }
    // Render files based on prefix count
    if prefix.len() == 1 {
        for i in 0..prefix.len() {
            render_file::render_file(
                folders.clone(),
                text_buffer.clone(),
                prefix[i].clone()
            );
        }
    } else if prefix.len() > 1 {
        for i in 0..prefix.len() - 1 {
            render_file::render_file(
                folders.clone(),
                text_buffer.clone(),
                prefix[i].clone()
            );
        }
    }
    // Redraw the application
    app.redraw();
}
