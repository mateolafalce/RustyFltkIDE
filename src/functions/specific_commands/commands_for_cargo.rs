#[path="../../commands/cargo_build.rs"]
mod cargo_build;
#[path="../../commands/cargo_run.rs"]
mod cargo_run;
#[path="../../commands/cargo_run_release.rs"]
mod cargo_run_release;
#[path="../../commands/cargo_clean.rs"]
mod cargo_clean;
#[path="../../commands/cargo_version.rs"]
mod cargo_version;
#[path="../../commands/cargo_help.rs"]
mod cargo_help;
#[path="../../commands/cargo_update.rs"]
mod cargo_update;
#[path="../../commands/cargo_build_release.rs"]
mod cargo_build_release;
#[path="../root/get_root.rs"]
mod get_root;

pub fn commands_for_cargo(
    input: String,
    text: fltk::text::TextBuffer,
    terminal: fltk::text::TextDisplay,
    command_input: &str
){
    let mut root = get_root::get_root();
    root.pop();
    match command_input {
        "cargo build" => {
            cargo_build::cargo_build(input,text.clone(),terminal.clone(), root.clone());
        },
        "cargo b" => {
            cargo_build::cargo_build(input,text.clone(),terminal.clone(), root.clone());
        },
        "cargo build --release" => {
            cargo_build_release::cargo_build_release(input,text.clone(),terminal.clone(), root.clone());
        },
        "cargo run" => {
            cargo_run::cargo_run(input,text.clone(),terminal.clone(), root.clone());
        },
        "cargo r" => {
            cargo_run::cargo_run(input,text.clone(),terminal.clone(), root.clone());
        },
        "cargo run --release" => {
            cargo_run_release::cargo_run_release(input,text.clone(),terminal.clone(), root.clone());
        },
        "cargo clean" => {
            cargo_clean::cargo_clean(input,text.clone(),terminal.clone(), root.clone());
        },
        "cargo --version" => {
            cargo_version::cargo_version(input,text.clone(),terminal.clone(), root.clone());
        },
        "cargo -V" => {
            cargo_version::cargo_version(input,text.clone(),terminal.clone(), root.clone());
        },
        "cargo --help" => {
            cargo_help::cargo_help(input,text.clone(),terminal.clone(), root.clone());
        },
        "cargo -h" => {
            cargo_help::cargo_help(input,text.clone(),terminal.clone(), root.clone());
        },
        "cargo update" => {
            cargo_update::cargo_update(input,text.clone(),terminal.clone(), root.clone());
        },
        _ => (),
}
}
