use crate::gameboy::Controller;

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
    let (ctrl, close_receiver, _, _, _) = Controller::new();

    ctrl.close();

    match close_receiver.recv_timeout(std::time::Duration::from_secs(5)) {
        Ok(msg) => Ok(()),
        Err(err) => Err(TestFailureError {
            description: err.to_string(),
        }),
    }
}

#[test]
fn pause() -> Result<(), TestFailureError> {
    let (mut ctrl, _, pause_receiver, _, _) = Controller::new();

    ctrl.pause();
    assert!(ctrl.paused);

    match pause_receiver.recv_timeout(std::time::Duration::from_secs(5)) {
        Ok(msg) => Ok(()),
        Err(err) => Err(TestFailureError {
            description: err.to_string(),
        }),
    }
}

#[test]
fn already_paused() -> Result<(), TestFailureError> {
    let (mut ctrl, _, pause_receiver, _, _) = Controller::new();
    ctrl.paused = true;

    ctrl.pause();
    assert!(ctrl.paused);

    match pause_receiver.recv_timeout(std::time::Duration::from_secs(1)) {
        Ok(_) => Err(TestFailureError {
            description: String::from(
                "received additional pause signal when controller was already paused",
            ),
        }),
        Err(_) => Ok(()),
    }
}
