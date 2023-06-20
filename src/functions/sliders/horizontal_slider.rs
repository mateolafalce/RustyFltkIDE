use fltk::prelude::*;
#[path="../event/mouse_select.rs"]
mod mouse_select;
#[path="../event/slider_move.rs"]
mod slider_move;

pub fn horizontal_slider(
    folders: fltk::tree::Tree,
    text_editor: fltk::text::TextEditor,
    terminal_output: fltk::text::TextDisplay,
    terminal_input: fltk::input::Input,
    app: fltk::app::App,
    right_slider: fltk::valuator::NiceSlider
) -> fltk::valuator::HorNiceSlider {
    // Clone the input parameters and assign them to mutable variables
    let mut folders: fltk::tree::Tree = folders.clone();
    let mut text_editor: fltk::text::TextEditor = text_editor.clone();
    let mut terminal_output: fltk::text::TextDisplay = terminal_output.clone();
    let mut terminal_input: fltk::input::Input = terminal_input.clone();
    let app: fltk::app::App = app.clone();
    // Create a new horizontal slider with certain settings
    let mut slider: fltk::valuator::HorNiceSlider = fltk::valuator::HorNiceSlider::new(0, 10, 1000, 10, None);
    slider.set_minimum(0.);
    slider.set_maximum(100.);
    slider.set_step(1., 1);
    slider.set_value(20.);
    // Get the screen size and calculate the width based on the slider value
    let width_size: f64 = fltk::app::screen_size().0;
    slider.set_callback(move |slider_value| {
        let left_width: f64 = width_size * (slider_value.value() / 100.0);
        let right_width: f64 = width_size - left_width;
        // Resize the "folders", "text_editor", "terminal_output", and "terminal_input" based on the slider value
        folders.resize(folders.x(), folders.y(), left_width as i32, folders.height());
        text_editor.resize(left_width as i32, text_editor.y(), right_width as i32 - right_slider.width() as i32, text_editor.height());
        terminal_output.resize(left_width as i32, terminal_output.y(), right_width as i32 - right_slider.width(), terminal_output.height());
        terminal_input.resize(left_width as i32, terminal_input.y(), right_width as i32 - right_slider.width(), terminal_input.height());
        app.redraw(); // Redraw the application
    });
    // Handle the slider events (i.e. push, no event, and leave)
    slider.handle(move |_, event| {
        mouse_select::mouse_select(event);
        slider_move::slider_move(event);
        false
    });
    slider // Return the horizontal slider
}
