
use std::{thread,time,sync::mpsc};

use enigo::*;

#[path = "input.rs"]
mod input;

pub fn square() {
    // do something lol
    println!("insquarefunction yay");

    let (sender, receiver) = mpsc::channel();

    let mut enigo: Enigo = Enigo::new();

    let ten_ms = time::Duration::from_millis(10);

    let mut i: i32 = 0;
    let mut j: i32 = 0;

    let handler = thread::spawn(move || {
        input::input(sender);
    });


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

        while i < 100 {

            println!("{}", j );



            if i % 3 == 0 && i != 0 {
                println!("heti pielessä!!!");
                i = 0;
                break;
            }

            enigo.mouse_move_to(j, j);
            i+= 1;
            j += 1;

            thread::sleep(ten_ms);
        }
    }





}