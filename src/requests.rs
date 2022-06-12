use crate::{
    types::{
        DataBreakpoint, ExceptionFilterOptions, ExceptionOptions, FunctionBreakpoint,
        InstructionBreakpoint, Source, SourceBreakpoint, StackFrameFormat, SteppingGranularity,
        ValueFormat,
    },
    utils::{eq_default, true_},
    ProtocolMessageContent,
};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use std::collections::HashMap;
use typed_builder::TypedBuilder;

/// A client or debug adapter initiated request.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase", tag = "command", content = "arguments")]
pub enum Request {
    /// The attach request is sent from the client to the debug adapter to attach to a debuggee that is already running.
    ///
    /// Since attaching is debugger/runtime specific, the arguments for this request are not part of this specification.
    Attach(AttachRequestArguments),

    /// The 'breakpointLocations' request returns all possible locations for source breakpoints in a given range.
    ///
    /// Clients should only call this request if the capability 'supportsBreakpointLocationsRequest' is true.
    BreakpointLocations(BreakpointLocationsRequestArguments),

    /// The 'cancel' request is used by the frontend in two situations:
    ///
    /// - to indicate that it is no longer interested in the result produced by a specific request issued earlier
    ///
    /// - to cancel a progress sequence. Clients should only call this request if the capability 'supportsCancelRequest' is true.
    ///
    /// This request has a hint characteristic: a debug adapter can only be expected to make a 'best effort' in honouring this request but there are no guarantees.
    ///
    /// The 'cancel' request may return an error if it could not cancel an operation but a frontend should refrain from presenting this error to end users.
    ///
    /// A frontend client should only call this request if the capability 'supportsCancelRequest' is true.
    ///
    /// The request that got canceled still needs to send a response back. This can either be a normal result ('success' attribute true)
    ///
    /// or an error response ('success' attribute false and the 'message' set to 'cancelled').
    ///
    /// Returning partial results from a cancelled request is possible but please note that a frontend client has no generic way for detecting that a response is partial or not.
    ///
    ///  The progress that got cancelled still needs to send a 'progressEnd' event back.
    ///
    ///  A client should not assume that progress just got cancelled after sending the 'cancel' request.
    Cancel(CancelRequestArguments),

    /// Returns a list of possible completions for a given caret position and text.
    ///
    /// Clients should only call this request if the capability 'supportsCompletionsRequest' is true.
    Completions(CompletionsRequestArguments),

    /// This optional request indicates that the client has finished initialization of the debug adapter.
    ///
    /// So it is the last request in the sequence of configuration requests (which was started by the 'initialized' event).
    ///
    /// Clients should only call this request if the capability 'supportsConfigurationDoneRequest' is true.
    ConfigurationDone,

    /// The request starts the debuggee to run again.
    Continue(ContinueRequestArguments),

    /// Obtains information on a possible data breakpoint that could be set on an expression or variable.
    ///
    /// Clients should only call this request if the capability 'supportsDataBreakpoints' is true.
    DataBreakpointInfo(DataBreakpointInfoRequestArguments),

    /// Disassembles code stored at the provided location.
    ///
    /// Clients should only call this request if the capability 'supportsDisassembleRequest' is true.
    Disassemble(DisassembleRequestArguments),

    /// The 'disconnect' request is sent from the client to the debug adapter in order to stop debugging.
    ///
    /// It asks the debug adapter to disconnect from the debuggee and to terminate the debug adapter.
    ///
    /// If the debuggee has been started with the 'launch' request, the 'disconnect' request terminates the debuggee.
    ///
    /// If the 'attach' request was used to connect to the debuggee, 'disconnect' does not terminate the debuggee.
    ///
    /// This behavior can be controlled with the 'terminateDebuggee' argument (if supported by the debug adapter).
    Disconnect(DisconnectRequestArguments),

    /// Evaluates the given expression in the context of the top most stack frame.
    ///
    /// The expression has access to any variables and arguments that are in scope.
    Evaluate(EvaluateRequestArguments),

    /// Retrieves the details of the exception that caused this event to be raised.
    ///
    /// Clients should only call this request if the capability 'supportsExceptionInfoRequest' is true.
    ExceptionInfo(ExceptionInfoRequestArguments),

    /// The request sets the location where the debuggee will continue to run.
    ///
    /// This makes it possible to skip the execution of code or to executed code again.
    ///
    /// The code between the current location and the goto target is not executed but skipped.
    ///
    /// The debug adapter first sends the response and then a 'stopped' event with reason 'goto'.
    ///
    /// Clients should only call this request if the capability 'supportsGotoTargetsRequest' is true (because only then goto targets exist that can be passed as arguments).
    Goto(GotoRequestArguments),

    /// This request retrieves the possible goto targets for the specified source location.
    ///
    /// These targets can be used in the 'goto' request.
    ///
    /// Clients should only call this request if the capability 'supportsGotoTargetsRequest' is true.
    GotoTargets(GotoTargetsRequestArguments),

    /// The 'initialize' request is sent as the first request from the client to the debug adapter
    ///
    /// in order to configure it with client capabilities and to retrieve capabilities from the debug adapter.
    ///
    /// Until the debug adapter has responded to with an 'initialize' response, the client must not send any additional requests or events to the debug adapter.
    ///
    /// In addition the debug adapter is not allowed to send any requests or events to the client until it has responded with an 'initialize' response.
    ///
    /// The 'initialize' request may only be sent once.
    Initialize(InitializeRequestArguments),

    /// This launch request is sent from the client to the debug adapter to start the debuggee with or without debugging (if 'noDebug' is true).
    ///
    /// Since launching is debugger/runtime specific, the arguments for this request are not part of this specification.
    Launch(LaunchRequestArguments),

    /// Retrieves the set of all sources currently loaded by the debugged process.
    ///
    /// Clients should only call this request if the capability 'supportsLoadedSourcesRequest' is true.
    LoadedSources,

    /// Modules can be retrieved from the debug adapter with this request which can either return all modules or a range of modules to support paging.
    ///
    /// Clients should only call this request if the capability 'supportsModulesRequest' is true.
    Modules(ModulesRequestArguments),

    /// The request starts the debuggee to run again for one step.
    ///
    /// The debug adapter first sends the response and then a 'stopped' event (with reason 'step') after the step has completed.
    Next(NextRequestArguments),

    /// The request suspends the debuggee.
    ///
    /// The debug adapter first sends the response and then a 'stopped' event (with reason 'pause') after the thread has been paused successfully.
    Pause(PauseRequestArguments),

