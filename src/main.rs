use std::{
    sync::{Arc, Mutex},
    thread,
    time::{self, Instant},
};

use rdev::{simulate, EventType, SimulateError};

fn main() {
    println!("[+] Starting...");
    let ts = Arc::new(Mutex::new(Instant::now()));
    let ts2 = ts.clone();

    thread::spawn(move || loop {
        let mut ts = ts.lock().unwrap();
        if Instant::now().duration_since(*ts) > time::Duration::from_secs(120) {
            keep_presence();
            *ts = Instant::now();
        }
    });

    if let Err(error) = rdev::listen(move |event| {
        let mut ts = ts2.lock().unwrap();
        match event.event_type {
            EventType::KeyPress(_key) => {
                *ts = Instant::now();
            }
            EventType::ButtonPress(_button) => {
                *ts = Instant::now();
            }
            EventType::MouseMove { .. } => {
                *ts = Instant::now();
            }
            EventType::Wheel { .. } => {
                *ts = Instant::now();
            }
            _ => (),
        }
    }) {
        println!("Error: {:?}", error);
    }
}

fn keep_presence() {
    send(&EventType::MouseMove { x: 0., y: 0. });
    send(&EventType::MouseMove { x: 200., y: 200. });
    println!("+++ KEEP PRESENCE +++");
}

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
