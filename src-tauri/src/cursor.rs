
use std::{thread,time,sync::mpsc};

use enigo::*;

#[path = "input.rs"]
mod input;

pub fn square() {

    let (sender, receiver) = mpsc::channel();

    let mut enigo: Enigo = Enigo::new();

    let ten_ms = time::Duration::from_millis(1000);

    let mut i: i32 = 0;
    let mut j: i32 = 0;

    let handler = thread::spawn(move || {
        input::input(sender);
    });


    let (unit, p1x, p1y,p2x, p2y, p3x, p3y, p4x,p4y) = calculate_square();

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

            println!("{}", i );

            match i {
                0 => {enigo.mouse_move_to(p1x, p1y)}
                1 => {enigo.mouse_move_to(p2x, p2y)}
                2 => {enigo.mouse_move_to(p3x, p3y)}
                3 => {enigo.mouse_move_to(p4x, p4y)}
                _ => {}
            }
            thread::sleep(ten_ms);


            if i % 3 == 0 && i != 0 {
                i = 0;
                break;
            }

            //enigo.mouse_move_to(j, j);
            i+= 1;
            j += 1;

            
        }
    }





}

fn calculate_square() -> (i32,i32,i32,i32,i32,i32,i32,i32,i32){

    let mut enigo: Enigo = Enigo::new();
    let (width, height) = enigo.main_display_size();

    let (center_x, center_y) = (width/2, height/2);

    let unit = (height/4)/2;

    let (point1_x, point1_y) = (center_x + unit, center_y - unit);
    
    let (point2_x, point2_y) = (point1_x, point1_y + unit*2);

    let (point3_x, point3_y) = (point2_x - unit*2, point2_y);

    let (point4_x, point4_y) = (point3_x, point3_y - unit*2);

    println!("SQUARE POINTS:");
    println!("{}, {}", point1_x, point1_y);
    println!("{}, {}", point2_x, point2_y);
    println!("{}, {}", point3_x, point3_y);
    println!("{}, {}", point4_x, point4_y);

    return(unit, point1_x, point1_y, point2_x, point2_y, point3_x, point3_y, point4_x, point4_y);

}