    /// Reads bytes from memory at the provided location.
    ///
    /// Clients should only call this request if the capability 'supportsReadMemoryRequest' is true.
    ReadMemory(ReadMemoryRequestArguments),

    /// The request restarts execution of the specified stackframe.
    ///
    /// The debug adapter first sends the response and then a 'stopped' event (with reason 'restart') after the restart has completed.
    ///
    /// Clients should only call this request if the capability 'supportsRestartFrame' is true.
    RestartFrame(RestartFrameRequestArguments),

    // /// Restarts a debug session. Clients should only call this request if the capability 'supportsRestartRequest' is true.
    // ///
    // /// If the capability is missing or has the value false, a typical client will emulate 'restart' by terminating the debug adapter first and then launching it anew.
    // Restart(RestartRequestArguments), TODO
    /// The request starts the debuggee to run backward.
    ///
    /// Clients should only call this request if the capability 'supportsStepBack' is true.
    ReverseContinue(ReverseContinueRequestArguments),

    /// This optional request is sent from the debug adapter to the client to run a command in a terminal.
    ///
    /// This is typically used to launch the debuggee in a terminal provided by the client.
    ///
    /// This request should only be called if the client has passed the value true for the 'supportsRunInTerminalRequest' capability of the 'initialize' request.
    RunInTerminal(RunInTerminalRequestArguments),

    /// The request returns the variable scopes for a given stackframe ID.
    Scopes(ScopesRequestArguments),

    /// Sets multiple breakpoints for a single source and clears all previous breakpoints in that source.
    ///
    /// To clear all breakpoint for a source, specify an empty array.
    ///
    /// When a breakpoint is hit, a 'stopped' event (with reason 'breakpoint') is generated.
    SetBreakpoints(SetBreakpointsRequestArguments),

    /// Replaces all existing data breakpoints with new data breakpoints.
    ///
    /// To clear all data breakpoints, specify an empty array.
    ///
    /// When a data breakpoint is hit, a 'stopped' event (with reason 'data breakpoint') is generated.
    ///
    /// Clients should only call this request if the capability 'supportsDataBreakpoints' is true.
    SetDataBreakpoints(SetDataBreakpointsRequestArguments),

    /// The request configures the debuggers response to thrown exceptions.
    ///
    /// If an exception is configured to break, a 'stopped' event is fired (with reason 'exception').
    ///
    /// Clients should only call this request if the capability 'exceptionBreakpointFilters' returns one or more filters.
    SetExceptionBreakpoints(SetExceptionBreakpointsRequestArguments),

    /// Evaluates the given 'value' expression and assigns it to the 'expression' which must be a modifiable l-value.
    ///
    /// The expressions have access to any variables and arguments that are in scope of the specified frame.
    ///
    /// Clients should only call this request if the capability 'supportsSetExpression' is true.
    ///
    /// If a debug adapter implements both setExpression and setVariable, a client will only use setExpression if the variable has an evaluateName property.
    SetExpression(SetExpressionRequestArguments),

    /// Replaces all existing function breakpoints with new function breakpoints.
    ///
    /// To clear all function breakpoints, specify an empty array.
    ///
    /// When a function breakpoint is hit, a 'stopped' event (with reason 'function breakpoint') is generated.
    ///
    /// Clients should only call this request if the capability 'supportsFunctionBreakpoints' is true.
    SetFunctionBreakpoints(SetFunctionBreakpointsRequestArguments),

    /// Replaces all existing instruction breakpoints. Typically, instruction breakpoints would be set from a diassembly window.
    ///
    /// To clear all instruction breakpoints, specify an empty array.
    ///
    /// When an instruction breakpoint is hit, a 'stopped' event (with reason 'instruction breakpoint') is generated.
    ///
    /// Clients should only call this request if the capability 'supportsInstructionBreakpoints' is true.
    SetInstructionBreakpoints(SetInstructionBreakpointsRequestArguments),

    /// Set the variable with the given name in the variable container to a new value. Clients should only call this request if the capability 'supportsSetVariable' is true.
    ///
    /// If a debug adapter implements both setVariable and setExpression, a client will only use setExpression if the variable has an evaluateName property.
    SetVariable(SetVariableRequestArguments),

    /// The request retrieves the source code for a given source reference.
    Source(SourceRequestArguments),

    /// The request returns a stacktrace from the current execution state of a given thread.
    ///
    /// A client can request all stack frames by omitting the startFrame and levels arguments. For performance conscious clients and if the debug adapter's 'supportsDelayedStackTraceLoading' capability is true, stack frames can be retrieved in a piecemeal way with the startFrame and levels arguments. The response of the stackTrace request may contain a totalFrames property that hints at the total number of frames in the stack. If a client needs this total number upfront, it can issue a request for a single (first) frame and depending on the value of totalFrames decide how to proceed. In any case a client should be prepared to receive less frames than requested, which is an indication that the end of the stack has been reached.
    StackTrace(StackTraceRequestArguments),

    /// The request starts the debuggee to run one step backwards.
    ///
    /// The debug adapter first sends the response and then a 'stopped' event (with reason 'step') after the step has completed.
    ///
    /// Clients should only call this request if the capability 'supportsStepBack' is true.
    StepBack(StepBackRequestArguments),

    /// The request starts the debuggee to step into a function/method if possible.
    ///
    /// If it cannot step into a target, 'stepIn' behaves like 'next'.
    ///
    /// The debug adapter first sends the response and then a 'stopped' event (with reason 'step') after the step has completed.
    ///
    /// If there are multiple function/method calls (or other targets) on the source line,
    ///
    /// the optional argument 'targetId' can be used to control into which target the 'stepIn' should occur.
    ///
    /// The list of possible targets for a given source line can be retrieved via the 'stepInTargets' request.
    StepIn(StepInRequestArguments),

    /// This request retrieves the possible stepIn targets for the specified stack frame.
    ///
    /// These targets can be used in the 'stepIn' request.
    ///
    /// The StepInTargets may only be called if the 'supportsStepInTargetsRequest' capability exists and is true.
    ///
    /// Clients should only call this request if the capability 'supportsStepInTargetsRequest' is true.
    StepInTargets(StepInTargetsRequestArguments),

    /// The request starts the debuggee to run again for one step.
    ///
    /// The debug adapter first sends the response and then a 'stopped' event (with reason 'step') after the step has completed.
    StepOut(StepOutRequestArguments),

    /// The 'terminate' request is sent from the client to the debug adapter in order to give the debuggee a chance for terminating itself.
    ///
    /// Clients should only call this request if the capability 'supportsTerminateRequest' is true.
    Terminate(TerminateRequestArguments),

