use std::{sync::mpsc, thread, time};

use enigo::{
    Enigo, Mouse, Settings,
    {Coordinate::Abs, Coordinate::Rel},
};
use rand::Rng;

use crate::input;

pub fn square(keys: String) {
    let (sender, receiver) = mpsc::channel();

    let mut enigo: Enigo = Enigo::new(&Settings::default()).unwrap();

    let ten_ms = time::Duration::from_millis(10);

    thread::spawn(move || {
        input::input(sender, keys);
    });

    let (min_x, min_y, max_x, max_y) = calculate_square();

    // Set initial cursor position
    let mut x = min_x;
    let mut y = min_y;
    enigo.move_mouse(min_x, min_y, Abs).unwrap();

    loop {
        // Move cursor
        if x == min_x && y == min_y {
            x += 5;
            enigo.move_mouse(x, y, Abs).unwrap();
        } else if x < max_x && y == min_y {
            x += 5;
            if x > max_x {
                x = max_x;
            }
            enigo.move_mouse(x, y, Abs).unwrap();
        } else if x == max_x && y == min_y {
            y += 5;
            enigo.move_mouse(x, y, Abs).unwrap();
        } else if x == max_x && y < max_y {
            y += 5;
            if y > max_y {
                y = max_y;
            }
            enigo.move_mouse(x, y, Abs).unwrap();
        } else if x == max_x && y == max_y {
            x -= 5;
            enigo.move_mouse(x, y, Abs).unwrap();
        } else if x > min_x && y == max_y {
            x -= 5;
            if x < min_x {
                x = min_x;
            }
            enigo.move_mouse(x, y, Abs).unwrap();
        } else if x == min_x && y == max_y {
            y -= 5;
            enigo.move_mouse(x, y, Abs).unwrap();
        } else if x == min_x && y > min_y {
            y -= 5;
            if y < min_y {
                y = min_y;
            }
            enigo.move_mouse(x, y, Abs).unwrap();
        }

        let (mouse_x, mouse_y) = enigo.location().unwrap();
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
    let mut enigo: Enigo = Enigo::new(&Settings::default()).unwrap();
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
            0 => enigo.move_mouse(0, -amount, Rel).unwrap(),
            1 => enigo.move_mouse(amount, 0, Rel).unwrap(),
            2 => enigo.move_mouse(0, amount, Rel).unwrap(),
            3 => enigo.move_mouse(-amount, 0, Rel).unwrap(),
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

fn calculate_square() -> (i32, i32, i32, i32) {
    let mut enigo: Enigo = Enigo::new(&Settings::default()).unwrap();
    let (width, height) = enigo.main_display().unwrap();

    let (center_x, center_y) = (width / 2, height / 2);

    let unit = (height / 4) / 2;

    let (min_x, min_y) = (center_x - unit, center_y - unit);

    let (max_x, max_y) = (center_x + unit, center_y + unit);

    return (min_x, min_y, max_x, max_y);
}
