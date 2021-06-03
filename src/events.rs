use serde::{Deserialize, Serialize};

/// Event-specific information.
#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase", tag = "event", content = "body")]
pub enum Event {
    #[serde(rename_all = "camelCase")]
    Exited {
        /// The exit code returned from the debuggee.
        exit_code: i32,
    },
    Initialized,
}
