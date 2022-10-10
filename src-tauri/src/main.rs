#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::sync::Mutex;

use anyhow::Result;
use cargo::{CargoReader, Package};

mod cargo;

#[derive(Default)]
struct AppState {
    reader: Mutex<Option<CargoReader>>,
}

impl AppState {
    fn set_reader(&self, reader: CargoReader) {
        *self.reader.lock().unwrap() = Some(reader);
    }
}

fn main() {
    tauri::Builder::default()
        .manage(AppState::default())
        .invoke_handler(tauri::generate_handler![
            greet,
            open_manifest,
            list_packages
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[tauri::command]
fn open_manifest(manifest_path: String, state: tauri::State<AppState>) -> Result<(), String> {
    state.set_reader(CargoReader::open(manifest_path).map_err(|err| format!("{}", err))?);
    Ok(())
}

#[tauri::command]
fn list_packages(state: tauri::State<AppState>) -> Result<Vec<Package>, String> {
    Ok(state
        .reader
        .lock()
        .unwrap()
        .as_ref()
        .ok_or_else(|| "Missing manifest file.".to_string())
        .and_then(|reader| reader.list_packages().map_err(|err| format!("{}", err)))?)
}
