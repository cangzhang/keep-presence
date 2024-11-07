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

    thread::spawn(move || {
        let mut enigo = Enigo::new(&Settings::default()).unwrap();
        loop {
            let mut ts = ts.lock().unwrap();
            if Instant::now().duration_since(*ts)
                > time::Duration::from_secs(KEEP_PRESENCE_INTERVAL)
            {
                keep_presence(&mut enigo);
                *ts = Instant::now();
            }
            thread::sleep(time::Duration::from_millis(300));
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
        log::error!("Error: {:?}", error);
    }
}

fn keep_presence(enigo: &mut Enigo) {
    enigo.move_mouse(1, 1, Coordinate::Rel).unwrap();
    enigo.move_mouse(-1, -1, Coordinate::Rel).unwrap();
    log::info!("+++ KEEP PRESENCE +++");
}
