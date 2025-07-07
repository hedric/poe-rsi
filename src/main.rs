use rdev::{grab, simulate, Button, Event, EventType, Key, SimulateError};
use std::sync::atomic::{AtomicBool, Ordering};
use std::{thread, time};

static EVENT_DELAY_MS: u64 = 2;
static CTRL_PRESSED: AtomicBool = AtomicBool::new(false);

fn send(event_type: &EventType) {
    let delay = time::Duration::from_millis(EVENT_DELAY_MS);
    match simulate(event_type) {
        Ok(()) => (),
        Err(SimulateError) => {
            println!("We could not send {:?}", event_type);
        }
    }
    thread::sleep(delay);
}

fn main() {
    let callback = |event: Event| -> Option<Event> {
        let ctrl_pressed = CTRL_PRESSED.load(Ordering::Relaxed);

        if let EventType::Wheel {
            delta_x: 0,
            delta_y: _,
        } = event.event_type
        {
            if ctrl_pressed {
                //println!("ctrl pressed and wheel, so lets click {:?}", event);
                send(&EventType::KeyPress(Key::ControlLeft));
                send(&EventType::ButtonPress(Button::Left));
                send(&EventType::ButtonRelease(Button::Left));
                send(&EventType::KeyRelease(Key::ControlLeft));
                None
            } else {
                Some(event)
            }
        } else if let EventType::KeyPress(Key::ControlLeft) = event.event_type {
            //println!("Detected left ctrl press {:?}", event);
            send(&EventType::KeyPress(Key::ControlLeft));
            CTRL_PRESSED.store(true, Ordering::Relaxed);
            None
        } else if let EventType::KeyRelease(Key::ControlLeft) = event.event_type {
            //println!("Detected left ctrl release {:?}", event);
            send(&EventType::KeyRelease(Key::ControlLeft));
            CTRL_PRESSED.store(false, Ordering::Relaxed);
            None
        } else {
            Some(event)
        }
    };

    // This will block
    if let Err(error) = grab(callback) {
        println! {"Error: {:?}", error};
    }
}
