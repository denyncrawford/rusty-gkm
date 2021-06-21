use rdev::{simulate, Button, EventType, Key, SimulateError};
use std::{thread, time};

fn send(event_type: &EventType) {
    let delay = time::Duration::from_millis(20);
    match simulate(event_type) {
        Ok(()) => (),
        Err(SimulateError) => {
            println!("We could not send {:?}", event_type);
        }
    }
    // Let ths OS catchup (at least MacOS)
    thread::sleep(delay);
}

send(&EventType::KeyPress(Key::KeyS));
send(&EventType::KeyRelease(Key::KeyA));
send(&EventType::KeyRelease(Key::KeyB));
send(&EventType::KeyRelease(Key::KeyB));
send(&EventType::KeyRelease(Key::KeyB));
send(&EventType::KeyRelease(Key::KeyB));
send(&EventType::KeyRelease(Key::KeyB));
send(&EventType::KeyRelease(Key::KeyReturn));