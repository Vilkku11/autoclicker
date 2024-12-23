use std::thread;
use enigo::{
    Direction::{Click, Press, Release},
    Enigo, Key, Settings, Keyboard,
};
use spin_sleep::{self, SpinSleeper};

use crate::file::read_file;

#[tauri::command]
pub fn execute_commands(file_path: String) -> () {


    thread::spawn(move || {

    let sleeper = SpinSleeper::default();

    let mut enigo = Enigo::new(&Settings::default()).unwrap();


    let mut lines: Vec<String> = Vec::new();

    match read_file(&file_path) {
        Ok(instructions) => {
            lines = instructions.lines().map(|line| line.to_string()).collect();
            println!("{:?}", lines);
        }
        Err(e) => {
            println!("Error reading file: {}", e);
        }
    };

    let arrow_mapping = [
        ("→", Key::RightArrow),
        ("←", Key::LeftArrow),
        ("↑", Key::UpArrow),
        ("↓", Key::DownArrow),

    ];

    sleeper.sleep_s(5.0);

    for line in &lines {
        if line.contains("Level") {
            println!("{}", line);
            sleeper.sleep_s(5.0);
            continue;
        }
        println!("{}", line);

        for char in line.chars() {

            if char.is_whitespace() {
                continue;
            }

            let mut found_arrow = false;

            for (arrow, action) in &arrow_mapping {
                if char.to_string() == *arrow {
                    println!("{:?}", action);
                    enigo.key(*action, Press).unwrap();
                    sleeper.sleep_s(0.1);
                    enigo.key(*action, Release).unwrap();
                    found_arrow = true;
                    break;
                }
            }
            if !found_arrow {
                    enigo.key(Key::Unicode(char), Click).unwrap();

            }

            sleeper.sleep_s(0.8);
        }

    }


    /*for token in &tokens {
        println!("Testing token: {:?}", token);
        enigo.execute(token).unwrap();
        thread::sleep(delay);
    }*/




    });

    

}