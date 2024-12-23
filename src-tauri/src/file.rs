use tauri::AppHandle;
use tauri_plugin_dialog::DialogExt;
use std::sync::{Arc, Mutex};
use tokio::sync::Notify;

use std::fs::File;
use std::io::{self, Read};

#[tauri::command]
pub async fn get_file_path (app: AppHandle) -> String{

    //let mut result = "No file selected".to_string();
    let result = Arc::new(Mutex::new("No file selected".to_string()));
    let notify = Arc::new(Notify::new());

    
    let result_clone = Arc::clone(&result);
    let notify_clone = Arc::clone(&notify);

    tokio::spawn(async move {
        app.dialog().file().pick_file(move |file_path| {
        
            let mut result = result_clone.lock().unwrap();
            
            match file_path {
                Some(path) => {
                    println!("Selected file: {:?}", path);
                    *result = path.to_string();
                }
                None => {
                    println!("No file selecetd");
                }
            }

            notify_clone.notify_one();
        });
    });
        

        notify.notified().await;

        let result = result.lock().unwrap();
        return result.clone();

    }


pub fn read_file(file_path: &String) -> Result<String, io::Error> {
    let mut file = File::open(file_path)?;

    let mut contents = String::new();

    file.read_to_string(&mut contents)?;

    Ok(contents)
}