#![allow(clippy::unwrap_used, clippy::expect_used)] // integration tests are not shipped code
//! Slice 1 acceptance (c): the commands (de)serialise through Tauri's mock context.

use tauri::ipc::InvokeBody;
use tauri::test::mock_builder;
use tauri::webview::InvokeRequest;
use zotero_standin::StandIn;

fn request(cmd: &str, body: serde_json::Value) -> InvokeRequest {
    InvokeRequest {
        cmd: cmd.into(),
        callback: tauri::ipc::CallbackFn(0),
        error: tauri::ipc::CallbackFn(1),
        // WebView2 serves the app from http://tauri.localhost; the other OS use tauri://localhost.
        url: (if cfg!(windows) {
            "http://tauri.localhost"
        } else {
            "tauri://localhost"
        })
        .parse()
        .unwrap(),
        body: InvokeBody::Json(body),
        headers: Default::default(),
        invoke_key: tauri::test::INVOKE_KEY.to_string(),
    }
}

#[test]
fn import_then_read_round_trips_through_ipc() {
    let fixtures =
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../tests/fixtures/zotero");
    let server = StandIn::start(fixtures, 0).unwrap();
    let workspace = tempfile::tempdir().unwrap();

    let app = scribetrace_shell::register(mock_builder(), server.port())
        .build(tauri::generate_context!())
        .unwrap();
    let webview = tauri::WebviewWindowBuilder::new(&app, "main", Default::default())
        .build()
        .unwrap();

    let call = |cmd: &str, body: serde_json::Value| {
        tauri::test::get_ipc_response(&webview, request(cmd, body))
            .map(|r| r.deserialize::<serde_json::Value>().unwrap())
    };

    // Anything outside the three allowed commands is refused by the capability file.
    assert!(call(
        "plugin:fs|read_file",
        serde_json::json!({"path":"/etc/passwd"})
    )
    .is_err());

    // No folder yet: a plain, typed error.
    assert!(call(
        "import_marks",
        serde_json::json!({"paperId":"z_EFGH6789","attachmentKey":"EFGH6789"})
    )
    .is_err());

    let opened = call(
        "open_workspace",
        serde_json::json!({"path": workspace.path()}),
    )
    .unwrap();
    assert!(opened["path"].is_string());

    let report = call(
        "import_marks",
        serde_json::json!({"paperId":"z_EFGH6789","attachmentKey":"EFGH6789"}),
    )
    .unwrap();
    assert_eq!(report["imported"], 1);

    let history = call("read_history", serde_json::json!({"paperId":"z_EFGH6789"})).unwrap();
    let events = history.as_array().unwrap();
    assert_eq!(events.len(), 1);
    assert_eq!(events[0]["type"], "mark_import");
    assert_eq!(events[0]["author"], "Me");
    assert_eq!(events[0]["payload"]["key"], "WXYZ2345");
}
