use fltk::prelude::*;
#[derive(Copy, Clone)]
pub enum Message {
    Save,
}

pub fn app_events(
    mut menu: fltk::menu::SysMenuBar,
    s: fltk::app::Sender<Message>,
    r: fltk::app::Receiver<Message>
) -> bool {
    menu.add_emit(
        "&File/Save\t",
        fltk::enums::Shortcut::Ctrl | 's',
        fltk::menu::MenuFlag::Normal,
        s,
        Message::Save,
    );
    use Message::*;
    if let Some(msg) = r.recv() {
        match msg {
            Save => println!("Ctrl + S was pressed!"),
        }
    }
    true
}
