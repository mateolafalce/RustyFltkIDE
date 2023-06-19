#[path="./center.rs"]
mod center;

// Display an error message dialog at the center of the screen
pub fn error(e: &str) -> bool{
    fltk::dialog::alert(
        center::center().0 - 100,
        center::center().1 - 100,
        &format!("Error: {}", e)
    );
    true
}
