use std::{sync::mpsc, thread, time};

use enigo::{
    Button,
    Direction::{Click, Press, Release},
    Enigo, Key, Keyboard, Mouse, Settings,
};

use crate::input;

use spin_sleep::LoopHelper;

pub fn click(speed: f64, keys: String) {
    let (sender, receiver) = mpsc::channel();

    let mut enigo: Enigo = Enigo::new(&Settings::default()).unwrap();

    thread::spawn(move || {
        input::input(sender, keys);
    });

    let wait_time = time::Duration::from_millis(1000);
    thread::sleep(wait_time);

    let mut loop_helper = LoopHelper::builder().build_with_target_rate(speed);

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
        //enigo.mouse_click(MouseButton::Left);
        enigo.button(Button::Left, Click).unwrap();
        loop_helper.loop_sleep();
    }
}

pub fn hold(key_to_hold: String, keys: String) {
    let (sender, receiver) = mpsc::channel();
    let mut enigo: Enigo = Enigo::new(&Settings::default()).unwrap();

    thread::spawn(move || {
        input::input(sender, keys);
    });

    let wait_time = time::Duration::from_millis(1000);
    thread::sleep(wait_time);
    let key = Key::Unicode(' ');
    println!("{:?}", key);

    //enigo.key_down(Key::Space);
    enigo.key(Key::Space, Press).unwrap();

    loop {
        match receiver.try_recv() {
            Ok(_) => {
                println!("input received");
                enigo.key(Key::Space, Release).unwrap();
                break;
            }
            Err(mpsc::TryRecvError::Disconnected) => {
                println!("err disconnected");
                enigo.key(Key::Space, Release).unwrap();
                break;
            }
            _ => {}
        }
    }
}
