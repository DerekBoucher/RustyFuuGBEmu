use crate::gameboy::Orchestrator;

#[derive(Debug, Clone)]
struct TestFailureError {
    description: String,
}

impl std::fmt::Display for TestFailureError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.description)
    }
}

#[test]
fn close() -> Result<(), TestFailureError> {
    let (ctrl, close_receiver, _, _, _) = Orchestrator::new();

    ctrl.close();

    match close_receiver.recv_timeout(std::time::Duration::from_secs(1)) {
        Ok(_) => Ok(()),
        Err(err) => Err(TestFailureError {
            description: err.to_string(),
        }),
    }
}

#[test]
fn pause() -> Result<(), TestFailureError> {
    let (mut ctrl, _, pause_receiver, _, _) = Orchestrator::new();

    ctrl.pause();
    assert!(ctrl.paused);

    match pause_receiver.recv_timeout(std::time::Duration::from_secs(1)) {
        Ok(_) => Ok(()),
        Err(err) => Err(TestFailureError {
            description: err.to_string(),
        }),
    }
}

#[test]
fn already_paused() -> Result<(), TestFailureError> {
    let (mut ctrl, _, pause_receiver, _, _) = Orchestrator::new();
    ctrl.paused = true;

    ctrl.pause();
    assert!(ctrl.paused);

    match pause_receiver.recv_timeout(std::time::Duration::from_secs(1)) {
        Ok(_) => Err(TestFailureError {
            description: String::from(
                "received additional pause signal when controller was already paused",
            ),
        }),
        Err(err) => match err {
            crossbeam::channel::RecvTimeoutError::Timeout => Ok(()),
            _ => Err(TestFailureError {
                description: err.to_string(),
            }),
        },
    }
}

#[test]
fn resume() -> Result<(), TestFailureError> {
    let (mut ctrl, _, pause_receiver, _, _) = Orchestrator::new();

    ctrl.resume();
    match pause_receiver.recv_timeout(std::time::Duration::from_secs(1)) {
        Ok(_) => Err(TestFailureError {
            description: String::from(
                "received a resume signal when the controller was not in a paused state",
            ),
        }),
        Err(err) => match err {
            crossbeam::channel::RecvTimeoutError::Timeout => Ok(()),
            _ => Err(TestFailureError {
                description: err.to_string(),
            }),
        },
    }
}

#[test]
fn resume_paused() -> Result<(), TestFailureError> {
    let (mut ctrl, _, pause_receiver, _, _) = Orchestrator::new();
    ctrl.paused = true;

    ctrl.resume();

    match pause_receiver.recv_timeout(std::time::Duration::from_secs(1)) {
        Ok(_) => Ok(()),
        Err(err) => Err(TestFailureError {
            description: err.to_string(),
        }),
    }
}

#[test]
fn load_rom() -> Result<(), TestFailureError> {
    let (ctrl, _, _, _, rom_receiver) = Orchestrator::new();

    let data: Vec<u8> = vec![0x01, 0x02, 0x03];

    ctrl.load_rom(data);

    match rom_receiver.recv_timeout(std::time::Duration::from_secs(1)) {
        Ok(recv_data) => {
            assert_eq!(recv_data, vec![0x01, 0x02, 0x03]);
            Ok(())
        }
        Err(err) => Err(TestFailureError {
            description: err.to_string(),
        }),
    }
}

#[test]
fn join() -> Result<(), TestFailureError> {
    let (ctrl, _, _, ack_sender, _) = Orchestrator::new();

    let send_result = match ack_sender.send(()) {
        Ok(()) => Ok(()),
        Err(err) => Err(TestFailureError {
            description: err.to_string(),
        }),
    };
    if send_result.is_err() {
        return send_result;
    }

    match ctrl.join() {
        Ok(_) => Ok(()),
        Err(err) => Err(TestFailureError {
            description: err.to_string(),
        }),
    }
}
