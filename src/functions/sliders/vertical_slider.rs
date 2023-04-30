use fltk::{
    prelude::*,
    app::{
        App,
        screen_size
    },
    text::{
        TextEditor,
        TextDisplay,
    },
    valuator::NiceSlider,
    input::Input,
    enums::{
        Cursor,
        Event,
    },
    draw::set_cursor,
};

pub fn vertical_slider(
    mut text_editor: TextEditor,
    mut terminal_output: TextDisplay,
    terminal_input: Input,
    app: App
) -> NiceSlider {
    let mut vertical_slider: NiceSlider = NiceSlider::new(990, 20, 10, 580, None);
    vertical_slider.set_minimum(0.);
    vertical_slider.set_maximum(100.);
    vertical_slider.set_step(1., 1);
    vertical_slider.set_value(66.);
    let height: f64 = screen_size().1;
    let vertical_slider_: NiceSlider = vertical_slider.clone();
    vertical_slider.set_callback(move |slider_value| {
        let mut top_height: f64 = (height * (slider_value.value() / 100.0)) - vertical_slider_.width() as f64;
        let mut y_for_terminal_output: i32 = top_height as i32 + terminal_input.height();
        let mut bottom_height: f64 = height - top_height - terminal_input.height() as f64;
        if top_height <= height * (10.0 / 100.0) {
            top_height = height * (10.0 / 100.0);
            y_for_terminal_output = top_height as i32 - terminal_input.height();
            bottom_height = height - top_height - (terminal_input.height() as f64 * 1.5)
        }
        text_editor.resize(text_editor.x(),text_editor.y(),text_editor.width(),top_height as i32);
        terminal_output.resize(
            terminal_output.x(),
            y_for_terminal_output,
            text_editor.width(),
            bottom_height as i32
        );
        app.redraw();
    });
    vertical_slider.handle(move |_, event| {
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
    vertical_slider
}
