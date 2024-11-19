use device_query::{DeviceEvents, DeviceQuery, DeviceState, Keycode};
use std::{str::FromStr, sync::mpsc, thread, time};

pub fn input(sender: mpsc::Sender<bool>, keys: String) {
    //    println!("Current Mouse Coordinates: {:?}", mouse.coords);
    //    let keys: Vec<Keycode> = device_state.get_keys();
    //    println!("Is A pressed? {}", keys.contains(&Keycode::A));

    let device_state = DeviceState::new();

    let mut keys_pressed: Vec<Keycode>;

    let stop: bool = true;

    let mut key1: Keycode = Keycode::LShift;
    let mut key2: Keycode = Keycode::A;

    // String format firstkey + secondkey
    if keys.contains("+") {
        let index = keys.find("+").unwrap();

        let first_key = keys.get(0..(index - 1)).unwrap();
        let second_key = keys.get((index + 2)..).unwrap();

        key1 = Keycode::from_str(first_key).unwrap();
        key2 = Keycode::from_str(second_key).unwrap();
    } else {
        key1 = Keycode::from_str(keys.as_str()).unwrap();
    }

    let ten_ms = time::Duration::from_millis(15);

    loop {
        keys_pressed = device_state.get_keys();

        if keys.contains("+") {
            if keys_pressed.contains(&key1) && keys_pressed.contains(&key2) {
                println!("sending message");
                sender.send(stop).unwrap();
                break;
            }
        } else {
            if keys_pressed.contains(&key1) {
                println!("sending message");
                sender.send(stop).unwrap();
                break;
            }
        }

        thread::sleep(ten_ms);
    }
}

pub fn get_key_bind() -> Vec<Keycode> {
    let device_state = DeviceState::new();

    let mut keys: Vec<Keycode>;

    let fifteen_ms = time::Duration::from_millis(15);

    loop {
        keys = device_state.get_keys();

        if (keys.contains(&Keycode::LShift)
            || keys.contains(&Keycode::LControl)
            || keys.contains(&Keycode::LAlt)
            || keys.contains(&Keycode::RControl)
            || keys.contains(&Keycode::RAlt)
            || keys.contains(&Keycode::RShift))
            && keys.len() > 1
        {
            println!("{}", keys.len());
            return keys;
        } else if !(keys.contains(&Keycode::LShift)
            || keys.contains(&Keycode::LControl)
            || keys.contains(&Keycode::LAlt)
            || keys.contains(&Keycode::RControl)
            || keys.contains(&Keycode::RAlt)
            || keys.contains(&Keycode::RShift))
            && keys.len() != 0
            && keys.len() < 2
        {
            println!("only one key!!!");
            println!("{}", keys.len());
            println!("{}", !keys.contains(&Keycode::LControl));
            return keys;
        }

        println!("in get keys looop");
        thread::sleep(fifteen_ms);
    }
}
