// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]


//use tauri::SystemTray;
use enigo::*;



// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn test() -> (){

    let mut enigo = Enigo::new();

    enigo.mouse_move_to(500, 200);
    enigo.mouse_move_to(500, 300);
    enigo.mouse_move_to(400, 200);
}


fn main() {



    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, test])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
