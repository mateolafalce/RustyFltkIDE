use fltk::{
    prelude::*,
    app::App,
    text::{
        TextEditor,
        TextDisplay,
    },
    valuator::NiceSlider,
    enums::{
        Cursor,
        Event,
    },
    draw::set_cursor,
};

pub fn vertical_slider(
    mut text_editor: TextEditor,
    mut terminal_output: TextDisplay,
    app: App
) -> NiceSlider {
    let mut vertical_slider: NiceSlider = NiceSlider::new(990, 20, 10, 580, None);
    vertical_slider.set_minimum(0.);
    vertical_slider.set_maximum(100.);
    vertical_slider.set_step(1., 1);
    vertical_slider.set_value(66.);
    vertical_slider.set_callback(move |slider_value| {
        let mut y1_value: f64 = slider_value.value() * 5.75757575758;
        if y1_value >= 550.0 {
            y1_value = 550.0;
        }
        let y2_value: f64 = 580.0 - y1_value;
        text_editor.resize(text_editor.x(), 20, text_editor.width(), y1_value as i32);
        terminal_output.resize(terminal_output.x(), (y1_value + 20.0) as i32, text_editor.width(), y2_value as i32);
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
