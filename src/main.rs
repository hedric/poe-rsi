use rdev::{listen, Event, simulate, Button, EventType, SimulateError};
use std::{thread, time};

static EVENT_DELAY_MS: u64 = 5;

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

fn callback(event: Event) {
    //println!("My callback {:?}", event);
    if let EventType::Wheel {
        delta_x: 0,
        delta_y: _,
    } = event.event_type

    {
        send(&EventType::ButtonPress(Button::Left));
        thread::sleep(time::Duration::from_millis(1));
        //println!("Detected wheel, so lets click {:?}", event);
        send(&EventType::ButtonRelease(Button::Left));
    }
}

fn main() {

    // This will block
    if let Err(error) = listen(callback) {
        println!{"Error: {:?}", error};
    }
}
