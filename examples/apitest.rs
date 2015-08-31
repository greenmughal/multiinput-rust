extern crate multiinput;
extern crate time;

use multiinput::*;
fn main() {
    let mut manager = RawInputManager::new().unwrap();
    manager.register_devices(DeviceType::Joysticks);
    manager.register_devices(DeviceType::Keyboards);
    manager.register_devices(DeviceType::Mice);
    'outer: loop{
        if let Some(event) = manager.get_event(){
            match event{
                RawEvent::KeyboardEvent(_,  KeyId::Escape, State::Pressed)
                    => break 'outer,
                _ => (),
            }
            println!("{:?}", event);
        }
    }
    println!("Finishing");
}