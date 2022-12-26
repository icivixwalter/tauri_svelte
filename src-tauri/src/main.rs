#![cfg_attr(
all(not(debug_assertions), target_os = "windows"),
windows_subsystem = "windows"
)]

mod access;

use crate::access::access;

//altra funzione
#[tauri::command]
fn greet() -> String {
    println!("SONO IN RUST QUI");
    format!("Hello! You've been greeted from Rust!")
}


#[tauri::command]
fn connettiaccess() -> Vec<String> {
    println!("Eseguo la query");
    access()
}

fn main() {
    //inizializzo la connessione
    env_logger::init();
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![connettiaccess, greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
