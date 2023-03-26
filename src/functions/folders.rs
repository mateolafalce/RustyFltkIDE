use fltk::{
    prelude::*,
    tree::{
        Tree,
    },
    enums::{
        Event,
        FrameType,
        Cursor,
    },
    button::Button,
    dialog::{
        NativeFileChooser,
        NativeFileChooserType
    },
    draw::set_cursor,
};

pub fn folders() -> Tree {
    let mut folders: Tree = Tree::new(0, 20, 150, 558, None);
    folders.set_root_label("Project folders");
    folders.add("Item 1");
    folders.add("Item 2");
    folders.add("Item 3");
    folders.add("Item 3/Subitem 1");
    folders.add("Item 3/Subitem 2");
    folders.add("Item 3/Subitem 3");
    let mut add_project_folder: Button = Button::new(2, 579, 146, 20, "🗃️ Add Project");
    add_project_folder.set_frame(FrameType::UpBox);
    add_project_folder.set_callback(move |_| {
        let mut dialog: NativeFileChooser = NativeFileChooser::new(NativeFileChooserType::BrowseFile);
        dialog.show();
        println!("{:?}", dialog.filename());
    });
    add_project_folder.handle(move |_, event| {
        match event {
            Event::Enter => {
                set_cursor(Cursor::Hand);
                true
            },
            Event::Leave => {
                set_cursor(Cursor::Arrow);
                true
            },
            _ => false,
        }
    });
    folders
}
