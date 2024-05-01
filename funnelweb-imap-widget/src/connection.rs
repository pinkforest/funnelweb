use std::time::SystemTime;

#[derive(Debug)]
pub enum ImapConnectionState {
    RetryingIn(SystemTime),
    Connecting(SystemTime),
    Connected(SystemTime),
    LoggingIn(SystemTime),
    LoggedIn(SystemTime),
    GoingIdle(SystemTime),
    Idling(SystemTime),
    LoggingOut(SystemTime),
    Disconnecting(SystemTime),
    Disconnected(SystemTime),
}

#[derive(Debug)]
pub struct ImapConnection {
    pub name: String,
    pub state: Option<ImapConnectionState>,
}

impl ImapConnection {
    pub fn new(name: String) -> Self {
        Self { name, state: None }
    }
}
