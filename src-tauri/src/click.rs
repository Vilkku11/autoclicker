use std::{sync::mpsc, thread, time};

use enigo::{
    Button,
    Direction::{Click, Press, Release},
    Enigo, Key, Keyboard, Mouse, Settings,
};

use crate::input;

pub fn click(speed: f64, keys: String) {
    let (sender, receiver) = mpsc::channel();

    let mut enigo: Enigo = Enigo::new(&Settings::default()).unwrap();
    let sleep_time: f64 = 1.0 / speed;


    thread::spawn(move || {
        input::input(sender, keys);
    });

    let wait_time = time::Duration::from_millis(1000);
    thread::sleep(wait_time);

    let spin_sleeper = spin_sleep::SpinSleeper::new(100_000)
        .with_spin_strategy(spin_sleep::SpinStrategy::YieldThread);


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
        //enigo.mouse_click(MouseButton::Left);
        enigo.button(Button::Left, Click).unwrap();

        spin_sleeper.sleep_s(sleep_time);
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
