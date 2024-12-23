// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

//use tauri::SystemTray;
use device_query::Keycode;
use enigo::*;
use tauri::{ AppHandle};
use std::{sync::mpsc, thread, time};
//use tauri::{CustomMenuItem, tray::TrayIconBuilder, SystemTrayEvent, SystemTrayMenu};
use tauri_plugin_fs::FsExt;

use tauri_plugin_dialog::DialogExt;

pub mod click;
pub mod cursor;
pub mod input;

mod file;
use file::get_file_path;

mod commands;
use commands::execute_commands;

#[tauri::command]
fn cursor(data: Vec<String>) -> () {
    let state: &str = data[0].as_str();
    let keys = data[1].clone();

    match state {
        "square" => {
            println!("square chosen");
            thread::spawn(move || {
                cursor::square(keys);
            });
        }
        "random" => {
            println!("random chosen");
            thread::spawn(|| {
                cursor::random(keys);
            });
        }
        _ => {
            println!("something is wrong")
        }
    }
}

#[tauri::command]
fn click(data: Vec<String>) -> () {
    let speed = data[0].clone().parse::<f64>().unwrap();

    let keys = data[1].clone();

    thread::spawn(move || {
        click::click(speed, keys);
    });
    println!("ending click functiooon");
}

#[tauri::command]
async fn set_key_bind() -> Vec<String> {
    let keys = input::get_key_bind();
    println!("getkeybind ENDED");

    let mut v: Vec<String> = Vec::new();

    for Keycode in keys.iter() {
        v.push(Keycode.to_string());
        println!("{}", Keycode.to_string());
    }
    println!("strings now?");
    return v;

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

#[tauri::command]
fn hold(data: Vec<String>) -> () {
    let key_to_hold = data[0].clone();
    let keys = data[1].clone();

    thread::spawn(move || {
        click::hold(key_to_hold, keys);
    });
}


fn main() {
    // Systemtray

    //let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    //let tray_menu = SystemTrayMenu::new().add_item(quit);

    //let tray = SystemTray::new().with_menu(tray_menu);
    //let newTray = TrayIconBuilder::with_id("my-tray").build(app)?;

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        //.system_tray(tray)
        /* .on_system_tray_event(|app, event| match event {
            SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
                "quit" => {
                    std::process::exit(0);
                }
                _ => {}
            },
            _ => {}
        })*/
        .invoke_handler(tauri::generate_handler![cursor, click, set_key_bind, hold, get_file_path, execute_commands])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