    /// The request terminates the threads with the given ids.
    ///
    /// Clients should only call this request if the capability 'supportsTerminateThreadsRequest' is true.
    TerminateThreads(TerminateThreadsRequestArguments),

    /// The request retrieves a list of all threads.
    Threads,

    /// Retrieves all child variables for the given variable reference.
    ///
    /// An optional filter can be used to limit the fetched children to either named or indexed children.
    Variables(VariablesRequestArguments),
}
impl From<Request> for ProtocolMessageContent {
    fn from(request: Request) -> Self {
        Self::Request(request)
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize, TypedBuilder)]
pub struct AttachRequestArguments {
    /// Optional data from the previous, restarted session.
    ///
    /// The data is sent as the 'restart' attribute of the 'terminated' event.
    ///
    /// The client should leave the data intact.
    #[serde(rename = "__restart", skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub restart: Option<Value>,

    #[serde(skip)]
    #[builder(default, setter(skip))]
    private: (),
}
impl From<AttachRequestArguments> for Request {
    fn from(args: AttachRequestArguments) -> Self {
        Self::Attach(args)
    }
}
impl From<AttachRequestArguments> for ProtocolMessageContent {
    fn from(args: AttachRequestArguments) -> Self {
        Self::from(Request::from(args))
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize, TypedBuilder)]
pub struct BreakpointLocationsRequestArguments {
    /// The source location of the breakpoints; either 'source.path' or 'source.reference' must be specified.
    #[serde(rename = "source")]
    pub source: Source,

    /// Start line of range to search possible breakpoint locations in. If only the line is specified, the request returns all possible locations in that line.
    #[serde(rename = "line")]
    pub line: i32,

    /// Optional start column of range to search possible breakpoint locations in. If no start column is given, the first column in the start line is assumed.
    #[serde(rename = "column", skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub column: Option<i32>,

    /// Optional end line of range to search possible breakpoint locations in. If no end line is given, then the end line is assumed to be the start line.
    #[serde(rename = "endLine", skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub end_line: Option<i32>,

    /// Optional end column of range to search possible breakpoint locations in. If no end column is given, then it is assumed to be in the last column of the end line.
    #[serde(rename = "endColumn", skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub end_column: Option<i32>,

    #[serde(skip)]
    #[builder(default, setter(skip))]
    private: (),
}
impl From<BreakpointLocationsRequestArguments> for Request {
    fn from(args: BreakpointLocationsRequestArguments) -> Self {
        Self::BreakpointLocations(args)
    }
}
impl From<BreakpointLocationsRequestArguments> for ProtocolMessageContent {
    fn from(args: BreakpointLocationsRequestArguments) -> Self {
        Self::from(Request::from(args))
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize, TypedBuilder)]
pub struct CancelRequestArguments {
    /// The ID (attribute 'seq') of the request to cancel. If missing no request is cancelled.
    ///
    /// Both a 'requestId' and a 'progressId' can be specified in one request.
    #[serde(rename = "requestId", skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub request_id: Option<i32>,

    /// The ID (attribute 'progressId') of the progress to cancel. If missing no progress is cancelled.
    ///
    /// Both a 'requestId' and a 'progressId' can be specified in one request.
    #[serde(rename = "progressId", skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub progress_id: Option<String>,

    #[serde(skip)]
    #[builder(default, setter(skip))]
    private: (),
}
impl From<CancelRequestArguments> for Request {
    fn from(args: CancelRequestArguments) -> Self {
        Self::Cancel(args)
    }
}
impl From<CancelRequestArguments> for ProtocolMessageContent {
    fn from(args: CancelRequestArguments) -> Self {
        Self::from(Request::from(args))
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize, TypedBuilder)]
pub struct CompletionsRequestArguments {
    /// Returns completions in the scope of this stack frame. If not specified, the completions are returned for the global scope.
    #[serde(rename = "frameId", skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub frame_id: Option<i32>,

    /// One or more source lines. Typically this is the text a user has typed into the debug console before he asked for completion.
    #[serde(rename = "text")]
    pub text: String,

    /// The character position for which to determine the completion proposals.
    #[serde(rename = "column")]
    pub column: i32,

    /// An optional line for which to determine the completion proposals. If missing the first line of the text is assumed.
    #[serde(rename = "line", skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub line: Option<i32>,

    #[serde(skip)]
    #[builder(default, setter(skip))]
    private: (),
}
impl From<CompletionsRequestArguments> for Request {
    fn from(args: CompletionsRequestArguments) -> Self {
        Self::Completions(args)
    }
}
impl From<CompletionsRequestArguments> for ProtocolMessageContent {
    fn from(args: CompletionsRequestArguments) -> Self {
        Self::from(Request::from(args))
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize, TypedBuilder)]
pub struct ContinueRequestArguments {
    /// Continue execution for the specified thread (if possible).
    ///
    /// If the backend cannot continue on a single thread but will continue on all threads, it should set the 'allThreadsContinued' attribute in the response to true.
    #[serde(rename = "threadId")]
    pub thread_id: i32,

    #[serde(skip)]
    #[builder(default, setter(skip))]
    private: (),
}
impl From<ContinueRequestArguments> for Request {
    fn from(args: ContinueRequestArguments) -> Self {
        Self::Continue(args)
    }
}
impl From<ContinueRequestArguments> for ProtocolMessageContent {
    fn from(args: ContinueRequestArguments) -> Self {
        Self::from(Request::from(args))
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize, TypedBuilder)]
pub struct DataBreakpointInfoRequestArguments {
    /// Reference to the Variable container if the data breakpoint is requested for a child of the container.
    #[serde(rename = "variablesReference", skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub variables_reference: Option<i32>,

    /// The name of the Variable's child to obtain data breakpoint information for.
    ///
    /// If variablesReference isn’t provided, this can be an expression.
    #[serde(rename = "name")]
    pub name: String,

    #[serde(skip)]
    #[builder(default, setter(skip))]
    private: (),
}
impl From<DataBreakpointInfoRequestArguments> for Request {
    fn from(args: DataBreakpointInfoRequestArguments) -> Self {
        Self::DataBreakpointInfo(args)
    }
}
impl From<DataBreakpointInfoRequestArguments> for ProtocolMessageContent {
    fn from(args: DataBreakpointInfoRequestArguments) -> Self {
        Self::from(Request::from(args))
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize, TypedBuilder)]
pub struct DisassembleRequestArguments {
    /// Memory reference to the base location containing the instructions to disassemble.
    #[serde(rename = "memoryReference")]
    pub memory_reference: String,

