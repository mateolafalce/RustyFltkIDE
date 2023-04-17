use crate::commands::{
    cargo_build::cargo_build,
    cargo_run::cargo_run,
    cargo_run_release::cargo_run_release,
    cargo_clean::cargo_clean,
    cargo_version::cargo_version,
    cargo_help::cargo_help,
    cargo_update::cargo_update,
    cargo_build_release::cargo_build_release,
};
use fltk::text::{
    TextDisplay,
    TextBuffer
};
#[path="../root/get_root.rs"]
mod root;

#[allow(dead_code)]
pub fn commands_for_cargo(
    input: String,
    text: TextBuffer,
    terminal: TextDisplay,
    command_input: &str
) {
    let mut root = root::root();
    root.pop();
    match command_input {
        "cargo build" => {
            cargo_build(input,text.clone(),terminal.clone(), root.clone());
        },
        "cargo b" => {
            cargo_build(input,text.clone(),terminal.clone(), root.clone());
        },
        "cargo build --release" => {
            cargo_build_release(input,text.clone(),terminal.clone(), root.clone());
        },
        "cargo run" => {
            cargo_run(input,text.clone(),terminal.clone(), root.clone());
        },
        "cargo r" => {
            cargo_run(input,text.clone(),terminal.clone(), root.clone());
        },
        "cargo run --release" => {
            cargo_run_release(input,text.clone(),terminal.clone(), root.clone());
        },
        "cargo clean" => {
            cargo_clean(input,text.clone(),terminal.clone(), root.clone());
        },
        "cargo --version" => {
            cargo_version(input,text.clone(),terminal.clone(), root.clone());
        },
        "cargo -V" => {
            cargo_version(input,text.clone(),terminal.clone(), root.clone());
        },
        "cargo --help" => {
            cargo_help(input,text.clone(),terminal.clone(), root.clone());
        },
        "cargo -h" => {
            cargo_help(input,text.clone(),terminal.clone(), root.clone());
        },
        "cargo update" => {
            cargo_update(input,text.clone(),terminal.clone(), root.clone());
        },
        _ => unreachable!(),
}
}
