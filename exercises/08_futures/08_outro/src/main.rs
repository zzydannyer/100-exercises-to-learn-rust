#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use outro_08::{app, state::AppState};
use std::time::Duration;

fn main() {
    std::thread::spawn(|| {
        let runtime = tokio::runtime::Runtime::new().expect("tokio runtime");
        runtime.block_on(async {
            let state = AppState::default();
            let router = app(state);
            let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
                .await
                .expect("bind 3000");
            println!("API listening on http://127.0.0.1:3000");
            axum::serve(listener, router).await.expect("axum serve");
        });
    });

    std::thread::sleep(Duration::from_millis(300));

    tauri::Builder::default()
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
