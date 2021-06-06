use serde::{Deserialize, Serialize};

/// Event-specific information.
#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase", tag = "event", content = "body")]
pub enum Event {
    Exited(ExitedEventBody),
    Initialized,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExitedEventBody {
    /// The exit code returned from the debuggee.
    pub exit_code: i32,
}
