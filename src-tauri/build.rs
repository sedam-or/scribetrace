fn main() {
    // Only these commands exist for the UI; each gets an `allow-*` permission in capabilities/.
    let manifest = tauri_build::AppManifest::new().commands(&[
        "choose_folder",
        "get_workspace",
        "list_zotero_papers",
        "import_marks",
        "read_history",
    ]);
    if let Err(err) = tauri_build::try_build(tauri_build::Attributes::new().app_manifest(manifest))
    {
        eprintln!("tauri-build failed: {err:#}");
        std::process::exit(1);
    }
}
