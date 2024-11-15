mod ui;

use chrono::Local;
use enigo::{Coordinate, Enigo, Mouse, Settings};
use env_logger::Builder;
use floem::prelude::create_rw_signal;
use floem::reactive::{create_effect, SignalGet};
use kv_log_macro as log;
use rdev::EventType;
use std::io::Write;
use std::{
    sync::{Arc, Mutex},
    time::{self, Instant},
};
use tokio::task::JoinHandle;

const KEEP_PRESENCE_INTERVAL: u64 = 5 * 60;

#[tokio::main]
async fn main() {
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
    let presence_interval = create_rw_signal(KEEP_PRESENCE_INTERVAL);

    let ts_clone = ts.clone();
    let handle = Arc::new(Mutex::new(None::<JoinHandle<()>>));
    create_effect(move |_| {
        log::info!("=== presence interval: {}", presence_interval.get());
        let mut handle = handle.lock().unwrap();
        if let Some(handle) = handle.as_mut() {
            handle.abort();
        }
        let new_handle = spawn_timer(ts_clone.clone(), presence_interval.get());
        *handle = Some(new_handle);
    });

    ui::run(presence_interval);

    if let Err(error) = rdev::listen(move |event| match event.event_type {
        EventType::KeyPress(_key) => {
            let mut ts = ts.lock().unwrap();
            *ts = Instant::now();
            log::debug!("key press");
        }
        EventType::ButtonPress(_button) => {
            let mut ts = ts.lock().unwrap();
            *ts = Instant::now();
            log::debug!("button press");
        }
        EventType::MouseMove { .. } => {
            let mut ts = ts.lock().unwrap();
            *ts = Instant::now();
            log::debug!("mouse move");
        }
        EventType::Wheel { .. } => {
            let mut ts = ts.lock().unwrap();
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

fn spawn_timer(ts: Arc<Mutex<Instant>>, interval: u64) -> JoinHandle<()> {
    tokio::spawn(async move {
        loop {
            tokio::time::sleep(time::Duration::from_secs(1)).await;
            log::info!("=== interval: {}", interval);
            let mut ts = ts.lock().unwrap();
            let now = Instant::now();
            if now.duration_since(*ts) >= time::Duration::from_secs(interval) {
                keep_presence();
                *ts = Instant::now();
            }
        }
    })
}
