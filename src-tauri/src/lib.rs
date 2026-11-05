//! Thin IPC gateway. One command per use case; no logic of its own beyond path confinement.
//! The UI can reach only the commands listed in `build.rs` and `capabilities/default.json`.

mod commands;

pub use commands::{Session, WorkspaceInfo};

/// Zotero's local API port; the only network endpoint the app ever contacts, and only on click.
pub const ZOTERO_PORT: u16 = 23119;

/// Registers the session state and the three allowed commands on `builder`.
/// Shared by `run` and by the mock-context tests so both exercise the same wiring.
pub fn register<R: tauri::Runtime>(
    builder: tauri::Builder<R>,
    zotero_port: u16,
) -> tauri::Builder<R> {
    builder
        .manage(Session::new(zotero_port))
        .invoke_handler(tauri::generate_handler![
            commands::open_workspace,
            commands::import_marks,
            commands::read_history
        ])
}

/// Builds the app and runs it until the window closes.
pub fn run(context: tauri::Context) {
    let result = register(tauri::Builder::default(), ZOTERO_PORT).run(context);
    if let Err(err) = result {
        eprintln!("ScribeTrace failed to start: {err}");
        std::process::exit(1);
    }
}
