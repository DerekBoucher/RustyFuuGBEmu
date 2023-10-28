use std::sync::mpsc::{self, Receiver, Sender};
use std::time::Duration;

/// Not to be confused with the Gameboy's player controller, this struct is a
/// mechanism for controlling the behaviour of the asynchronous gameboy thread.
pub struct Controller {
    close_sender: mpsc::Sender<()>,
    pause_sender: mpsc::SyncSender<()>,
    ack_receiver: mpsc::Receiver<()>,

    paused: bool,
}

impl Controller {
    pub fn new() -> (Self, Receiver<()>, Receiver<()>, Sender<()>) {
        let (close_sender, close_receiver) = mpsc::channel();
        let (pause_sender, pause_receiver) = mpsc::sync_channel(1);
        let (ack_sender, ack_receiver) = mpsc::channel();

        return (
            Self {
                close_sender: close_sender,
                pause_sender: pause_sender,
                ack_receiver: ack_receiver,

                paused: false,
            },
            close_receiver,
            pause_receiver,
            ack_sender,
        );
    }

    pub fn close(&self) {
        self.close_sender.send(()).unwrap();
    }

    pub fn join(&self) {
        match self.ack_receiver.recv_timeout(Duration::from_secs(5)) {
            Ok(_) => (),
            Err(err) => {
                log::error!("gb thread did not close in time: {:?}", err);
            }
        }
    }

    pub fn pause(&mut self) {
        if self.paused {
            log::debug!("gb emulation already paused");
            return;
        }

        self.paused = true;
        self.pause_sender.send(()).unwrap();
    }

    pub fn resume(&mut self) {
        if !self.paused {
            log::debug!("gb emulation already running");
            return;
        }

        self.paused = false;
        self.pause_sender.send(()).unwrap();
    }
}
