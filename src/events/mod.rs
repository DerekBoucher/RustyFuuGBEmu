use crossbeam::channel::{Receiver, Sender};
use std::collections::HashMap;

#[derive(Hash, Eq, PartialEq)]
pub enum Event {
    TimerDivWrite,
}

static mut BUS: Option<HashMap<Event, Vec<Sender<()>>>> = None;

pub fn init() {
    unsafe {
        BUS = Some(HashMap::new());
    }
}

pub fn subscribe(event: Event) -> Receiver<()> {
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
    unsafe {
        if let Some(subscribers) = BUS.as_ref().unwrap().get(&event) {
            for subscriber in subscribers {
                subscriber.send(()).unwrap();
            }
        }
    }
}
