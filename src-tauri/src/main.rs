// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod server;

use src_shared::rand;
use std::thread;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn random() -> String {
    format!("Your random number, {}! from Rust!", rand())
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            // AppHandle
            let handle = app.handle();
            // Box<AppHandle>
            let boxed_handle = Box::new(handle);
            // closure captures ownership of the boxed_handle variable using the move keyword, which moves the boxed handle into the closure
            thread::spawn(move || {
                server::init(*boxed_handle).unwrap();
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![greet, random])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
