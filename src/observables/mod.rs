use crossbeam::channel;

pub struct Subscriber<T> {
    pub id: uuid::Uuid,
    pub tx: channel::Sender<T>,
}
