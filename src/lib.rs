use core::panic;
use serde::{
    de::{self, Unexpected},
    Deserialize, Deserializer, Serialize,
};
use serde_json::{Number, Value};

type SequenceNumber = u64;

#[derive(Debug, Deserialize, PartialEq)]
struct ProtocolMessage {
    /// Sequence number (also known as message ID). For protocol messages of type
    /// 'request' this ID can be used to cancel the request.
    seq: SequenceNumber,

    /// Message type.
    /// Values: 'request', 'response', 'event', etc.
    #[serde(flatten)]
    type_: ProtocolMessageType,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase", tag = "type")]
enum ProtocolMessageType {
    Request {
        /// The command to execute.
        #[serde(flatten)]
        request: Request,
    },
    Response {
        /// Sequence number of the corresponding request.
        request_seq: SequenceNumber,

        #[serde(flatten)]
        response_type: ResponseType,
    },
    Event {
        /// Type of event.
        event: Event,
    },
}

/// Object containing arguments for the command.
#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase", tag = "command", content = "arguments")]
enum Request {
    Attach {
        /// Optional data from the previous, restarted session.
        /// The data is sent as the 'restart' attribute of the 'terminated' event.
        /// The client should leave the data intact.
        #[serde(skip_serializing_if = "Option::is_none")]
        __restart: Option<String>,
    },
    #[serde(rename_all = "camelCase")]
    Initialize {
        /// The ID of the (frontend) client using this adapter.
        #[serde(skip_serializing_if = "Option::is_none", rename = "clientID")]
        client_id: Option<String>,

        /// The human readable name of the (frontend) client using this adapter.
        #[serde(skip_serializing_if = "Option::is_none")]
        client_name: Option<String>,

        /// The ID of the debug adapter.
        #[serde(rename = "adapterID")]
        adapter_id: String,

        /// The ISO-639 locale of the (frontend) client using this adapter, e.g. en-US
        /// or de-CH.
        #[serde(skip_serializing_if = "Option::is_none")]
        locale: Option<String>,

        /// If true all line numbers are 1-based (default).
        #[serde(default = "true_")]
        lines_start_at1: bool,

        /// If true all column numbers are 1-based (default).
        #[serde(default = "true_")]
        columns_start_at1: bool,

        /// Determines in what format paths are specified. The default is 'path', which
        /// is the native format.
        /// Values: 'path', 'uri', etc.
        #[serde(default)]
        path_format: PathFormat,

        /// Client supports the optional type attribute for variables.
        #[serde(default, skip_serializing_if = "eq_default")]
        supports_variable_type: bool,

        /// Client supports the paging of variables.
        #[serde(default, skip_serializing_if = "eq_default")]
        supports_variable_paging: bool,

        /// Client supports the runInTerminal request.
        #[serde(default, skip_serializing_if = "eq_default")]
        supports_run_in_terminal_request: bool,

        /// Client supports memory references.
        #[serde(default, skip_serializing_if = "eq_default")]
        supports_memory_references: bool,

        /// Client supports progress reporting.
        #[serde(default, skip_serializing_if = "eq_default")]
        supports_progress_reporting: bool,

        /// Client supports the invalidated event.
        #[serde(default, skip_serializing_if = "eq_default")]
        supports_invalidated_event: bool,
    },
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
enum PathFormat {
    Path,
    URI,
}

impl Default for PathFormat {
    fn default() -> Self {
        PathFormat::Path
    }
}

#[derive(Debug, PartialEq)]
enum ResponseType {
    Success {
        /// The command requested.
        command: Response,
    },
    Error {
        /// The command requested.
        command: String,

        /// Contains the raw error in short form if 'success' is false.
        /// This raw error might be interpreted by the frontend and is not shown in the
        /// UI.
        /// Some predefined values exist.
        /// Values:
        /// 'cancelled': request was cancelled.
        /// etc.
        message: String,
        // body: Option<Message>,
    },
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
struct ErrorResponse {
    command: String,
    message: String,
    // body: Option<Message>,
}

fn unexpected_value<'l>(value: &'l Value) -> Unexpected<'l> {
    match value {
        Value::Null => Unexpected::Other("null"),
        Value::Bool(b) => Unexpected::Bool(*b),
        Value::Number(n) => unexpected_number(n),
        Value::String(s) => Unexpected::Str(s),
        Value::Array(_) => Unexpected::Seq,
        Value::Object(_) => Unexpected::Map,
    }
}

fn unexpected_number(number: &Number) -> Unexpected<'static> {
    if number.is_f64() {
        return Unexpected::Float(number.as_f64().unwrap());
    }
    if number.is_u64() {
        return Unexpected::Unsigned(number.as_u64().unwrap());
    }
    if number.is_i64() {
        return Unexpected::Signed(number.as_i64().unwrap());
    }
    panic!("Unknown number {}", number)
}

