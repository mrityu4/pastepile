use rdev::{listen, Event, EventType, Key};

fn main() {
    println!("Character");
    listen(callback);
}

fn callback(event: Event) {
    match event.event_type {
        EventType::KeyPress(key) => match key {
            Key::Character(s) => println!("Character: {}", s),
            Key::Raw(code) => println!("Raw code: {}", code),
            Key::Special(special_key) => println!("Special key: {:?}", special_key),
            _ => (),
        },
        _ => (),
    }
}
use std::thread;
// ... other imports ...

