use fltk::{
    prelude::*,
    valuator::{
        HorNiceSlider,
        NiceSlider
    },
    tree::Tree,
    text::{
        TextEditor,
        TextDisplay
    },
    input::Input,
    app::{
        App,
        screen_size
    },
    draw::set_cursor,
    enums::{
        Cursor,
        Event,
    },
};

pub fn horizontal_slider(
    folders: Tree,
    text_editor: TextEditor,
    terminal_output: TextDisplay,
    terminal_input: Input,
    app: App,
    right_slider: NiceSlider
) -> HorNiceSlider {
    let mut folders: Tree = folders.clone();
    let mut text_editor: TextEditor = text_editor.clone();
    let mut terminal_output: TextDisplay = terminal_output.clone();
    let mut terminal_input: Input = terminal_input.clone();
    let app: App = app.clone();
    let mut slider: HorNiceSlider = HorNiceSlider::new(0, 10, 1000, 10, None);
    slider.set_minimum(0.);
    slider.set_maximum(100.);
    slider.set_step(1., 1);
    slider.set_value(20.);
    let width_size: f64 = screen_size().0;
    slider.set_callback(move |slider_value| {
        let left_width: f64 = width_size * (slider_value.value() / 100.0);
        let right_width: f64 = width_size - left_width;
        folders.resize(folders.x(), folders.y(), left_width as i32, folders.height());
        text_editor.resize(left_width as i32, text_editor.y(), right_width as i32 - right_slider.width() as i32, text_editor.height());
        terminal_output.resize(left_width as i32, terminal_output.y(), right_width as i32, terminal_output.height());
        terminal_input.resize(left_width as i32, terminal_input.y(), right_width as i32, terminal_input.height());
        app.redraw();
    });
    slider.handle(move |_, event| {
        match event {
            Event::Push => {
                set_cursor(Cursor::Move);
                true
            },
            Event::NoEvent => {
                set_cursor(Cursor::Arrow);
                true
            },
            Event::Leave => {
                set_cursor(Cursor::Arrow);
                true
            },
            _ => false,
        }
    });
    slider
}
