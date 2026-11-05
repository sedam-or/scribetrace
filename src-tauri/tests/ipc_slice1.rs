#![allow(clippy::unwrap_used, clippy::expect_used)] // integration tests are not shipped code
//! Slice 1 acceptance (c): the commands (de)serialise through Tauri's mock context.

use std::sync::Arc;
use tauri::ipc::InvokeBody;
use tauri::test::mock_builder;
use tauri::webview::InvokeRequest;
use zotero_standin::StandIn;

struct TestFolderPicker(std::path::PathBuf);
impl scribetrace_shell::FolderPicker for TestFolderPicker {
    fn pick_folder<'a>(
        &'a self,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Option<std::path::PathBuf>> + Send + 'a>>
    {
        Box::pin(std::future::ready(Some(self.0.clone())))
    }
}

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

    let picker = Arc::new(TestFolderPicker(workspace.path().to_path_buf()));
    let session = scribetrace_shell::Session::with_picker(server.port(), picker);

    let app = scribetrace_shell::register_with_session(mock_builder(), session)
        .build(tauri::generate_context!())
        .unwrap();
    let webview = tauri::WebviewWindowBuilder::new(&app, "main", Default::default())
        .build()
        .unwrap();

    let call = |cmd: &str, body: serde_json::Value| {
        tauri::test::get_ipc_response(&webview, request(cmd, body))
            .map(|r| r.deserialize::<serde_json::Value>().unwrap())
    };

    // SEC-16: Strict capabilities allowlist verification.
    let cap_path =
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("capabilities/default.json");
    let cap_data: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(cap_path).unwrap()).unwrap();
    let permissions = cap_data["permissions"].as_array().unwrap();
    let perm_strings: Vec<&str> = permissions.iter().map(|p| p.as_str().unwrap()).collect();
    assert_eq!(
        perm_strings,
        vec![
            "allow-choose-folder",
            "allow-get-workspace",
            "allow-list-zotero-papers",
            "allow-import-marks",
            "allow-read-history"
        ],
        "Capabilities allowlist must strictly match the 5 allowed commands"
    );

    // SEC-16/18: Arbitrary filesystem access is denied.
    assert!(call(
        "plugin:fs|read_file",
        serde_json::json!({"path":"/etc/passwd"})
    )
    .is_err());

    // SEC-16/18 negative test: dialog plugin is not exposed, no dialog capabilities exist.
    assert!(
        call("plugin:dialog|open", serde_json::json!({})).is_err(),
        "plugin:dialog must not exist on the IPC allowlist"
    );
    assert!(
        call("dialog:open", serde_json::json!({})).is_err(),
        "dialog:open must not exist on the IPC allowlist"
    );

    // SEC-18 negative test: open_workspace accepting a raw path argument was REMOVED.
    assert!(
        call(
            "open_workspace",
            serde_json::json!({"path": workspace.path()})
        )
        .is_err(),
        "open_workspace must not exist on the IPC allowlist"
    );

    // No folder chosen yet: import_marks fails with a typed error contract { "code": "<code>" }.
    let err_resp = tauri::test::get_ipc_response(
        &webview,
        request(
            "import_marks",
            serde_json::json!({"paperId":"z_EFGH6789","attachmentKey":"EFGH6789"}),
        ),
    )
    .unwrap_err();
    assert_eq!(
        err_resp["code"], "internal",
        "IPC error must strictly conform to {{ \"code\": \"<code>\" }}"
    );

    // List papers via IPC command list_zotero_papers (SEC-16)
    let papers_val = call("list_zotero_papers", serde_json::json!({})).unwrap();
    let papers_arr = papers_val.as_array().unwrap();
    assert_eq!(
        papers_arr.len(),
        3,
        "Expected 3 papers from stand-in fixtures"
    );
    assert!(papers_arr
        .iter()
        .any(|p| p["id"] == "z_6WG6QNHB" && p["marksCount"] == 2));

    // Initially get_workspace returns null/None.
    let initial_ws = call("get_workspace", serde_json::json!({})).unwrap();
    assert!(initial_ws.is_null());

    // choose_folder opens via the port picker (no path argument accepted from UI).
    let chosen = call("choose_folder", serde_json::json!({})).unwrap();
    assert!(chosen.is_object());
    assert!(chosen["path"].is_string());

    // get_workspace now returns the configured folder.
    let active_ws = call("get_workspace", serde_json::json!({})).unwrap();
    assert_eq!(active_ws["path"], chosen["path"]);

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