    /// Optional offset (in bytes) to be applied to the reference location before disassembling. Can be negative.
    #[serde(rename = "offset", default, skip_serializing_if = "eq_default")]
    #[builder(default)]
    pub offset: i32,

    /// Optional offset (in instructions) to be applied after the byte offset (if any) before disassembling. Can be negative.
    #[serde(
        rename = "instructionOffset",
        default,
        skip_serializing_if = "eq_default"
    )]
    #[builder(default)]
    pub instruction_offset: i32,

    /// Number of instructions to disassemble starting at the specified location and offset.
    ///
    /// An adapter must return exactly this number of instructions - any unavailable instructions should be replaced with an implementation-defined 'invalid instruction' value.
    #[serde(rename = "instructionCount")]
    pub instruction_count: i32,

    /// If true, the adapter should attempt to resolve memory addresses and other values to symbolic names.
    #[serde(rename = "resolveSymbols", default, skip_serializing_if = "eq_default")]
    #[builder(default)]
    pub resolve_symbols: bool,

    #[serde(skip)]
    #[builder(default, setter(skip))]
    private: (),
}
impl From<DisassembleRequestArguments> for Request {
    fn from(args: DisassembleRequestArguments) -> Self {
        Self::Disassemble(args)
    }
}
impl From<DisassembleRequestArguments> for ProtocolMessageContent {
    fn from(args: DisassembleRequestArguments) -> Self {
        Self::from(Request::from(args))
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize, TypedBuilder)]
pub struct DisconnectRequestArguments {
    /// A value of true indicates that this 'disconnect' request is part of a restart sequence.
    #[serde(rename = "restart", default, skip_serializing_if = "eq_default")]
    #[builder(default)]
    pub restart: bool,

    /// Indicates whether the debuggee should be terminated when the debugger is disconnected.
    ///
    /// If unspecified, the debug adapter is free to do whatever it thinks is best.
    ///
    /// The attribute is only honored by a debug adapter if the capability 'supportTerminateDebuggee' is true.
    #[serde(rename = "terminateDebuggee", skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub terminate_debuggee: Option<bool>,

    /// Indicates whether the debuggee should stay suspended when the debugger is disconnected.
    ///
    /// If unspecified, the debuggee should resume execution.
    ///
    /// The attribute is only honored by a debug adapter if the capability 'supportSuspendDebuggee' is true.
    #[serde(
        rename = "suspendDebuggee",
        default,
        skip_serializing_if = "eq_default"
    )]
    #[builder(default)]
    pub suspend_debuggee: bool,

    #[serde(skip)]
    #[builder(default, setter(skip))]
    private: (),
}
impl From<DisconnectRequestArguments> for Request {
    fn from(args: DisconnectRequestArguments) -> Self {
        Self::Disconnect(args)
    }
}
impl From<DisconnectRequestArguments> for ProtocolMessageContent {
    fn from(args: DisconnectRequestArguments) -> Self {
        Self::from(Request::from(args))
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize, TypedBuilder)]
pub struct EvaluateRequestArguments {
    /// The expression to evaluate.
    #[serde(rename = "expression")]
    pub expression: String,

    /// Evaluate the expression in the scope of this stack frame. If not specified, the expression is evaluated in the global scope.
    #[serde(rename = "frameId", skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub frame_id: Option<i32>,

    /// The context in which the evaluate request is run.
    #[serde(rename = "context", skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub context: Option<EvaluateRequestContext>,

    /// Specifies details on how to format the Evaluate result.
    ///
    /// The attribute is only honored by a debug adapter if the capability 'supportsValueFormattingOptions' is true.
    #[serde(rename = "format", skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub format: Option<ValueFormat>,

    #[serde(skip)]
    #[builder(default, setter(skip))]
    private: (),
}
impl From<EvaluateRequestArguments> for Request {
    fn from(args: EvaluateRequestArguments) -> Self {
        Self::Evaluate(args)
    }
}
impl From<EvaluateRequestArguments> for ProtocolMessageContent {
    fn from(args: EvaluateRequestArguments) -> Self {
        Self::from(Request::from(args))
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum EvaluateRequestContext {
    /// evaluate is run in a watch.
    Watch,

    /// evaluate is run from REPL console.
    REPL,

    /// evaluate is run from a data hover.
    Hover,

    /// evaluate is run to generate the value that will be stored in the clipboard.
    ///
    /// The attribute is only honored by a debug adapter if the capability 'supportsClipboardContext' is true.
    Clipboard,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize, TypedBuilder)]
pub struct ExceptionInfoRequestArguments {
    /// Thread for which exception information should be retrieved.
    #[serde(rename = "threadId")]
    pub thread_id: i32,

    #[serde(skip)]
    #[builder(default, setter(skip))]
    private: (),
}
impl From<ExceptionInfoRequestArguments> for Request {
    fn from(args: ExceptionInfoRequestArguments) -> Self {
        Self::ExceptionInfo(args)
    }
}
impl From<ExceptionInfoRequestArguments> for ProtocolMessageContent {
    fn from(args: ExceptionInfoRequestArguments) -> Self {
        Self::from(Request::from(args))
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize, TypedBuilder)]
pub struct GotoRequestArguments {
    /// Set the goto target for this thread.
    #[serde(rename = "threadId")]
    pub thread_id: i32,

    /// The location where the debuggee will continue to run.
    #[serde(rename = "targetId")]
    pub target_id: i32,

    #[serde(skip)]
    #[builder(default, setter(skip))]
    private: (),
}
impl From<GotoRequestArguments> for Request {
    fn from(args: GotoRequestArguments) -> Self {
        Self::Goto(args)
    }
}
impl From<GotoRequestArguments> for ProtocolMessageContent {
    fn from(args: GotoRequestArguments) -> Self {
        Self::from(Request::from(args))
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize, TypedBuilder)]
pub struct GotoTargetsRequestArguments {
    /// The source location for which the goto targets are determined.
    #[serde(rename = "source")]
    pub source: Source,

    /// The line location for which the goto targets are determined.
    #[serde(rename = "line")]
    pub line: i32,

    /// An optional column location for which the goto targets are determined.
    #[serde(rename = "column", skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub column: Option<i32>,

