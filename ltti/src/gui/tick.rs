use std::error::Error;
use std::sync::mpsc;
use std::time::{Duration, Instant};

use crossterm::event::{Event::*, KeyEvent};

pub enum Event {
    Input(KeyEvent),
    Resize,
    Tick,
}

pub fn background_terminal_events_and_ticks(
    tick_rate: Duration,
    tx: mpsc::Sender<Event>,
) -> impl FnOnce() -> () {
    move || {
        let mut last_tick = Instant::now();

        loop {
            let timeout = tick_rate.saturating_sub(last_tick.elapsed());

            if let Some(event) = poll_terminal(timeout) {
                tx.send(event)
                    .expect("Can send terminal events to fg thread")
            }

            if last_tick.elapsed() >= tick_rate {
                tx.send(Event::Tick)
                    .expect("Can send tick events to fg thread");
                last_tick = Instant::now();
            }
        }
    }
}

fn poll_terminal(timeout: Duration) -> Option<Event> {
    use crossterm::event::{poll, read};

    if poll(timeout).expect("can poll terminal") {
        match read().expect("can read from terminal") {
            Key(key_event) => Some(Event::Input(key_event)),
            Resize(_, _) => Some(Event::Resize),
            Mouse(_) => None,
        }
    } else {
        None
    }
}
