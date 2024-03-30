use std::sync::{Arc, Weak};

use crate::cpu::register;

pub enum Event {
    RegisterWritten(register::ID),
    RegisterRead(register::ID),
    MemoryWritten(u16),
    MemoryRead(u16),
}

pub trait Observer<T>: Send + Sync {
    fn notify(&mut self, event: &T);
}

impl<T, F: FnMut(&T) + Send + Sync> Observer<T> for F {
    fn notify(&mut self, event: &T) {
        self(&event);
    }
}

pub trait Observable {
    fn subscribe(&self, observer: Arc<dyn Observer<Event>>);
    fn unsubscribe(&self, observer: &Weak<dyn Observer<Event>>);
}