    #[serde(skip)]
    #[builder(default, setter(skip))]
    private: (),
}
impl From<GotoTargetsRequestArguments> for Request {
    fn from(args: GotoTargetsRequestArguments) -> Self {
        Self::GotoTargets(args)
    }
}
impl From<GotoTargetsRequestArguments> for ProtocolMessageContent {
    fn from(args: GotoTargetsRequestArguments) -> Self {
        Self::from(Request::from(args))
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize, TypedBuilder)]
pub struct InitializeRequestArguments {
    /// The ID of the (frontend) client using this adapter.
    #[serde(rename = "clientID", skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub client_id: Option<String>,

    /// The human readable name of the (frontend) client using this adapter.
    #[serde(rename = "clientName", skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub client_name: Option<String>,

    /// The ID of the debug adapter.
    #[serde(rename = "adapterID")]
    pub adapter_id: String,

    /// The ISO-639 locale of the (frontend) client using this adapter, e.g. en-US or de-CH.
    #[serde(rename = "locale", skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub locale: Option<String>,

    /// If true all line numbers are 1-based (default).
    #[serde(rename = "linesStartAt1", default = "true_")]
    #[builder(default = true)]
    pub lines_start_at_1: bool,

    /// If true all column numbers are 1-based (default).
    #[serde(rename = "columnsStartAt1", default = "true_")]
    #[builder(default = true)]
    pub columns_start_at_1: bool,

    /// Determines in what format paths are specified. The default is 'path', which is the native format.
    #[serde(rename = "pathFormat", default, skip_serializing_if = "eq_default")]
    #[builder(default)]
    pub path_format: PathFormat,

    /// Client supports the optional type attribute for variables.
    #[serde(
        rename = "supportsVariableType",
        default,
        skip_serializing_if = "eq_default"
    )]
    #[builder(default)]
    pub supports_variable_type: bool,

    /// Client supports the paging of variables.
    #[serde(
        rename = "supportsVariablePaging",
        default,
        skip_serializing_if = "eq_default"
    )]
    #[builder(default)]
    pub supports_variable_paging: bool,

    /// Client supports the runInTerminal request.
    #[serde(
        rename = "supportsRunInTerminalRequest",
        default,
        skip_serializing_if = "eq_default"
    )]
    #[builder(default)]
    pub supports_run_in_terminal_request: bool,

    /// Client supports memory references.
    #[serde(
        rename = "supportsMemoryReferences",
        default,
        skip_serializing_if = "eq_default"
    )]
    #[builder(default)]
    pub supports_memory_references: bool,

    /// Client supports progress reporting.
    #[serde(
        rename = "supportsProgressReporting",
        default,
        skip_serializing_if = "eq_default"
    )]
    #[builder(default)]
    pub supports_progress_reporting: bool,

    /// Client supports the invalidated event.
    #[serde(
        rename = "supportsInvalidatedEvent",
        default,
        skip_serializing_if = "eq_default"
    )]
    #[builder(default)]
    pub supports_invalidated_event: bool,

    #[serde(skip)]
    #[builder(default, setter(skip))]
    private: (),
}
impl From<InitializeRequestArguments> for Request {
    fn from(args: InitializeRequestArguments) -> Self {
        Self::Initialize(args)
    }
}
impl From<InitializeRequestArguments> for ProtocolMessageContent {
    fn from(args: InitializeRequestArguments) -> Self {
        Self::from(Request::from(args))
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum PathFormat {
    Path,
    URI,
}

impl Default for PathFormat {
    fn default() -> Self {
        PathFormat::Path
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize, TypedBuilder)]
pub struct LaunchRequestArguments {
    /// If noDebug is true the launch request should launch the program without enabling debugging.
    #[serde(rename = "noDebug", default, skip_serializing_if = "eq_default")]
    #[builder(default)]
    pub no_debug: bool,

    /// Optional data from the previous, restarted session.
    ///
    /// The data is sent as the 'restart' attribute of the 'terminated' event.
    ///
    /// The client should leave the data intact.
    #[serde(rename = "__restart", skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub restart: Option<Value>,

    /// Additional attributes are implementation specific.
    #[serde(flatten)]
    #[builder(default)]
    pub additional_attributes: Map<String, Value>,

    #[serde(skip)]
    #[builder(default, setter(skip))]
    private: (),
}
impl From<LaunchRequestArguments> for Request {
    fn from(args: LaunchRequestArguments) -> Self {
        Self::Launch(args)
    }
}
impl From<LaunchRequestArguments> for ProtocolMessageContent {
    fn from(args: LaunchRequestArguments) -> Self {
        Self::from(Request::from(args))
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize, TypedBuilder)]
pub struct ModulesRequestArguments {
    /// The index of the first module to return; if omitted modules start at 0.
    #[serde(rename = "startModule", default, skip_serializing_if = "eq_default")]
    #[builder(default)]
    pub start_module: i32,

    /// The number of modules to return. If moduleCount is not specified or 0, all modules are returned.
    #[serde(rename = "moduleCount", default, skip_serializing_if = "eq_default")]
    #[builder(default)]
    pub module_count: i32,

    #[serde(skip)]
    #[builder(default, setter(skip))]
    private: (),
}
impl From<ModulesRequestArguments> for Request {
    fn from(args: ModulesRequestArguments) -> Self {
        Self::Modules(args)
    }
}
impl From<ModulesRequestArguments> for ProtocolMessageContent {
    fn from(args: ModulesRequestArguments) -> Self {
        Self::from(Request::from(args))
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize, TypedBuilder)]
pub struct NextRequestArguments {
    /// Execute 'next' for this thread.
    #[serde(rename = "threadId")]
    pub thread_id: i32,

    /// Optional granularity to step. If no granularity is specified, a granularity of 'statement' is assumed.
    #[serde(rename = "granularity", default, skip_serializing_if = "eq_default")]
    #[builder(default)]
    pub granularity: SteppingGranularity,

    #[serde(skip)]
    #[builder(default, setter(skip))]
    private: (),
}
impl From<NextRequestArguments> for Request {
    fn from(args: NextRequestArguments) -> Self {
        Self::Next(args)
    }
}
impl From<NextRequestArguments> for ProtocolMessageContent {
    fn from(args: NextRequestArguments) -> Self {
        Self::from(Request::from(args))
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize, TypedBuilder)]
pub struct PauseRequestArguments {
    /// Pause execution for this thread.
    #[serde(rename = "threadId")]
    pub thread_id: i32,

    #[serde(skip)]
    #[builder(default, setter(skip))]
    private: (),
}
impl From<PauseRequestArguments> for Request {
    fn from(args: PauseRequestArguments) -> Self {
        Self::Pause(args)
    }
}
impl From<PauseRequestArguments> for ProtocolMessageContent {
    fn from(args: PauseRequestArguments) -> Self {
        Self::from(Request::from(args))
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize, TypedBuilder)]
pub struct ReadMemoryRequestArguments {
    /// Memory reference to the base location from which data should be read.
    #[serde(rename = "memoryReference")]
    pub memory_reference: String,

    /// Optional offset (in bytes) to be applied to the reference location before reading data. Can be negative.
    #[serde(rename = "offset", default, skip_serializing_if = "eq_default")]
    #[builder(default)]
    pub offset: i32,

    /// Number of bytes to read at the specified location and offset.
    #[serde(rename = "count")]
    pub count: i32,

    #[serde(skip)]
    #[builder(default, setter(skip))]
    private: (),
}
impl From<ReadMemoryRequestArguments> for Request {
    fn from(args: ReadMemoryRequestArguments) -> Self {
        Self::ReadMemory(args)
    }
}
impl From<ReadMemoryRequestArguments> for ProtocolMessageContent {
    fn from(args: ReadMemoryRequestArguments) -> Self {
        Self::from(Request::from(args))
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize, TypedBuilder)]
pub struct RestartFrameRequestArguments {
    /// Restart this stackframe.
    #[serde(rename = "frameId")]
    pub frame_id: i32,

    #[serde(skip)]
    #[builder(default, setter(skip))]
    private: (),
}
impl From<RestartFrameRequestArguments> for Request {
    fn from(args: RestartFrameRequestArguments) -> Self {
        Self::RestartFrame(args)
    }
}
impl From<RestartFrameRequestArguments> for ProtocolMessageContent {
    fn from(args: RestartFrameRequestArguments) -> Self {
        Self::from(Request::from(args))
    }
}

// #[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
// pub struct RestartRequestArguments {
//   /// The latest version of the 'launch' or 'attach' configuration.
//   #[serde(rename="arguments", skip_serializing_if = "Option::is_none")]
//   pub arguments: Option<TODO oneOf>,
// }

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize, TypedBuilder)]
pub struct ReverseContinueRequestArguments {
    /// Execute 'reverseContinue' for this thread.
    #[serde(rename = "threadId")]
    pub thread_id: i32,

