//! ScribeTrace desktop entry point.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    scribetrace_shell::run(tauri::generate_context!());
}
