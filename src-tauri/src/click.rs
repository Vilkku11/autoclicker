
use std::{thread,time,sync::mpsc};

use enigo::*;


use crate::input;

pub fn click (speed: u64) {


    let (sender, receiver) = mpsc::channel();

    let mut enigo: Enigo = Enigo::new();

    thread::spawn(move || {
        input::input(sender);
    });

    let wait_time = time::Duration::from_millis(1000);
    thread::sleep(wait_time);


    let time_between_clicks = 1000 / speed;



    let wait_between_clicks = time::Duration::from_millis(time_between_clicks);
    
    loop {

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
        thread::sleep(wait_between_clicks);

    }


}