    #[serde(skip)]
    #[builder(default, setter(skip))]
    private: (),
}
impl From<ReverseContinueRequestArguments> for Request {
    fn from(args: ReverseContinueRequestArguments) -> Self {
        Self::ReverseContinue(args)
    }
}
impl From<ReverseContinueRequestArguments> for ProtocolMessageContent {
    fn from(args: ReverseContinueRequestArguments) -> Self {
        Self::from(Request::from(args))
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize, TypedBuilder)]
pub struct RunInTerminalRequestArguments {
    /// What kind of terminal to launch.
    #[serde(rename = "kind", skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub kind: Option<TerminalKind>,

    /// Optional title of the terminal.
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub title: Option<String>,

    /// Working directory for the command. For non-empty, valid paths this typically results in execution of a change directory command.
    #[serde(rename = "cwd")]
    pub cwd: String,

    /// List of arguments. The first argument is the command to run.
    #[serde(rename = "args")]
    pub args: Vec<String>,

    /// Environment key-value pairs that are added to or removed from the default environment.
    #[serde(rename = "env", default, skip_serializing_if = "HashMap::is_empty")]
    #[builder(default)]
    pub env: HashMap<String, Option<String>>,

    #[serde(skip)]
    #[builder(default, setter(skip))]
    private: (),
}
impl From<RunInTerminalRequestArguments> for Request {
    fn from(args: RunInTerminalRequestArguments) -> Self {
        Self::RunInTerminal(args)
    }
}
impl From<RunInTerminalRequestArguments> for ProtocolMessageContent {
    fn from(args: RunInTerminalRequestArguments) -> Self {
        Self::from(Request::from(args))
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum TerminalKind {
    Integrated,

    External,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize, TypedBuilder)]
pub struct ScopesRequestArguments {
    /// Retrieve the scopes for this stackframe.
    #[serde(rename = "frameId")]
    pub frame_id: i32,

    #[serde(skip)]
    #[builder(default, setter(skip))]
    private: (),
}
impl From<ScopesRequestArguments> for Request {
    fn from(args: ScopesRequestArguments) -> Self {
        Self::Scopes(args)
    }
}
impl From<ScopesRequestArguments> for ProtocolMessageContent {
    fn from(args: ScopesRequestArguments) -> Self {
        Self::from(Request::from(args))
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize, TypedBuilder)]
pub struct SetBreakpointsRequestArguments {
    /// The source location of the breakpoints; either 'source.path' or 'source.reference' must be specified.
    #[serde(rename = "source")]
    pub source: Source,

    /// The code locations of the breakpoints.
    #[serde(rename = "breakpoints", default, skip_serializing_if = "Vec::is_empty")]
    #[builder(default)]
    pub breakpoints: Vec<SourceBreakpoint>,

    /// Deprecated: The code locations of the breakpoints.
    #[serde(rename = "lines", default, skip_serializing_if = "Vec::is_empty")]
    #[builder(default)]
    pub lines: Vec<i32>,

    /// A value of true indicates that the underlying source has been modified which results in new breakpoint locations.
    #[serde(rename = "sourceModified", default, skip_serializing_if = "eq_default")]
    #[builder(default)]
    pub source_modified: bool,

    #[serde(skip)]
    #[builder(default, setter(skip))]
    private: (),
}
impl From<SetBreakpointsRequestArguments> for Request {
    fn from(args: SetBreakpointsRequestArguments) -> Self {
        Self::SetBreakpoints(args)
    }
}
impl From<SetBreakpointsRequestArguments> for ProtocolMessageContent {
    fn from(args: SetBreakpointsRequestArguments) -> Self {
        Self::from(Request::from(args))
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize, TypedBuilder)]
pub struct SetDataBreakpointsRequestArguments {
    /// The contents of this array replaces all existing data breakpoints. An empty array clears all data breakpoints.
    #[serde(rename = "breakpoints")]
    pub breakpoints: Vec<DataBreakpoint>,

    #[serde(skip)]
    #[builder(default, setter(skip))]
    private: (),
}
impl From<SetDataBreakpointsRequestArguments> for Request {
    fn from(args: SetDataBreakpointsRequestArguments) -> Self {
        Self::SetDataBreakpoints(args)
    }
}
impl From<SetDataBreakpointsRequestArguments> for ProtocolMessageContent {
    fn from(args: SetDataBreakpointsRequestArguments) -> Self {
        Self::from(Request::from(args))
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize, TypedBuilder)]
pub struct SetExceptionBreakpointsRequestArguments {
    /// Set of exception filters specified by their ID. The set of all possible exception filters is defined by the 'exceptionBreakpointFilters' capability. The 'filter' and 'filterOptions' sets are additive.
    #[serde(rename = "filters")]
    pub filters: Vec<String>,

