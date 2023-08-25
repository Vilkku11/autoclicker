
use std::{thread,time,sync::mpsc};

use enigo::*;
use rand::Rng;


use crate::input;

pub fn square(keys: String) {

    let (sender, receiver) = mpsc::channel();

    let mut enigo: Enigo = Enigo::new();

    let ten_ms = time::Duration::from_millis(10);


    thread::spawn(move || {
        input::input(sender, keys);
    });


    let (min_x, min_y, max_x, max_y) = calculate_square();


    // Set initial cursor position
    let mut x = min_x;
    let mut y = min_y;
    enigo.mouse_move_to(min_x,min_y);

    loop {


        // Move cursor
        if x == min_x && y == min_y {
            x += 5;
            enigo.mouse_move_to(x, y);
        }else if x < max_x && y == min_y {
            x += 5;
            if x > max_x {
                x = max_x;
            }
            enigo.mouse_move_to( x , y);
        }else if x == max_x && y == min_y {
            y += 5;
            enigo.mouse_move_to(x, y);
        }else if x == max_x && y < max_y {
            y += 5;
            if y > max_y {
                y = max_y;
            }
            enigo.mouse_move_to(x, y);
        }else if x == max_x && y == max_y {
            x -= 5;
            enigo.mouse_move_to(x, y);
        }else if x > min_x && y == max_y {
            x -= 5;
            if x < min_x {
                x = min_x;
            }
            enigo.mouse_move_to(x, y);
        }else if x == min_x && y == max_y {
            y -= 5;
            enigo.mouse_move_to(x, y);
        }else if x == min_x && y > min_y {
            y -= 5;
            if y < min_y {
                y = min_y;
            }
            enigo.mouse_move_to(x, y);
        }

        let (mouse_x, mouse_y) = enigo.mouse_location();
        println!("{}, {}", mouse_x, mouse_y);


        // Check termination input
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


        thread::sleep(ten_ms);

    }





}


pub fn random(keys: String) -> () {
    
    let (sender, receiver) = mpsc::channel();
    let mut enigo: Enigo = Enigo::new();
    let ten_ms = time::Duration::from_millis(10);


    let mut direction: i32;
    let mut amount: i32;


    thread::spawn(move || {
        input::input(sender, keys);
    });

    loop {

        
        // calculate direction (up, down, right left) and how many pixels)
        direction = rand::thread_rng().gen_range(0..4);
        amount = rand::thread_rng().gen_range(1..6);

        match direction {
            0 => {
                enigo.mouse_move_relative(0,-amount)
            }
            1 => {
                enigo.mouse_move_relative(amount, 0)
            }
            2 => {
                enigo.mouse_move_relative(0, amount)
            }
            3 => {
                enigo.mouse_move_relative(-amount, 0)
            }
            _ => {}
        }

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

        thread::sleep(ten_ms);

    }




}



fn calculate_square() -> (i32,i32,i32,i32){

    let  enigo: Enigo = Enigo::new();
    let (width, height) = enigo.main_display_size();

    let (center_x, center_y) = (width/2, height/2);

    let unit = (height/4)/2;

    let (min_x, min_y) = (center_x - unit, center_y - unit);

    let (max_x, max_y) = (center_x + unit, center_y + unit);


    return(min_x, min_y, max_x, max_y);

}