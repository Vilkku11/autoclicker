
use std::{thread,time,sync::mpsc};

use enigo::*;


use crate::input;


use spin_sleep::LoopHelper;


pub fn click (speed: f64) {


    let (sender, receiver) = mpsc::channel();

    let mut enigo: Enigo = Enigo::new();

    thread::spawn(move || {
        input::input(sender);
    });

    let wait_time = time::Duration::from_millis(1000);
    thread::sleep(wait_time);




   let mut loop_helper = LoopHelper::builder()
        .build_with_target_rate(speed);

    
    loop {

        loop_helper.loop_start();

        match receiver.try_recv() {
            Ok(_) => {
                println!("input received");
                break;
            }
            Err(mpsc::TryRecvError::Disconnected) => {
                println!("err disconnected");
                break;
            }
            _ => {}
        }

        println!("click once");
        enigo.mouse_click(MouseButton::Left);
        loop_helper.loop_sleep();
    }


}

pub fn hold () {
    
    let (sender, receiver) = mpsc::channel();
    let mut enigo: Enigo = Enigo::new();
    thread::spawn(move || {
        input::input(sender);
    });

    let wait_time = time::Duration::from_millis(1000);
    thread::sleep(wait_time);


    enigo.key_down(Key::Space);

    loop {
        
        match receiver.try_recv() {
            Ok(_) => {
                println!("input received");
                enigo.key_up(Key::Space);
                break;
            }
            Err(mpsc::TryRecvError::Disconnected) => {
                println!("err disconnected");
                enigo.key_up(Key::Space);
                break;
            }
            _ => {}
        }
    }


}