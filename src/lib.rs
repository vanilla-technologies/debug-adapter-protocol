use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, PartialEq, Serialize)]
struct ProtocolMessage {
    /// Sequence number (also known as message ID). For protocol messages of type
    /// 'request' this ID can be used to cancel the request.
    seq: u64,

    /// Message type.
    /// Values: 'request', 'response', 'event', etc.
    #[serde(flatten)]
    type_: ProtocolMessageType,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase", tag = "type")]
enum ProtocolMessageType {
    Request {
        /// The command to execute.
        #[serde(flatten)]
        command: Command,
    },
    Response {},
    Event {
        // Type of event.
        event: Event,
    },
}

/// Object containing arguments for the command.
#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase", tag = "command", content = "arguments")]
enum Command {
    Attach {
        /// Optional data from the previous, restarted session.
        /// The data is sent as the 'restart' attribute of the 'terminated' event.
        /// The client should leave the data intact.
        #[serde(skip_serializing_if = "Option::is_none")]
        __restart: Option<String>,
    },
    Initialize {
        /// The ID of the (frontend) client using this adapter.
        #[serde(skip_serializing_if = "Option::is_none")]
        clientID: Option<String>,

        /// The human readable name of the (frontend) client using this adapter.
        #[serde(skip_serializing_if = "Option::is_none")]
        clientName: Option<String>,

        /// The ID of the debug adapter.
        adapterID: String,

        /// The ISO-639 locale of the (frontend) client using this adapter, e.g. en-US
        /// or de-CH.
        #[serde(skip_serializing_if = "Option::is_none")]
        locale: Option<String>,

        /// If true all line numbers are 1-based (default).
        #[serde(skip_serializing_if = "Option::is_none")]
        linesStartAt1: Option<bool>,

        /// If true all column numbers are 1-based (default).
        #[serde(skip_serializing_if = "Option::is_none")]
        columnsStartAt1: Option<bool>,

        /// Determines in what format paths are specified. The default is 'path', which
        /// is the native format.
        /// Values: 'path', 'uri', etc.
        #[serde(skip_serializing_if = "Option::is_none")]
        pathFormat: Option<PathFormat>,

        /// Client supports the optional type attribute for variables.
        #[serde(skip_serializing_if = "Option::is_none")]
        supportsVariableType: Option<bool>,

        /// Client supports the paging of variables.
        #[serde(skip_serializing_if = "Option::is_none")]
        supportsVariablePaging: Option<bool>,

        /// Client supports the runInTerminal request.
        #[serde(skip_serializing_if = "Option::is_none")]
        supportsRunInTerminalRequest: Option<bool>,

        /// Client supports memory references.
        #[serde(skip_serializing_if = "Option::is_none")]
        supportsMemoryReferences: Option<bool>,

        /// Client supports progress reporting.
        #[serde(skip_serializing_if = "Option::is_none")]
        supportsProgressReporting: Option<bool>,

        /// Client supports the invalidated event.
        #[serde(skip_serializing_if = "Option::is_none")]
        supportsInvalidatedEvent: Option<bool>,
    },
}

/// Object containing arguments for the command.
#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
enum PathFormat {
    Path,
    URI,
}

/// Event-specific information.
#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[serde(tag = "event", content = "body")]
enum Event {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize() {
        // given:
        let json = r#"{"command":"initialize","arguments":{"clientID":"vscode","clientName":"Visual Studio Code","adapterID":"mock","pathFormat":"path","linesStartAt1":true,"columnsStartAt1":true,"supportsVariableType":true,"supportsVariablePaging":true,"supportsRunInTerminalRequest":true,"locale":"de","supportsProgressReporting":true,"supportsInvalidatedEvent":true},"type":"request","seq":1}"#;

        // when:
        let actual = serde_json::from_str::<ProtocolMessage>(&json).unwrap();

        // then:
        assert_eq!(
            actual,
            ProtocolMessage {
                seq: 1,
                type_: ProtocolMessageType::Request {
                    command: Command::Initialize {
                        clientID: Some("vscode".to_string()),
                        clientName: Some("Visual Studio Code".to_string()),
                        adapterID: "mock".to_string(),
                        locale: Some("de".to_string()),
                        linesStartAt1: Some(true),
                        columnsStartAt1: Some(true),
                        pathFormat: Some(PathFormat::Path),
                        supportsVariableType: Some(true),
                        supportsVariablePaging: Some(true),
                        supportsRunInTerminalRequest: Some(true),
                        supportsMemoryReferences: None,
                        supportsProgressReporting: Some(true),
                        supportsInvalidatedEvent: Some(true),
                    },
                },
            }
        );
    }

    #[test]
    fn test_serialize() {
        // when:
        let request = ProtocolMessage {
            seq: 1,
            type_: ProtocolMessageType::Request {
                command: Command::Initialize {
                    clientID: Some("vscode".to_string()),
                    clientName: Some("Visual Studio Code".to_string()),
                    adapterID: "mock".to_string(),
                    locale: Some("de".to_string()),
                    linesStartAt1: Some(true),
                    columnsStartAt1: Some(true),
                    pathFormat: Some(PathFormat::Path),
                    supportsVariableType: Some(true),
                    supportsVariablePaging: Some(true),
                    supportsRunInTerminalRequest: Some(true),
                    supportsMemoryReferences: None,
                    supportsProgressReporting: Some(true),
                    supportsInvalidatedEvent: Some(true),
                },
            },
        };

        let actual = serde_json::to_string(&request).unwrap();

        // then:
        assert_eq!(
            actual,
            r#"{"seq":1,"type":"request","command":"initialize","arguments":{"clientID":"vscode","clientName":"Visual Studio Code","adapterID":"mock","locale":"de","linesStartAt1":true,"columnsStartAt1":true,"pathFormat":"path","supportsVariableType":true,"supportsVariablePaging":true,"supportsRunInTerminalRequest":true,"supportsProgressReporting":true,"supportsInvalidatedEvent":true}}"#
        );
    }
}
