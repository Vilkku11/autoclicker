// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]


//use tauri::SystemTray;
use enigo::*;
use std::{thread, time, sync::mpsc};


pub mod input;






// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
async fn test() -> (){

    let mut enigo = Enigo::new();

    let ten_ms = time::Duration::from_millis(100);

    let (sender, receiver) = mpsc::channel();

    thread::spawn(move|| {
        input::input(sender);
    });

    for i in 0..100{

        match receiver.try_recv() {
            Ok(_) => {
                println!("received input");
                break;
            }
            Err(mpsc::TryRecvError::Disconnected) => break,
            _ => {}
        }


        enigo.mouse_move_to(i, i);
        thread::sleep(ten_ms);
    }


    

    //enigo.mouse_move_to(i, i);
    //enigo.mouse_move_to(500, 300);
    //enigo.mouse_move_to(400, 200);
}


fn main() {



    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, test])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
