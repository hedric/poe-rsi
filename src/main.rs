use rdev::{grab, simulate, Button, Event, EventType, SimulateError};
use std::{thread, time};

static EVENT_DELAY_MS: u64 = 2;

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
        if let EventType::Wheel {
            delta_x: 0,
            delta_y: _,
        } = event.event_type
        {
            send(&EventType::ButtonPress(Button::Left));
            //println!("Detected wheel, so lets click {:?}", event);
            send(&EventType::ButtonRelease(Button::Left));
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
