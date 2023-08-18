// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]


//use tauri::SystemTray;
use enigo::*;
use std::{thread, time, sync::mpsc};
use tauri::{SystemTray, CustomMenuItem, SystemTrayMenu, SystemTrayEvent};


pub mod input;
pub mod cursor;
pub mod click;



#[tauri::command]
fn cursor(state: &str) -> (){
    //println!("{}", state);



    match state {
        "square" => {
            println!("square chosen");
            thread::spawn(move || {
                cursor::square();
            });
            
        }
        "random" => {
            println!("random chosen");
            thread::spawn( || {
                cursor::random();
            });
        }
        _ => {println!("something is wrong")}
    }


}

#[tauri::command(rename_all = "snake_case")]
fn click(cps: &str) -> (){

    let speed = cps.parse::<f64>().unwrap();

    thread::spawn(move || {
        click::click(speed);
    });
    println!("ending click functiooon");
}

#[tauri::command]
async fn set_key_bind() -> () {



    let keys = input::get_key_bind();
    println!("getkeybind ENDED");


    println!("endind set key bind");


   /*  match receiver.try_recv() {
        Ok(_) => {
            println!("received input");
        }
        Err(mpsc::TryRecvError::Disconnected) => {
            println!("disconnected :(");
        }
        _ => {}

    }*/

}



fn main() {


    // Systemtray

    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let tray_menu = SystemTrayMenu::new().add_item(quit);

    let tray = SystemTray::new().with_menu(tray_menu);

    tauri::Builder::default()
        .system_tray(tray)
        .on_system_tray_event(|app, event| match event {
            SystemTrayEvent::MenuItemClick {id, ..} => {
                match id.as_str() {
                    "quit" => {
                        std::process::exit(0);
                    }
                    _ => {}
                }
            }
            _ => {}
        })
        .invoke_handler(tauri::generate_handler![cursor, click, set_key_bind])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
