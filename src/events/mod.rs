use crossbeam::channel::{Receiver, Sender};
use std::collections::HashMap;

#[derive(Hash, Eq, PartialEq)]
pub enum Event {
    TimerDivWrite,
}

static mut BUS: Option<HashMap<Event, Vec<Sender<()>>>> = None;
static LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());

pub fn init() {
    unsafe {
        BUS = Some(HashMap::new());
    }
}

pub fn subscribe(event: Event) -> Receiver<()> {
    let _lock = LOCK.lock().unwrap();
    let (sender, receiver) = crossbeam::channel::unbounded();

    unsafe {
        BUS.as_mut()
            .unwrap()
            .entry(event)
            .or_insert(Vec::new())
            .push(sender);
    }

    receiver
}

pub fn notify(event: Event) {
    let _lock = LOCK.lock().unwrap();

    unsafe {
        if let Some(subscribers) = BUS.as_ref().unwrap().get(&event) {
            for subscriber in subscribers {
                match subscriber.send(()) {
                    Ok(_) => {}
                    Err(_) => {}
                }
            }
        }
    }
}
