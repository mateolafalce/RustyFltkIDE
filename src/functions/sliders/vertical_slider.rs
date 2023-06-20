use fltk::prelude::*;
#[path="../event/mouse_select.rs"]
mod mouse_select;
#[path="../event/slider_move.rs"]
mod slider_move;

pub fn vertical_slider(
    mut text_editor: fltk::text::TextEditor,
    mut terminal_output: fltk::text::TextDisplay,
    terminal_input: fltk::input::Input,
    app: fltk::app::App
) -> fltk::valuator::NiceSlider {
    let mut vertical_slider: fltk::valuator::NiceSlider = fltk::valuator::NiceSlider::new(990, 20, 10, 580, None);
    vertical_slider.set_minimum(0.);
    vertical_slider.set_maximum(100.);
    vertical_slider.set_step(1., 1);
    vertical_slider.set_value(66.);
    let height: f64 = fltk::app::screen_size().1;
    let vertical_slider_: fltk::valuator::NiceSlider = vertical_slider.clone();
    vertical_slider.set_callback(move |slider_value| {
        let mut top_height: f64 = (height * (slider_value.value() / 100.0)) - vertical_slider_.width() as f64;
        let mut y_for_terminal_output: i32 = top_height as i32 + terminal_input.height();
        let bottom_height: f64 = height - top_height - (terminal_input.height() as f64 * 1.5);
        if top_height <= height * (10.0 / 100.0) {
            top_height = height * (10.0 / 100.0);
            y_for_terminal_output = top_height as i32 - (terminal_input.height() as f64 * 1.5) as i32;
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
        mouse_select::mouse_select(event);
        slider_move::slider_move(event);
        false
    });
    vertical_slider
}