impl<'de> Deserialize<'de> for ResponseType {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let value = Value::deserialize(d)?;

        let success = value
            .get("success")
            .ok_or_else(|| de::Error::missing_field("success"))?
            .as_bool()
            .ok_or_else(|| de::Error::invalid_type(unexpected_value(&value), &"success bool"))?;

        Ok(if success {
            let command =
                Response::deserialize(value).map_err(|e| de::Error::custom(e.to_string()))?;
            ResponseType::Success { command }
        } else {
            let response =
                ErrorResponse::deserialize(value).map_err(|e| de::Error::custom(e.to_string()))?;
            ResponseType::Error {
                command: response.command,
                message: response.message,
            }
        })
    }
}

/// Contains request result if success is true and optional error details if
/// success is false.
#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase", tag = "command", content = "body")]
enum Response {
    Initialize {
        #[serde(flatten)]
        capabilities: Capabilities,
    },
}

#[derive(Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
struct Capabilities {
    /// The debug adapter supports the 'configurationDone' request.
    #[serde(default, skip_serializing_if = "eq_default")]
    supports_configuration_done_request: bool,

    /// The debug adapter supports function breakpoints.
    #[serde(default, skip_serializing_if = "eq_default")]
    supports_function_breakpoints: bool,

    /// The debug adapter supports conditional breakpoints.
    #[serde(default, skip_serializing_if = "eq_default")]
    supports_conditional_breakpoints: bool,

    /// The debug adapter supports breakpoints that break execution after a
    /// specified number of hits.
    #[serde(default, skip_serializing_if = "eq_default")]
    supports_hit_conditional_breakpoints: bool,

    /// The debug adapter supports a (side effect free) evaluate request for data
    /// hovers.
    #[serde(default, skip_serializing_if = "eq_default")]
    supports_evaluate_for_hovers: bool,

    //   /// Available exception filter options for the 'setExceptionBreakpoints'
    //    /// request.
    //   exceptionBreakpointFilters?: ExceptionBreakpointsFilter[];
    /// The debug adapter supports stepping back via the 'stepBack' and
    /// 'reverseContinue' requests.
    #[serde(default, skip_serializing_if = "eq_default")]
    supports_step_back: bool,

    /// The debug adapter supports setting a variable to a value.
    #[serde(default, skip_serializing_if = "eq_default")]
    supports_set_variable: bool,

    /// The debug adapter supports restarting a frame.
    #[serde(default, skip_serializing_if = "eq_default")]
    supports_restart_frame: bool,

    /// The debug adapter supports the 'gotoTargets' request.
    #[serde(default, skip_serializing_if = "eq_default")]
    supports_goto_targets_request: bool,

    /// The debug adapter supports the 'stepInTargets' request.
    #[serde(default, skip_serializing_if = "eq_default")]
    supports_step_in_targets_request: bool,

    /// The debug adapter supports the 'completions' request.
    #[serde(default, skip_serializing_if = "eq_default")]
    supports_completions_request: bool,

