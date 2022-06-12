pub mod events;
pub mod requests;
pub mod responses;
pub mod types;

mod utils;

use events::Event;
use requests::Request;
use responses::Response;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

pub type SequenceNumber = u64;

/// Base class of requests, responses, and events.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct ProtocolMessage {
    /// Sequence number (also known as message ID). For protocol messages of type 'request' this ID can be used to cancel the request.
    pub seq: SequenceNumber,

    #[serde(flatten)]
    pub content: ProtocolMessageContent,
}

impl ProtocolMessage {
    pub fn new(seq: SequenceNumber, content: impl Into<ProtocolMessageContent>) -> ProtocolMessage {
        ProtocolMessage {
            seq,
            content: content.into(),
        }
    }
}

impl Display for ProtocolMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let json = serde_json::to_string(&self).unwrap();
        write!(f, "Content-Length: {}\r\n\r\n{}", json.len(), json)
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase", tag = "type")]
pub enum ProtocolMessageContent {
    /// A client or debug adapter initiated request.
    Request(Request),

    /// Response for a request.
    Response(Response),

    /// A debug adapter initiated event.
    Event(Event),
}

#[cfg(test)]
mod tests {
    use serde_json::{Map, Number, Value};

    use super::*;
    use crate::{events::*, requests::*, responses::*, types::*};
    use std::{collections::HashMap, iter::FromIterator};

    #[test]
    fn test_deserialize_request_initialize() {
        // given:
        let json = r#"{
            "command": "initialize",
            "arguments": {
                "clientID": "vscode",
                "clientName": "Visual Studio Code",
                "adapterID": "mock",
                "pathFormat": "path",
                "linesStartAt1": true,
                "columnsStartAt1": true,
                "supportsVariableType": true,
                "supportsVariablePaging": true,
                "supportsRunInTerminalRequest": true,
                "locale": "de",
                "supportsProgressReporting": true,
                "supportsInvalidatedEvent": true
            },
            "type": "request",
            "seq": 1
        }"#;

        // when:
        let actual = serde_json::from_str::<ProtocolMessage>(&json).unwrap();

