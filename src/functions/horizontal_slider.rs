use fltk::{
    prelude::*,
    valuator::HorNiceSlider,
    tree::Tree,
    text::{
        TextEditor,
        TextDisplay
    },
    input::Input,
    app::App,
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
    app: App
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
    slider.set_callback(move |slider_value| {
        let x1_value: f64 = slider_value.value() * 10.0;
        let x2_value: f64 = 1000.0 - x1_value - 10.0;
        folders.resize(0, 20, x1_value as i32, 560);
        text_editor.resize(x1_value as i32 + 1, 20, x2_value as i32, text_editor.height());
        terminal_output.resize(x1_value as i32 + 1, terminal_output.y(), x2_value as i32, terminal_output.height());
        terminal_input.resize(x1_value as i32 + 1, 570, x2_value as i32, 30);
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
