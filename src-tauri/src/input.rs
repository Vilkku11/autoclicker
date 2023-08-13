

use device_query::{DeviceQuery, DeviceState, Keycode};
use std::{sync::mpsc,str::FromStr, thread, time };


pub fn input(sender: mpsc::Sender<bool>) {

    //    println!("Current Mouse Coordinates: {:?}", mouse.coords);
    //    let keys: Vec<Keycode> = device_state.get_keys();
    //    println!("Is A pressed? {}", keys.contains(&Keycode::A));


    let device_state = DeviceState::new();
 


    let mut keys: Vec<Keycode>;
    
    let stop: bool = true;


    
    let s = "LShift";

    let xtest = Keycode::from_str(s).unwrap();


     let ten_ms = time::Duration::from_millis(10);
    loop {

        keys = device_state.get_keys();

       // println!("Current Mouse Coordinates: {:?}", mouse1.coords);
       // println!("Keyboard: {:?}", keys.last().is_none());
        

         
        if keys.contains(&Keycode::A) && keys.contains(&xtest) {
            println!("sending message");
            sender.send(stop).unwrap();
            break;
        }

        thread::sleep(ten_ms);
    }
    


}


