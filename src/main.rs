use chrono::Local;
use enigo::{Coordinate, Enigo, Mouse, Settings};
use env_logger::Builder;
use kv_log_macro as log;
use rdev::EventType;
use std::io::Write;
use std::{
    sync::{Arc, Mutex},
    thread,
    time::{self, Instant},
};

const KEEP_PRESENCE_INTERVAL: u64 = 5 * 60;

fn main() {
    Builder::new()
        .format(|buf, record| {
            writeln!(
                buf,
                "{} [{}] - {}",
                Local::now().format("%Y-%m-%dT%H:%M:%S"),
                record.level(),
                record.args()
            )
        })
        .filter(None, femme::LevelFilter::Info)
        .init();

    log::info!("Starting...");

    let ts = Arc::new(Mutex::new(Instant::now()));
    let ts2 = ts.clone();

    thread::spawn(move || loop {
        thread::sleep(time::Duration::from_secs(1));

        let mut ts = ts.lock().unwrap();
        let now = Instant::now();
        if now.duration_since(*ts) >= time::Duration::from_secs(KEEP_PRESENCE_INTERVAL) {
            keep_presence();
            *ts = Instant::now();
        }
    });

    if let Err(error) = rdev::listen(move |event| match event.event_type {
        EventType::KeyPress(_key) => {
            let mut ts = ts2.lock().unwrap();
            *ts = Instant::now();
            log::debug!("key press");
        }
        EventType::ButtonPress(_button) => {
            let mut ts = ts2.lock().unwrap();
            *ts = Instant::now();
            log::debug!("button press");
        }
        EventType::MouseMove { .. } => {
            let mut ts = ts2.lock().unwrap();
            *ts = Instant::now();
            log::debug!("mouse move");
        }
        EventType::Wheel { .. } => {
            let mut ts = ts2.lock().unwrap();
            *ts = Instant::now();
            log::debug!("wheel");
        }
        _ => (),
    }) {
        log::error!("Error: {:?}", error);
    }
}

fn keep_presence() {
    let mut enigo = Enigo::new(&Settings::default()).unwrap();
    enigo.move_mouse(1, 1, Coordinate::Rel).unwrap();
    enigo.move_mouse(-1, -1, Coordinate::Rel).unwrap();
    log::info!("KEEP PRESENCE");
}
