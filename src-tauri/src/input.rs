

use device_query::{DeviceQuery, DeviceEvents, DeviceState, MouseState, Keycode};
use std::sync::mpsc;
use std::str::FromStr;

pub fn input(sender: mpsc::Sender<bool>) {

    //    println!("Current Mouse Coordinates: {:?}", mouse.coords);
    //    let keys: Vec<Keycode> = device_state.get_keys();
    //    println!("Is A pressed? {}", keys.contains(&Keycode::A));


    let device_state = DeviceState::new();
    let mouse: MouseState = device_state.get_mouse();

    let mut mouse1: MouseState;
    let mut keys: Vec<Keycode>;
    
    let stop: bool = true;


    
    let s = "LShift";

    let xtest = Keycode::from_str(s).unwrap();


     
    loop {
        mouse1 = device_state.get_mouse();
        keys = device_state.get_keys();

       // println!("Current Mouse Coordinates: {:?}", mouse1.coords);
       // println!("Keyboard: {:?}", keys.last().is_none());
        
    




         
        if keys.contains(&Keycode::A) && keys.contains(&xtest) {
            println!("sending message");
            sender.send(stop).unwrap();
            break;
        }

    }
    


}