        // then:
        assert_eq!(
            actual,
            ProtocolMessage {
                seq: 1,
                content: ProtocolMessageContent::Request(Request::Initialize(
                    InitializeRequestArguments::builder()
                        .client_id(Some("vscode".to_string()))
                        .client_name(Some("Visual Studio Code".to_string()))
                        .adapter_id("mock".to_string())
                        .locale(Some("de".to_string()))
                        .lines_start_at_1(true)
                        .columns_start_at_1(true)
                        .path_format(PathFormat::Path)
                        .supports_variable_type(true)
                        .supports_variable_paging(true)
                        .supports_run_in_terminal_request(true)
                        .supports_memory_references(false)
                        .supports_progress_reporting(true)
                        .supports_invalidated_event(true)
                        .build()
                ))
            }
        );
    }

    #[test]
    fn test_serialize_request_initialize() {
        // given:
        let under_test = ProtocolMessage {
            seq: 1,
            content: ProtocolMessageContent::Request(Request::Initialize(
                InitializeRequestArguments::builder()
                    .client_id(Some("vscode".to_string()))
                    .client_name(Some("Visual Studio Code".to_string()))
                    .adapter_id("mock".to_string())
                    .locale(Some("de".to_string()))
                    .lines_start_at_1(true)
                    .columns_start_at_1(true)
                    .path_format(PathFormat::Path)
                    .supports_variable_type(true)
                    .supports_variable_paging(true)
                    .supports_run_in_terminal_request(true)
                    .supports_memory_references(false)
                    .supports_progress_reporting(true)
                    .supports_invalidated_event(true)
                    .build(),
            )),
        };

        // when:
        let actual = serde_json::to_string_pretty(&under_test).unwrap();

        // then:
        assert_eq!(
            actual,
            r#"{
  "seq": 1,
  "type": "request",
  "command": "initialize",
  "arguments": {
    "clientID": "vscode",
    "clientName": "Visual Studio Code",
    "adapterID": "mock",
    "locale": "de",
    "linesStartAt1": true,
    "columnsStartAt1": true,
    "supportsVariableType": true,
    "supportsVariablePaging": true,
    "supportsRunInTerminalRequest": true,
    "supportsProgressReporting": true,
    "supportsInvalidatedEvent": true
  }
}"#
        );
    }

    #[test]
    fn test_deserialize_response_initialize() {
        // given:
        let json = r#"{
            "seq": 1,
            "type": "response",
            "request_seq": 1,
            "success": true,
            "command": "initialize",
            "body": {
                "supportsConfigurationDoneRequest": true,
                "supportsFunctionBreakpoints": true,
                "supportsConditionalBreakpoints": true,
                "supportsHitConditionalBreakpoints": true,
                "supportsDataBreakpoints": true,
                "supportsInstructionBreakpoints": true
            }
        }"#;

        // when:
        let actual = serde_json::from_str::<ProtocolMessage>(json).unwrap();

        // then:
        assert_eq!(
            actual,
            ProtocolMessage {
                seq: 1,
                content: ProtocolMessageContent::Response(Response {
                    request_seq: 1,
                    result: Ok(SuccessResponse::Initialize(Capabilities {
                        supports_configuration_done_request: true,
                        supports_function_breakpoints: true,
                        supports_conditional_breakpoints: true,
                        supports_hit_conditional_breakpoints: true,
                        supports_data_breakpoints: true,
                        supports_instruction_breakpoints: true,
                        ..Default::default()
                    })),
                })
            }
        )
    }

    #[test]
    fn test_serialize_response_initialize() {
        // given:
        let under_test = ProtocolMessage {
            seq: 1,
            content: ProtocolMessageContent::Response(Response {
                request_seq: 1,
                result: Ok(SuccessResponse::Initialize(Capabilities {
                    supports_configuration_done_request: true,
                    supports_function_breakpoints: true,
                    supports_conditional_breakpoints: true,
                    supports_hit_conditional_breakpoints: true,
                    supports_data_breakpoints: true,
                    supports_instruction_breakpoints: true,
                    ..Default::default()
                })),
            }),
        };

        // when:
        let actual = serde_json::to_string_pretty(&under_test).unwrap();

        // then:
        assert_eq!(
            actual,
            r#"{
  "seq": 1,
  "type": "response",
  "request_seq": 1,
  "success": true,
  "command": "initialize",
  "body": {
    "supportsConfigurationDoneRequest": true,
    "supportsFunctionBreakpoints": true,
    "supportsConditionalBreakpoints": true,
    "supportsHitConditionalBreakpoints": true,
    "supportsDataBreakpoints": true,
    "supportsInstructionBreakpoints": true
  }
}"#
        )
    }

    #[test]
    fn test_deserialize_response_error() {
        // given:
        let json = r#"{
            "seq": 1,
            "type": "response",
            "request_seq": 2,
            "success": false,
            "command": "initialize",
            "message": "Something went wrong",
            "body": {
                "error": {
                    "id": 3,
                    "format": "This thing went wrong"
                }
            }
        }"#;

        // when:
        let actual = serde_json::from_str::<ProtocolMessage>(json).unwrap();

        // then:
        assert_eq!(
            actual,
            ProtocolMessage {
                seq: 1,
                content: ProtocolMessageContent::Response(Response {
                    request_seq: 2,
                    result: Err(ErrorResponse {
                        command: "initialize".to_string(),
                        message: "Something went wrong".to_string(),
                        body: ErrorResponseBody {
                            error: Some(Message {
                                id: 3,
                                format: "This thing went wrong".to_string(),
                                variables: HashMap::new(),
                                send_telemetry: false,
                                show_user: false,
                                url: None,
                                url_label: None
                            })
                        }
                    }),
                })
            }
        )
    }

    #[test]
    fn test_serialize_response_error() {
        // given:
        let under_test = ProtocolMessage {
            seq: 1,
            content: ProtocolMessageContent::Response(Response {
                request_seq: 2,
                result: Err(ErrorResponse {
                    command: "initialize".to_string(),
                    message: "Something went wrong".to_string(),
                    body: ErrorResponseBody {
                        error: Some(Message {
                            id: 3,
                            format: "This thing went wrong".to_string(),
                            variables: HashMap::new(),
                            send_telemetry: false,
                            show_user: false,
                            url: None,
                            url_label: None,
                        }),
                    },
                }),
            }),
        };

        // when:
        let actual = serde_json::to_string_pretty(&under_test).unwrap();

        // then:
        assert_eq!(
            actual,
            r#"{
  "seq": 1,
  "type": "response",
  "request_seq": 2,
  "success": false,
  "command": "initialize",
  "message": "Something went wrong",
  "body": {
    "error": {
      "id": 3,
      "format": "This thing went wrong"
    }
  }
}"#
        )
    }

    #[test]
    fn test_deserialize_event_exited() {
        // given:
        let json = r#"{
            "seq": 1,
            "type": "event",
            "event": "exited",
            "body": {
                "exitCode": 0
            }
        }"#;

        // when:
        let actual = serde_json::from_str::<ProtocolMessage>(json).unwrap();

        // then:
        assert_eq!(
            actual,
            ProtocolMessage {
                seq: 1,
                content: ProtocolMessageContent::Event(Event::Exited(ExitedEventBody {
                    exit_code: 0
                }))
            }
        )
    }

    #[test]
    fn test_serialize_event_exited() {
        // given:
        let under_test = ProtocolMessage {
            seq: 1,
            content: ProtocolMessageContent::Event(Event::Exited(ExitedEventBody { exit_code: 0 })),
        };

        // when:
        let actual = serde_json::to_string_pretty(&under_test).unwrap();

        // then:
        assert_eq!(
            actual,
            r#"{
  "seq": 1,
  "type": "event",
  "event": "exited",
  "body": {
    "exitCode": 0
  }
}"#
        )
    }

    #[test]
    fn test_deserialize_request_launch_with_additional_attributes() {
        // given:
        let json = r#"{
            "command": "launch",
            "arguments": {
                "noDebug": true,
                "__restart": "Some Value",
                "bli": { "foo": "bar" },
                "bla": 1,
                "blub": true
            },
            "type": "request",
            "seq": 1
        }"#;

        // when:
        let actual = serde_json::from_str::<ProtocolMessage>(&json).unwrap();

        // then:
        assert_eq!(
            actual,
            ProtocolMessage {
                seq: 1,
                content: ProtocolMessageContent::Request(Request::Launch(
                    LaunchRequestArguments::builder()
                        .no_debug(true)
                        .restart(Some(Value::String("Some Value".to_string())))
                        .additional_attributes(Map::from_iter([
                            (
                                "bli".to_string(),
                                Value::Object(Map::from_iter([(
                                    "foo".to_string(),
                                    Value::String("bar".to_string())
                                )]))
                            ),
                            ("bla".to_string(), Value::Number(Number::from(1))),
                            ("blub".to_string(), Value::Bool(true))
                        ]))
                        .build()
                ))
            }
        );
    }

    #[test]
    fn test_serialize_request_launch_with_additional_attributes() {
        // given:
        let under_test = ProtocolMessage {
            seq: 1,
            content: ProtocolMessageContent::Request(Request::Launch(
                LaunchRequestArguments::builder()
                    .no_debug(true)
                    .restart(Some(Value::String("Some Value".to_string())))
                    .additional_attributes(Map::from_iter([
                        (
                            "bli".to_string(),
                            Value::Object(Map::from_iter([(
                                "foo".to_string(),
                                Value::String("bar".to_string()),
                            )])),
                        ),
                        ("bla".to_string(), Value::Number(Number::from(1))),
                        ("blub".to_string(), Value::Bool(true)),
                    ]))
                    .build(),
            )),
        };

        // when:
        let actual = serde_json::to_string_pretty(&under_test).unwrap();

        // then:
        assert_eq!(
            actual,
            r#"{
  "seq": 1,
  "type": "request",
  "command": "launch",
  "arguments": {
    "noDebug": true,
    "__restart": "Some Value",
    "bli": {
      "foo": "bar"
    },
    "bla": 1,
    "blub": true
  }
}"#
        );
    }

    #[test]
    fn test_deserialize_request_launch_without_additional_attributes() {
        // given:
        let json = r#"{
            "seq": 1,
            "type": "request",
            "command": "launch",
            "arguments": {}
        }"#;

        // when:
        let actual = serde_json::from_str::<ProtocolMessage>(&json).unwrap();

        // then:
        assert_eq!(
            actual,
            ProtocolMessage {
                seq: 1,
                content: ProtocolMessageContent::Request(Request::Launch(
                    LaunchRequestArguments::builder().build()
                ))
            }
        );
    }

    #[test]
    fn test_serialize_request_launch_without_additional_attributes() {
        // given:
        let under_test = ProtocolMessage {
            seq: 1,
            content: ProtocolMessageContent::Request(Request::Launch(
                LaunchRequestArguments::builder().build(),
            )),
        };

        // when:
        let actual = serde_json::to_string_pretty(&under_test).unwrap();

        // then:
        assert_eq!(
            actual,
            r#"{
  "seq": 1,
  "type": "request",
  "command": "launch",
  "arguments": {}
}"#
        );
    }
}
