// ScribeTrace Spike 4: Minimal Tauri v2 Webview Harness
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::{Deserialize, Serialize};
use std::time::Instant;

#[derive(Debug, Serialize, Deserialize)]
pub struct WebviewMetrics {
    pub cold_first_page_ms: u64,
    pub idle_rss_mb: f64,
    pub selected_text_check: bool,
    pub pages_navigated: usize,
    pub platform: String,
}

fn get_process_rss_mb() -> f64 {
    #[cfg(target_os = "linux")]
    {
        if let Ok(statm) = std::fs::read_to_string("/proc/self/statm") {
            let parts: Vec<&str> = statm.split_whitespace().collect();
            if parts.len() >= 2 {
                if let Ok(pages) = parts[1].parse::<u64>() {
                    let page_size_kb = 4; // standard 4KB pages
                    return (pages * page_size_kb) as f64 / 1024.0;
                }
            }
        }
    }
    0.0
}

#[tauri::command]
fn report_benchmark_metrics(cold_first_page_ms: u64, selected_text_check: bool) {
    let rss = get_process_rss_mb();
    let metrics = WebviewMetrics {
        cold_first_page_ms,
        idle_rss_mb: (rss * 10.0).round() / 10.0,
        selected_text_check,
        pages_navigated: 10,
        platform: std::env::consts::OS.to_string(),
    };

    println!("=== TAURI WEBVIEW BENCHMARK OUTPUT ===");
    if let Ok(json) = serde_json::to_string_pretty(&metrics) {
        println!("{json}");
    }
    std::process::exit(0);
}

fn main() {
    let start_time = Instant::now();

    // Check if running in a headless environment without display on Linux
    #[cfg(target_os = "linux")]
    {
        if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() {
            eprintln!("No graphical display ($DISPLAY or $WAYLAND_DISPLAY) detected for webview.");
            eprintln!("Run under xvfb-run or a graphical desktop session.");
            println!("{{\"spike\":\"SPIKE-4\",\"status\":\"NOT_RUN\",\"reason\":\"headless_environment_no_display\"}}");
            return;
        }
    }

    println!("Starting ScribeTrace Tauri v2 webview benchmark...");
    tauri::Builder::default()
        .setup(move |_app| {
            let elapsed_ms = start_time.elapsed().as_millis() as u64;
            println!("Tauri window initialized in {elapsed_ms} ms");
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![report_benchmark_metrics])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
