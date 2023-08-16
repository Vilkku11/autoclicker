// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]


//use tauri::SystemTray;
use enigo::*;
use std::{thread, time, sync::mpsc};
use tauri::{SystemTray, CustomMenuItem, SystemTrayMenu, SystemTrayEvent};


pub mod input;
pub mod cursor;
pub mod click;






// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}


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
}

#[tauri::command]
async fn set_key_bind() -> () {

    let (sender, receiver) = mpsc::channel();

   let handler =  thread::spawn(move|| {
        input::input(sender);
    });

    let res = handler.join();


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
            Err(mpsc::TryRecvError::Disconnected) => {
                println!("disconnected :(");
                break;
            }
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
        .invoke_handler(tauri::generate_handler![greet, test, cursor, click, set_key_bind])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