    //   /// The set of characters that should trigger completion in a REPL. If not
    //    /// specified, the UI should assume the '.' character.
    //   completionTriggerCharacters?: string[];
    /// The debug adapter supports the 'modules' request.
    #[serde(default, skip_serializing_if = "eq_default")]
    supports_modules_requeststst: bool,

    //   /// The set of additional module information exposed by the debug adapter.
    //   additionalModuleColumns?: ColumnDescriptor[];

    //  /// Checksum algorithms supported by the debug adapter.
    //   supportedChecksumAlgorithms?: ChecksumAlgorithm[];
    /// The debug adapter supports the 'restart' request. In this case a client
    /// should not implement 'restart' by terminating and relaunching the adapter
    /// but by calling the RestartRequest.
    #[serde(default, skip_serializing_if = "eq_default")]
    supports_restart_request: bool,

    /// The debug adapter supports 'exceptionOptions' on the
    /// setExceptionBreakpoints request.
    #[serde(default, skip_serializing_if = "eq_default")]
    supports_exception_options: bool,

    /// The debug adapter supports a 'format' attribute on the stackTraceRequest,
    /// variablesRequest, and evaluateRequest.
    #[serde(default, skip_serializing_if = "eq_default")]
    supports_value_formatting_options: bool,

    /// The debug adapter supports the 'exceptionInfo' request.
    #[serde(default, skip_serializing_if = "eq_default")]
    supports_exception_info_request: bool,

    /// The debug adapter supports the 'terminateDebuggee' attribute on the
    /// 'disconnect' request.
    #[serde(default, skip_serializing_if = "eq_default")]
    support_terminate_debuggee: bool,

    /// The debug adapter supports the 'suspendDebuggee' attribute on the
    /// 'disconnect' request.
    #[serde(default, skip_serializing_if = "eq_default")]
    support_suspend_debuggee: bool,

    /// The debug adapter supports the delayed loading of parts of the stack, which
    /// requires that both the 'startFrame' and 'levels' arguments and an optional
    /// 'totalFrames' result of the 'StackTrace' request are supported.
    #[serde(default, skip_serializing_if = "eq_default")]
    supports_delayed_stack_trace_loading: bool,

    /// The debug adapter supports the 'loadedSources' request.
    #[serde(default, skip_serializing_if = "eq_default")]
    supports_loaded_sources_request: bool,

    /// The debug adapter supports logpoints by interpreting the 'logMessage'
    /// attribute of the SourceBreakpoint.
    #[serde(default, skip_serializing_if = "eq_default")]
    supports_log_points: bool,

    /// The debug adapter supports the 'terminateThreads' request.
    #[serde(default, skip_serializing_if = "eq_default")]
    supports_terminate_threads_request: bool,

    /// The debug adapter supports the 'setExpression' request.
    #[serde(default, skip_serializing_if = "eq_default")]
    supports_set_expression: bool,

    /// The debug adapter supports the 'terminate' request.
    #[serde(default, skip_serializing_if = "eq_default")]
    supports_terminate_request: bool,

    /// The debug adapter supports data breakpoints.
    #[serde(default, skip_serializing_if = "eq_default")]
    supports_data_breakpoints: bool,

    /// The debug adapter supports the 'readMemory' request.
    #[serde(default, skip_serializing_if = "eq_default")]
    supports_read_memory_request: bool,

    /// The debug adapter supports the 'disassemble' request.
    #[serde(default, skip_serializing_if = "eq_default")]
    supports_disassemble_request: bool,

    /// The debug adapter supports the 'cancel' request.
    #[serde(default, skip_serializing_if = "eq_default")]
    supports_cancel_request: bool,

    /// The debug adapter supports the 'breakpointLocations' request.
    #[serde(default, skip_serializing_if = "eq_default")]
    supports_breakpoint_locations_request: bool,

