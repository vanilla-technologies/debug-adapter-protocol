use crate::{
    types::{
        Breakpoint, BreakpointLocation, Capabilities, CompletionItem, DataBreakpointAccessType,
        DisassembledInstruction, ExceptionBreakMode, ExceptionDetails, GotoTarget, Message, Module,
        Scope, Source, StackFrame, StepInTarget, Thread, Variable, VariablePresentationHint,
    },
    utils::{eq_default, true_},
    SequenceNumber,
};
use serde::{
    de::{Error, Unexpected},
    Deserialize, Deserializer, Serialize, Serializer,
};
use serde_json::{Number, Value};

/// Response for a request.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Response {
    /// Sequence number of the corresponding request.
    pub request_seq: SequenceNumber,

    #[serde(
        flatten,
        deserialize_with = "deserialize_response_result",
        serialize_with = "serialize_response_result"
    )]
    pub result: Result<SuccessResponse, ErrorResponse>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ErrorResponse {
    /// The command requested.
    pub command: String,

    /// Contains the raw error in short form if 'success' is false.
    /// This raw error might be interpreted by the frontend and is not shown in the
    /// UI.
    /// Some predefined values exist.
    /// Values:
    /// 'cancelled': request was cancelled.
    /// etc.
    pub message: String,

    pub body: ErrorResponseBody,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ErrorResponseBody {
    /// An optional, structured error message.
    pub error: Option<Message>,
}

/// Contains request result if success is true and optional error details if success is false.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase", tag = "command", content = "body")]
pub enum SuccessResponse {
    /// Response to 'attach' request. This is just an acknowledgement, so no body field is required.
    Attach,

    /// Response to 'breakpointLocations' request.
    ///
    /// Contains possible locations for source breakpoints.
    BreakpointLocations(BreakpointLocationsResponseBody),

    /// Response to 'cancel' request. This is just an acknowledgement, so no body field is required.
    Cancel,

    /// Response to 'completions' request.
    Completions(CompletionsResponseBody),

    /// Response to 'configurationDone' request. This is just an acknowledgement, so no body field is required.
    ConfigurationDone,

    /// Response to 'continue' request.
    Continue(ContinueResponseBody),

    /// Response to 'dataBreakpointInfo' request.
    DataBreakpointInfo(DataBreakpointInfoResponseBody),

    /// Response to 'disassemble' request.
    Disassemble(DisassembleResponseBody),

    /// Response to 'disconnect' request. This is just an acknowledgement, so no body field is required.
    Disconnect,

    /// Response to 'evaluate' request.
    Evaluate(EvaluateResponseBody),

    /// Response to 'exceptionInfo' request.
    ExceptionInfo(ExceptionInfoResponseBody),

    /// Response to 'goto' request. This is just an acknowledgement, so no body field is required.
    Goto,

    /// Response to 'gotoTargets' request.
    GotoTargets(GotoTargetsResponseBody),

    /// Response to 'initialize' request.
    Initialize(Capabilities),

    /// Response to 'launch' request. This is just an acknowledgement, so no body field is required.
    Launch,

    /// Response to 'loadedSources' request.
    LoadedSources(LoadedSourcesResponseBody),

    /// Response to 'modules' request.
    Modules(ModulesResponseBody),

    /// Response to 'next' request. This is just an acknowledgement, so no body field is required.
    Next,

    /// Response to 'pause' request. This is just an acknowledgement, so no body field is required.
    Pause,

    /// Response to 'readMemory' request.
    ReadMemory(ReadMemoryResponseBody),

    /// Response to 'restartFrame' request. This is just an acknowledgement, so no body field is required.
    RestartFrame,

    /// Response to 'restart' request. This is just an acknowledgement, so no body field is required.
    Restart,

    /// Response to 'reverseContinue' request. This is just an acknowledgement, so no body field is required.
    ReverseContinue,

    /// Response to 'runInTerminal' request.
    RunInTerminal(RunInTerminalResponseBody),

    /// Response to 'scopes' request.
    Scopes(ScopesResponseBody),

    /// Response to 'setBreakpoints' request.
    ///
    /// Returned is information about each breakpoint created by this request.
    ///
    /// This includes the actual code location and whether the breakpoint could be verified.
    ///
    /// The breakpoints returned are in the same order as the elements of the 'breakpoints'
    ///
    /// (or the deprecated 'lines') array in the arguments.
    SetBreakpoints(SetBreakpointsResponseBody),

    /// Response to 'setDataBreakpoints' request.
    ///
    /// Returned is information about each breakpoint created by this request.
    SetDataBreakpoints(SetDataBreakpointsResponseBody),

    /// Response to 'setExceptionBreakpoints' request.
    ///
    /// The response contains an array of Breakpoint objects with information about each exception breakpoint or filter. The Breakpoint objects are in the same order as the elements of the 'filters', 'filterOptions', 'exceptionOptions' arrays given as arguments. If both 'filters' and 'filterOptions' are given, the returned array must start with 'filters' information first, followed by 'filterOptions' information.
    ///
    /// The mandatory 'verified' property of a Breakpoint object signals whether the exception breakpoint or filter could be successfully created and whether the optional condition or hit count expressions are valid. In case of an error the 'message' property explains the problem. An optional 'id' property can be used to introduce a unique ID for the exception breakpoint or filter so that it can be updated subsequently by sending breakpoint events.
    ///
    /// For backward compatibility both the 'breakpoints' array and the enclosing 'body' are optional. If these elements are missing a client will not be able to show problems for individual exception breakpoints or filters.
    SetExceptionBreakpoints(SetExceptionBreakpointsResponseBody),

    /// Response to 'setExpression' request.
    SetExpression(SetExpressionResponseBody),

    /// Response to 'setFunctionBreakpoints' request.
    ///
    /// Returned is information about each breakpoint created by this request.
    SetFunctionBreakpoints(SetFunctionBreakpointsResponseBody),

    /// Response to 'setInstructionBreakpoints' request
    SetInstructionBreakpoints(SetInstructionBreakpointsResponseBody),

    /// Response to 'setVariable' request.
    SetVariable(SetVariableResponseBody),

    /// Response to 'source' request.
    Source(SourceResponseBody),

    /// Response to 'stackTrace' request.
    StackTrace(StackTraceResponseBody),

    /// Response to 'stepBack' request. This is just an acknowledgement, so no body field is required.
    StepBack,

    /// Response to 'stepIn' request. This is just an acknowledgement, so no body field is required.
    StepIn,

    /// Response to 'stepInTargets' request.
    StepInTargets(StepInTargetsResponseBody),

    /// Response to 'stepOut' request. This is just an acknowledgement, so no body field is required.
    StepOut,

    /// Response to 'terminate' request. This is just an acknowledgement, so no body field is required.
    Terminate,

    /// Response to 'terminateThreads' request. This is just an acknowledgement, so no body field is required.
    TerminateThreads,

    /// Response to 'threads' request.
    Threads(ThreadsResponseBody),

    /// Response to 'variables' request.
    Variables(VariablesResponseBody),
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct BreakpointLocationsResponseBody {
    /// Sorted set of possible breakpoint locations.
    #[serde(rename = "breakpoints")]
    pub breakpoints: Vec<BreakpointLocation>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct CompletionsResponseBody {
    /// The possible completions for .
    #[serde(rename = "targets")]
    pub targets: Vec<CompletionItem>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ContinueResponseBody {
    /// If true, the 'continue' request has ignored the specified thread and continued all threads instead.
    ///
    /// If this attribute is missing a value of 'true' is assumed for backward compatibility.
    #[serde(rename = "allThreadsContinued", default = "true_")]
    pub all_threads_continued: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct DataBreakpointInfoResponseBody {
    /// An identifier for the data on which a data breakpoint can be registered with the setDataBreakpoints request or null if no data breakpoint is available.
    #[serde(rename = "dataId")]
    pub data_id: Option<String>,

    /// UI string that describes on what data the breakpoint is set on or why a data breakpoint is not available.
    #[serde(rename = "description")]
    pub description: String,

    /// Optional attribute listing the available access types for a potential data breakpoint. A UI frontend could surface this information.
    #[serde(rename = "accessTypes", skip_serializing_if = "Option::is_none")]
    pub access_types: Option<Vec<DataBreakpointAccessType>>,

    /// Optional attribute indicating that a potential data breakpoint could be persisted across sessions.
    #[serde(rename = "canPersist", default, skip_serializing_if = "eq_default")]
    pub can_persist: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct DisassembleResponseBody {
    /// The list of disassembled instructions.
    #[serde(rename = "instructions")]
    pub instructions: Vec<DisassembledInstruction>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct EvaluateResponseBody {
    /// The result of the evaluate request.
    #[serde(rename = "result")]
    pub result: String,

    /// The optional type of the evaluate result.
    ///
    /// This attribute should only be returned by a debug adapter if the client has passed the value true for the 'supportsVariableType' capability of the 'initialize' request.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,

    /// Properties of a evaluate result that can be used to determine how to render the result in the UI.
    #[serde(rename = "presentationHint", skip_serializing_if = "Option::is_none")]
    pub presentation_hint: Option<VariablePresentationHint>,

    /// If variablesReference is > 0, the evaluate result is structured and its children can be retrieved by passing variablesReference to the VariablesRequest.
    ///
    /// The value should be less than or equal to 2147483647 (2^31-1).
    #[serde(rename = "variablesReference")]
    pub variables_reference: i32,

    /// The number of named child variables.
    ///
    /// The client can use this optional information to present the variables in a paged UI and fetch them in chunks.
    ///
    /// The value should be less than or equal to 2147483647 (2^31-1).
    #[serde(rename = "namedVariables", skip_serializing_if = "Option::is_none")]
    pub named_variables: Option<i32>,

    /// The number of indexed child variables.
    ///
    /// The client can use this optional information to present the variables in a paged UI and fetch them in chunks.
    ///
    /// The value should be less than or equal to 2147483647 (2^31-1).
    #[serde(rename = "indexedVariables", skip_serializing_if = "Option::is_none")]
    pub indexed_variables: Option<i32>,

    /// Optional memory reference to a location appropriate for this result.
    ///
    /// For pointer type eval results, this is generally a reference to the memory address contained in the pointer.
    ///
    /// This attribute should be returned by a debug adapter if the client has passed the value true for the 'supportsMemoryReferences' capability of the 'initialize' request.
    #[serde(rename = "memoryReference", skip_serializing_if = "Option::is_none")]
    pub memory_reference: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ExceptionInfoResponseBody {
    /// ID of the exception that was thrown.
    #[serde(rename = "exceptionId")]
    pub exception_id: String,

    /// Descriptive text for the exception provided by the debug adapter.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Mode that caused the exception notification to be raised.
    #[serde(rename = "breakMode")]
    pub break_mode: ExceptionBreakMode,

    /// Detailed information about the exception.
    #[serde(rename = "details", skip_serializing_if = "Option::is_none")]
    pub details: Option<ExceptionDetails>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct GotoTargetsResponseBody {
    /// The possible goto targets of the specified location.
    #[serde(rename = "targets")]
    pub targets: Vec<GotoTarget>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct LoadedSourcesResponseBody {
    /// Set of loaded sources.
    #[serde(rename = "sources")]
    pub sources: Vec<Source>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ModulesResponseBody {
    /// All modules or range of modules.
    #[serde(rename = "modules")]
    pub modules: Vec<Module>,

    /// The total number of modules available.
    #[serde(rename = "totalModules", skip_serializing_if = "Option::is_none")]
    pub total_modules: Option<i32>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ReadMemoryResponseBody {
    /// The address of the first byte of data returned.
    ///
    /// Treated as a hex value if prefixed with '0x', or as a decimal value otherwise.
    #[serde(rename = "address")]
    pub address: String,

    /// The number of unreadable bytes encountered after the last successfully read byte.
    ///
    /// This can be used to determine the number of bytes that must be skipped before a subsequent 'readMemory' request will succeed.
    #[serde(rename = "unreadableBytes", skip_serializing_if = "Option::is_none")]
    pub unreadable_bytes: Option<i32>,

    /// The bytes read from memory, encoded using base64.
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct RunInTerminalResponseBody {
    /// The process ID. The value should be less than or equal to 2147483647 (2^31-1).
    #[serde(rename = "processId", skip_serializing_if = "Option::is_none")]
    pub process_id: Option<i32>,

    /// The process ID of the terminal shell. The value should be less than or equal to 2147483647 (2^31-1).
    #[serde(rename = "shellProcessId", skip_serializing_if = "Option::is_none")]
    pub shell_process_id: Option<i32>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ScopesResponseBody {
    /// The scopes of the stackframe. If the array has length zero, there are no scopes available.
    #[serde(rename = "scopes")]
    pub scopes: Vec<Scope>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct SetBreakpointsResponseBody {
    /// Information about the breakpoints.
    ///
    /// The array elements are in the same order as the elements of the 'breakpoints' (or the deprecated 'lines') array in the arguments.
    #[serde(rename = "breakpoints")]
    pub breakpoints: Vec<Breakpoint>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct SetDataBreakpointsResponseBody {
    /// Information about the data breakpoints. The array elements correspond to the elements of the input argument 'breakpoints' array.
    #[serde(rename = "breakpoints")]
    pub breakpoints: Vec<Breakpoint>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct SetExceptionBreakpointsResponseBody {
    /// Information about the exception breakpoints or filters.
    ///
    /// The breakpoints returned are in the same order as the elements of the 'filters', 'filterOptions', 'exceptionOptions' arrays in the arguments. If both 'filters' and 'filterOptions' are given, the returned array must start with 'filters' information first, followed by 'filterOptions' information.
    #[serde(rename = "breakpoints", skip_serializing_if = "Option::is_none")]
    pub breakpoints: Option<Vec<Breakpoint>>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct SetExpressionResponseBody {
    /// The new value of the expression.
    #[serde(rename = "value")]
    pub value: String,

    /// The optional type of the value.
    ///
    /// This attribute should only be returned by a debug adapter if the client has passed the value true for the 'supportsVariableType' capability of the 'initialize' request.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,

    /// Properties of a value that can be used to determine how to render the result in the UI.
    #[serde(rename = "presentationHint", skip_serializing_if = "Option::is_none")]
    pub presentation_hint: Option<VariablePresentationHint>,

    /// If variablesReference is > 0, the value is structured and its children can be retrieved by passing variablesReference to the VariablesRequest.
    ///
    /// The value should be less than or equal to 2147483647 (2^31-1).
    #[serde(rename = "variablesReference", skip_serializing_if = "Option::is_none")]
    pub variables_reference: Option<i32>,

    /// The number of named child variables.
    ///
    /// The client can use this optional information to present the variables in a paged UI and fetch them in chunks.
    ///
    /// The value should be less than or equal to 2147483647 (2^31-1).
    #[serde(rename = "namedVariables", skip_serializing_if = "Option::is_none")]
    pub named_variables: Option<i32>,

    /// The number of indexed child variables.
    ///
    /// The client can use this optional information to present the variables in a paged UI and fetch them in chunks.
    ///
    /// The value should be less than or equal to 2147483647 (2^31-1).
    #[serde(rename = "indexedVariables", skip_serializing_if = "Option::is_none")]
    pub indexed_variables: Option<i32>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct SetFunctionBreakpointsResponseBody {
    /// Information about the breakpoints. The array elements correspond to the elements of the 'breakpoints' array.
    #[serde(rename = "breakpoints")]
    pub breakpoints: Vec<Breakpoint>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct SetInstructionBreakpointsResponseBody {
    /// Information about the breakpoints. The array elements correspond to the elements of the 'breakpoints' array.
    #[serde(rename = "breakpoints")]
    pub breakpoints: Vec<Breakpoint>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct SetVariableResponseBody {
    /// The new value of the variable.
    #[serde(rename = "value")]
    pub value: String,

    /// The type of the new value. Typically shown in the UI when hovering over the value.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,

    /// If variablesReference is > 0, the new value is structured and its children can be retrieved by passing variablesReference to the VariablesRequest.
    ///
    /// The value should be less than or equal to 2147483647 (2^31-1).
    #[serde(rename = "variablesReference", skip_serializing_if = "Option::is_none")]
    pub variables_reference: Option<i32>,

    /// The number of named child variables.
    ///
    /// The client can use this optional information to present the variables in a paged UI and fetch them in chunks.
    ///
    /// The value should be less than or equal to 2147483647 (2^31-1).
    #[serde(rename = "namedVariables", skip_serializing_if = "Option::is_none")]
    pub named_variables: Option<i32>,

    /// The number of indexed child variables.
    ///
    /// The client can use this optional information to present the variables in a paged UI and fetch them in chunks.
    ///
    /// The value should be less than or equal to 2147483647 (2^31-1).
    #[serde(rename = "indexedVariables", skip_serializing_if = "Option::is_none")]
    pub indexed_variables: Option<i32>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct SourceResponseBody {
    /// Content of the source reference.
    #[serde(rename = "content")]
    pub content: String,

    /// Optional content type (mime type) of the source.
    #[serde(rename = "mimeType", skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct StackTraceResponseBody {
    /// The frames of the stackframe. If the array has length zero, there are no stackframes available.
    ///
    /// This means that there is no location information available.
    #[serde(rename = "stackFrames")]
    pub stack_frames: Vec<StackFrame>,

    /// The total number of frames available in the stack. If omitted or if totalFrames is larger than the available frames, a client is expected to request frames until a request returns less frames than requested (which indicates the end of the stack). Returning monotonically increasing totalFrames values for subsequent requests can be used to enforce paging in the client.
    #[serde(rename = "totalFrames", skip_serializing_if = "Option::is_none")]
    pub total_frames: Option<i32>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct StepInTargetsResponseBody {
    /// The possible stepIn targets of the specified source location.
    #[serde(rename = "targets")]
    pub targets: Vec<StepInTarget>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ThreadsResponseBody {
    /// All threads.
    #[serde(rename = "threads")]
    pub threads: Vec<Thread>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct VariablesResponseBody {
    /// All (or a range) of variables for the given variable reference.
    #[serde(rename = "variables")]
    pub variables: Vec<Variable>,
}

// Workaround from https://stackoverflow.com/a/65576570
// for https://github.com/serde-rs/serde/issues/745

fn deserialize_response_result<'de, D>(
    deserializer: D,
) -> Result<Result<SuccessResponse, ErrorResponse>, D::Error>
where
    D: Deserializer<'de>,
{
    let value = Value::deserialize(deserializer)?;

    let success = value
        .get("success")
        .ok_or_else(|| Error::missing_field("success"))?
        .as_bool()
        .ok_or_else(|| Error::invalid_type(unexpected_value(&value), &"success bool"))?;

    Ok(if success {
        Ok(Deserialize::deserialize(value).map_err(|e| Error::custom(e.to_string()))?)
    } else {
        Err(Deserialize::deserialize(value).map_err(|e| Error::custom(e.to_string()))?)
    })
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

fn serialize_response_result<S>(
    result: &Result<SuccessResponse, ErrorResponse>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    #[derive(Clone, Serialize)]
    #[serde(untagged)]
    enum Content<'l> {
        Success(&'l SuccessResponse),
        Error(&'l ErrorResponse),
    }

    #[derive(Clone, Serialize)]
    struct TaggedContent<'l> {
        success: bool,
        #[serde(flatten)]
        content: Content<'l>,
    }

    let serializable = match result {
        Ok(response) => TaggedContent {
            success: true,
            content: Content::Success(response),
        },
        Err(response) => TaggedContent {
            success: false,
            content: Content::Error(response),
        },
    };
    serializable.serialize(serializer)
}