    /// Set of exception filters and their options. The set of all possible exception filters is defined by the 'exceptionBreakpointFilters' capability. This attribute is only honored by a debug adapter if the capability 'supportsExceptionFilterOptions' is true. The 'filter' and 'filterOptions' sets are additive.
    #[serde(
        rename = "filterOptions",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    #[builder(default)]
    pub filter_options: Vec<ExceptionFilterOptions>,

    /// Configuration options for selected exceptions.
    ///
    /// The attribute is only honored by a debug adapter if the capability 'supportsExceptionOptions' is true.
    #[serde(
        rename = "exceptionOptions",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    #[builder(default)]
    pub exception_options: Vec<ExceptionOptions>,

    #[serde(skip)]
    #[builder(default, setter(skip))]
    private: (),
}
impl From<SetExceptionBreakpointsRequestArguments> for Request {
    fn from(args: SetExceptionBreakpointsRequestArguments) -> Self {
        Self::SetExceptionBreakpoints(args)
    }
}
impl From<SetExceptionBreakpointsRequestArguments> for ProtocolMessageContent {
    fn from(args: SetExceptionBreakpointsRequestArguments) -> Self {
        Self::from(Request::from(args))
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize, TypedBuilder)]
pub struct SetExpressionRequestArguments {
    /// The l-value expression to assign to.
    #[serde(rename = "expression")]
    pub expression: String,

    /// The value expression to assign to the l-value expression.
    #[serde(rename = "value")]
    pub value: String,

    /// Evaluate the expressions in the scope of this stack frame. If not specified, the expressions are evaluated in the global scope.
    #[serde(rename = "frameId", skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub frame_id: Option<i32>,

    /// Specifies how the resulting value should be formatted.
    #[serde(rename = "format", skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub format: Option<ValueFormat>,

    #[serde(skip)]
    #[builder(default, setter(skip))]
    private: (),
}
impl From<SetExpressionRequestArguments> for Request {
    fn from(args: SetExpressionRequestArguments) -> Self {
        Self::SetExpression(args)
    }
}
impl From<SetExpressionRequestArguments> for ProtocolMessageContent {
    fn from(args: SetExpressionRequestArguments) -> Self {
        Self::from(Request::from(args))
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize, TypedBuilder)]
pub struct SetFunctionBreakpointsRequestArguments {
    /// The function names of the breakpoints.
    #[serde(rename = "breakpoints")]
    pub breakpoints: Vec<FunctionBreakpoint>,

    #[serde(skip)]
    #[builder(default, setter(skip))]
    private: (),
}
impl From<SetFunctionBreakpointsRequestArguments> for Request {
    fn from(args: SetFunctionBreakpointsRequestArguments) -> Self {
        Self::SetFunctionBreakpoints(args)
    }
}
impl From<SetFunctionBreakpointsRequestArguments> for ProtocolMessageContent {
    fn from(args: SetFunctionBreakpointsRequestArguments) -> Self {
        Self::from(Request::from(args))
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize, TypedBuilder)]
pub struct SetInstructionBreakpointsRequestArguments {
    /// The instruction references of the breakpoints
    #[serde(rename = "breakpoints")]
    pub breakpoints: Vec<InstructionBreakpoint>,

    #[serde(skip)]
    #[builder(default, setter(skip))]
    private: (),
}
impl From<SetInstructionBreakpointsRequestArguments> for Request {
    fn from(args: SetInstructionBreakpointsRequestArguments) -> Self {
        Self::SetInstructionBreakpoints(args)
    }
}
impl From<SetInstructionBreakpointsRequestArguments> for ProtocolMessageContent {
    fn from(args: SetInstructionBreakpointsRequestArguments) -> Self {
        Self::from(Request::from(args))
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize, TypedBuilder)]
pub struct SetVariableRequestArguments {
    /// The reference of the variable container.
    #[serde(rename = "variablesReference")]
    pub variables_reference: i32,

    /// The name of the variable in the container.
    #[serde(rename = "name")]
    pub name: String,

    /// The value of the variable.
    #[serde(rename = "value")]
    pub value: String,

    /// Specifies details on how to format the response value.
    #[serde(rename = "format", skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub format: Option<ValueFormat>,

    #[serde(skip)]
    #[builder(default, setter(skip))]
    private: (),
}
impl From<SetVariableRequestArguments> for Request {
    fn from(args: SetVariableRequestArguments) -> Self {
        Self::SetVariable(args)
    }
}
impl From<SetVariableRequestArguments> for ProtocolMessageContent {
    fn from(args: SetVariableRequestArguments) -> Self {
        Self::from(Request::from(args))
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize, TypedBuilder)]
pub struct SourceRequestArguments {
    /// Specifies the source content to load. Either source.path or source.sourceReference must be specified.
    #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub source: Option<Source>,

    /// The reference to the source. This is the same as source.sourceReference.
    ///
    /// This is provided for backward compatibility since old backends do not understand the 'source' attribute.
    #[serde(rename = "sourceReference")]
    pub source_reference: i32,

    #[serde(skip)]
    #[builder(default, setter(skip))]
    private: (),
}
impl From<SourceRequestArguments> for Request {
    fn from(args: SourceRequestArguments) -> Self {
        Self::Source(args)
    }
}
impl From<SourceRequestArguments> for ProtocolMessageContent {
    fn from(args: SourceRequestArguments) -> Self {
        Self::from(Request::from(args))
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize, TypedBuilder)]
pub struct StackTraceRequestArguments {
    /// Retrieve the stacktrace for this thread.
    #[serde(rename = "threadId")]
    pub thread_id: i32,

    /// The index of the first frame to return; if omitted frames start at 0.
    #[serde(rename = "startFrame", default, skip_serializing_if = "eq_default")]
    #[builder(default)]
    pub start_frame: i32,

    /// The maximum number of frames to return. If levels is not specified or 0, all frames are returned.
    #[serde(rename = "levels", default, skip_serializing_if = "eq_default")]
    #[builder(default)]
    pub levels: i32,

    /// Specifies details on how to format the stack frames.
    ///
    /// The attribute is only honored by a debug adapter if the capability 'supportsValueFormattingOptions' is true.
    #[serde(rename = "format", skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub format: Option<StackFrameFormat>,