    /// The debug adapter supports the 'clipboard' context value in the 'evaluate'
    /// request.
    #[serde(default, skip_serializing_if = "eq_default")]
    supports_clipboard_context: bool,

    /// The debug adapter supports stepping granularities (argument 'granularity')
    /// for the stepping requests.
    #[serde(default, skip_serializing_if = "eq_default")]
    supports_stepping_granularity: bool,

    /// The debug adapter supports adding breakpoints based on instruction
    /// references.
    #[serde(default, skip_serializing_if = "eq_default")]
    supports_instruction_breakpoints: bool,

    /// The debug adapter supports 'filterOptions' as an argument on the
    /// 'setExceptionBreakpoints' request.
    #[serde(default, skip_serializing_if = "eq_default")]
    supports_exception_filter_options: bool,
}

/// Event-specific information.
#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase", tag = "event", content = "body")]
enum Event {}

fn true_() -> bool {
    true
}

fn eq_default<T: Default + PartialEq>(t: &T) -> bool {
    t.eq(&Default::default())
}

#[cfg(test)]
mod tests {
    use super::*;

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
                type_: ProtocolMessageType::Request {
                    request: Request::Initialize {
                        client_id: Some("vscode".to_string()),
                        client_name: Some("Visual Studio Code".to_string()),
                        adapter_id: "mock".to_string(),
                        locale: Some("de".to_string()),
                        lines_start_at1: true,
                        columns_start_at1: true,
                        path_format: PathFormat::Path,
                        supports_variable_type: true,
                        supports_variable_paging: true,
                        supports_run_in_terminal_request: true,
                        supports_memory_references: false,
                        supports_progress_reporting: true,
                        supports_invalidated_event: true,
                    },
                },
            }
        );
    }

    #[test]
    fn test_deserialize_response_initialize() {
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

        let actual = serde_json::from_str::<ProtocolMessage>(json).unwrap();

        assert_eq!(
            actual,
            ProtocolMessage {
                seq: 1,
                type_: ProtocolMessageType::Response {
                    request_seq: 1,
                    response_type: ResponseType::Success {
                        command: Response::Initialize {
                            capabilities: Capabilities {
                                supports_configuration_done_request: true,
                                supports_function_breakpoints: true,
                                supports_conditional_breakpoints: true,
                                supports_hit_conditional_breakpoints: true,
                                supports_data_breakpoints: true,
                                supports_instruction_breakpoints: true,
                                ..Default::default()
                            }
                        }
                    },
                }
            }
        )
    }

    // #[test]
    // fn test_serialize() {
    //     // when:
    //     let request = ProtocolMessage {
    //         seq: 1,
    //         type_: ProtocolMessageType::Request {
    //             request: Request::Initialize {
    //                 clientID: Some("vscode".to_string()),
    //                 clientName: Some("Visual Studio Code".to_string()),
    //                 adapterID: "mock".to_string(),
    //                 locale: Some("de".to_string()),
    //                 linesStartAt1: true,
    //                 columnsStartAt1: true,
    //                 pathFormat: PathFormat::Path,
    //                 supportsVariableType: true,
    //                 supportsVariablePaging: true,
    //                 supportsRunInTerminalRequest: true,
    //                 supportsMemoryReferences: false,
    //                 supportsProgressReporting: true,
    //                 supportsInvalidatedEvent: true,
    //             },
    //         },
    //     };

    //     let actual = serde_json::to_string(&request).unwrap();

    //     // then:
    //     assert_eq!(
    //         actual,
    //         r#"{"seq":1,"type":"request","command":"initialize","arguments":{"clientID":"vscode","clientName":"Visual Studio Code","adapterID":"mock","locale":"de","linesStartAt1":true,"columnsStartAt1":true,"pathFormat":"path","supportsVariableType":true,"supportsVariablePaging":true,"supportsRunInTerminalRequest":true,"supportsProgressReporting":true,"supportsInvalidatedEvent":true}}"#
    //     );
    // }
}
