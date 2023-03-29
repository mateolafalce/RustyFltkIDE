use fltk::{
    prelude::*,
    button::Button,
    enums::{
        Cursor,
        Event,
        FrameType
    },
    dialog::{
        NativeFileChooser,
        NativeFileChooserType
    },
    draw::set_cursor,
};

pub fn btn_add_folder() -> Button {
    let mut add_project_folder: Button = Button::new(0, 580, 200, 20, "🗃️ Add Project");
    add_project_folder.set_frame(FrameType::UpBox);
    add_project_folder.set_callback(move |_| {
        let mut dialog: NativeFileChooser = NativeFileChooser::new(NativeFileChooserType::BrowseFile);
        dialog.show();
        //println!("{:?}", dialog.filename());
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
    add_project_folder
}