    #[serde(skip)]
    #[builder(default, setter(skip))]
    private: (),
}
impl From<StackTraceRequestArguments> for Request {
    fn from(args: StackTraceRequestArguments) -> Self {
        Self::StackTrace(args)
    }
}
impl From<StackTraceRequestArguments> for ProtocolMessageContent {
    fn from(args: StackTraceRequestArguments) -> Self {
        Self::from(Request::from(args))
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize, TypedBuilder)]
pub struct StepBackRequestArguments {
    /// Execute 'stepBack' for this thread.
    #[serde(rename = "threadId")]
    pub thread_id: i32,

    /// Optional granularity to step. If no granularity is specified, a granularity of 'statement' is assumed.
    #[serde(rename = "granularity", default, skip_serializing_if = "eq_default")]
    #[builder(default)]
    pub granularity: SteppingGranularity,

    #[serde(skip)]
    #[builder(default, setter(skip))]
    private: (),
}
impl From<StepBackRequestArguments> for Request {
    fn from(args: StepBackRequestArguments) -> Self {
        Self::StepBack(args)
    }
}
impl From<StepBackRequestArguments> for ProtocolMessageContent {
    fn from(args: StepBackRequestArguments) -> Self {
        Self::from(Request::from(args))
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize, TypedBuilder)]
pub struct StepInRequestArguments {
    /// Execute 'stepIn' for this thread.
    #[serde(rename = "threadId")]
    pub thread_id: i32,

    /// Optional id of the target to step into.
    #[serde(rename = "targetId", skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub target_id: Option<i32>,

    /// Optional granularity to step. If no granularity is specified, a granularity of 'statement' is assumed.
    #[serde(rename = "granularity", default, skip_serializing_if = "eq_default")]
    #[builder(default)]
    pub granularity: SteppingGranularity,

    #[serde(skip)]
    #[builder(default, setter(skip))]
    private: (),
}
impl From<StepInRequestArguments> for Request {
    fn from(args: StepInRequestArguments) -> Self {
        Self::StepIn(args)
    }
}
impl From<StepInRequestArguments> for ProtocolMessageContent {
    fn from(args: StepInRequestArguments) -> Self {
        Self::from(Request::from(args))
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize, TypedBuilder)]
pub struct StepInTargetsRequestArguments {
    /// The stack frame for which to retrieve the possible stepIn targets.
    #[serde(rename = "frameId")]
    pub frame_id: i32,

    #[serde(skip)]
    #[builder(default, setter(skip))]
    private: (),
}
impl From<StepInTargetsRequestArguments> for Request {
    fn from(args: StepInTargetsRequestArguments) -> Self {
        Self::StepInTargets(args)
    }
}
impl From<StepInTargetsRequestArguments> for ProtocolMessageContent {
    fn from(args: StepInTargetsRequestArguments) -> Self {
        Self::from(Request::from(args))
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize, TypedBuilder)]
pub struct StepOutRequestArguments {
    /// Execute 'stepOut' for this thread.
    #[serde(rename = "threadId")]
    pub thread_id: i32,

    /// Optional granularity to step. If no granularity is specified, a granularity of 'statement' is assumed.
    #[serde(rename = "granularity", default, skip_serializing_if = "eq_default")]
    #[builder(default)]
    pub granularity: SteppingGranularity,

    #[serde(skip)]
    #[builder(default, setter(skip))]
    private: (),
}
impl From<StepOutRequestArguments> for Request {
    fn from(args: StepOutRequestArguments) -> Self {
        Self::StepOut(args)
    }
}
impl From<StepOutRequestArguments> for ProtocolMessageContent {
    fn from(args: StepOutRequestArguments) -> Self {
        Self::from(Request::from(args))
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize, TypedBuilder)]
pub struct TerminateRequestArguments {
    /// A value of true indicates that this 'terminate' request is part of a restart sequence.
    #[serde(rename = "restart", default, skip_serializing_if = "eq_default")]
    #[builder(default)]
    pub restart: bool,

    #[serde(skip)]
    #[builder(default, setter(skip))]
    private: (),
}
impl From<TerminateRequestArguments> for Request {
    fn from(args: TerminateRequestArguments) -> Self {
        Self::Terminate(args)
    }
}
impl From<TerminateRequestArguments> for ProtocolMessageContent {
    fn from(args: TerminateRequestArguments) -> Self {
        Self::from(Request::from(args))
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize, TypedBuilder)]
pub struct TerminateThreadsRequestArguments {
    /// Ids of threads to be terminated.
    #[serde(rename = "threadIds", default, skip_serializing_if = "Vec::is_empty")]
    #[builder(default)]
    pub thread_ids: Vec<i32>,

    #[serde(skip)]
    #[builder(default, setter(skip))]
    private: (),
}
impl From<TerminateThreadsRequestArguments> for Request {
    fn from(args: TerminateThreadsRequestArguments) -> Self {
        Self::TerminateThreads(args)
    }
}
impl From<TerminateThreadsRequestArguments> for ProtocolMessageContent {
    fn from(args: TerminateThreadsRequestArguments) -> Self {
        Self::from(Request::from(args))
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize, TypedBuilder)]
pub struct VariablesRequestArguments {
    /// The Variable reference.
    #[serde(rename = "variablesReference")]
    pub variables_reference: i32,

    /// Optional filter to limit the child variables to either named or indexed. If omitted, both types are fetched.
    #[serde(rename = "filter", skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub filter: Option<VariablesFilter>,

    /// The index of the first variable to return; if omitted children start at 0.
    #[serde(rename = "start", default, skip_serializing_if = "eq_default")]
    #[builder(default)]
    pub start: i32,

    /// The number of variables to return. If count is missing or 0, all variables are returned.
    #[serde(rename = "count", default, skip_serializing_if = "eq_default")]
    #[builder(default)]
    pub count: i32,

    /// Specifies details on how to format the Variable values.
    ///
    /// The attribute is only honored by a debug adapter if the capability 'supportsValueFormattingOptions' is true.
    #[serde(rename = "format", skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub format: Option<ValueFormat>,

    #[serde(skip)]
    #[builder(default, setter(skip))]
    private: (),
}
impl From<VariablesRequestArguments> for Request {
    fn from(args: VariablesRequestArguments) -> Self {
        Self::Variables(args)
    }
}
impl From<VariablesRequestArguments> for ProtocolMessageContent {
    fn from(args: VariablesRequestArguments) -> Self {
        Self::from(Request::from(args))
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum VariablesFilter {
    Indexed,

    Named,
}
