use crate::{
    types::{Breakpoint, Capabilities, InvalidatedAreas, Module, Source},
    utils::eq_default,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// A debug adapter initiated event.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase", tag = "event", content = "body")]
pub enum Event {
    /// The event indicates that some information about a breakpoint has changed.
    Breakpoint(BreakpointEventBody),

    /// The event indicates that one or more capabilities have changed.
    ///
    /// Since the capabilities are dependent on the frontend and its UI, it might not be possible to change that at random times (or too late).
    ///
    /// Consequently this event has a hint characteristic: a frontend can only be expected to make a 'best effort' in honouring individual capabilities but there are no guarantees.
    ///
    /// Only changed capabilities need to be included, all other capabilities keep their values.
    Capabilities(CapabilitiesEventBody),

    /// The event indicates that the execution of the debuggee has continued.
    ///
    /// Please note: a debug adapter is not expected to send this event in response to a request that implies that execution continues, e.g. 'launch' or 'continue'.
    ///
    /// It is only necessary to send a 'continued' event if there was no previous request that implied this.
    Continued(ContinuedEventBody),

    /// The event indicates that the debuggee has exited and returns its exit code.
    Exited(ExitedEventBody),

    /// This event indicates that the debug adapter is ready to accept configuration requests (e.g. SetBreakpointsRequest, SetExceptionBreakpointsRequest).
    ///
    /// A debug adapter is expected to send this event when it is ready to accept configuration requests (but not before the 'initialize' request has finished).
    ///
    /// The sequence of events/requests is as follows:
    ///
    /// - adapters sends 'initialized' event (after the 'initialize' request has returned)
    ///
    /// - frontend sends zero or more 'setBreakpoints' requests
    ///
    /// - frontend sends one 'setFunctionBreakpoints' request (if capability 'supportsFunctionBreakpoints' is true)
    ///
    /// - frontend sends a 'setExceptionBreakpoints' request if one or more 'exceptionBreakpointFilters' have been defined (or if 'supportsConfigurationDoneRequest' is not defined or false)
    ///
    /// - frontend sends other future configuration requests
    ///
    /// - frontend sends one 'configurationDone' request to indicate the end of the configuration.
    Initialized,

    /// This event signals that some state in the debug adapter has changed and requires that the client needs to re-render the data snapshot previously requested.
    ///
    /// Debug adapters do not have to emit this event for runtime changes like stopped or thread events because in that case the client refetches the new state anyway. But the event can be used for example to refresh the UI after rendering formatting has changed in the debug adapter.
    ///
    /// This event should only be sent if the debug adapter has received a value true for the 'supportsInvalidatedEvent' capability of the 'initialize' request.
    Invalidated(InvalidatedEventBody),

    /// The event indicates that some source has been added, changed, or removed from the set of all loaded sources.
    LoadedSource(LoadedSourceEventBody),

    /// The event indicates that some information about a module has changed.
    Module(ModuleEventBody),

    /// The event indicates that the target has produced some output.
    Output(OutputEventBody),

    /// The event indicates that the debugger has begun debugging a new process. Either one that it has launched, or one that it has attached to.
    Process(ProcessEventBody),

    /// The event signals the end of the progress reporting with an optional final message.
    ///
    /// This event should only be sent if the client has passed the value true for the 'supportsProgressReporting' capability of the 'initialize' request.
    ProgressEnd(ProgressEndEventBody),

    /// The event signals that a long running operation is about to start and
    ///
    /// provides additional information for the client to set up a corresponding progress and cancellation UI.
    ///
    /// The client is free to delay the showing of the UI in order to reduce flicker.
    ///
    /// This event should only be sent if the client has passed the value true for the 'supportsProgressReporting' capability of the 'initialize' request.
    ProgressStart(ProgressStartEventBody),

    /// The event signals that the progress reporting needs to updated with a new message and/or percentage.
    ///
    /// The client does not have to update the UI immediately, but the clients needs to keep track of the message and/or percentage values.
    ///
    /// This event should only be sent if the client has passed the value true for the 'supportsProgressReporting' capability of the 'initialize' request.
    ProgressUpdate(ProgressUpdateEventBody),

    /// The event indicates that the execution of the debuggee has stopped due to some condition.
    ///
    /// This can be caused by a break point previously set, a stepping request has completed, by executing a debugger statement etc.
    Stopped(StoppedEventBody),

    /// The event indicates that debugging of the debuggee has terminated. This does **not** mean that the debuggee itself has exited.
    Terminated(TerminatedEventBody),

    /// The event indicates that a thread has started or exited.
    Thread(ThreadEventBody),
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct BreakpointEventBody {
    /// The reason for the event.
    #[serde(rename = "reason")]
    pub reason: BreakpointEventReason,

    /// The 'id' attribute is used to find the target breakpoint and the other attributes are used as the new values.
    #[serde(rename = "breakpoint")]
    pub breakpoint: Breakpoint,
}

/// The reason for the event.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum BreakpointEventReason {
    #[serde(rename = "changed")]
    Changed,

    #[serde(rename = "new")]
    New,

    #[serde(rename = "removed")]
    Removed,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct CapabilitiesEventBody {
    /// The set of updated capabilities.
    #[serde(rename = "capabilities")]
    pub capabilities: Capabilities,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ContinuedEventBody {
    /// The thread which was continued.
    #[serde(rename = "threadId")]
    pub thread_id: i32,

    /// If 'allThreadsContinued' is true, a debug adapter can announce that all threads have continued.
    #[serde(
        rename = "allThreadsContinued",
        default,
        skip_serializing_if = "eq_default"
    )]
    pub all_threads_continued: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ExitedEventBody {
    /// The exit code returned from the debuggee.
    #[serde(rename = "exitCode")]
    pub exit_code: i32,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct InvalidatedEventBody {
    /// Optional set of logical areas that got invalidated. This property has a hint characteristic: a client can only be expected to make a 'best effort' in honouring the areas but there are no guarantees. If this property is missing, empty, or if values are not understand the client should assume a single value 'all'.
    #[serde(rename = "areas", default, skip_serializing_if = "Vec::is_empty")]
    pub areas: Vec<InvalidatedAreas>,

    /// If specified, the client only needs to refetch data related to this thread.
    #[serde(rename = "threadId", skip_serializing_if = "Option::is_none")]
    pub thread_id: Option<i32>,

    /// If specified, the client only needs to refetch data related to this stack frame (and the 'threadId' is ignored).
    #[serde(rename = "stackFrameId", skip_serializing_if = "Option::is_none")]
    pub stack_frame_id: Option<i32>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct LoadedSourceEventBody {
    /// The reason for the event.
    #[serde(rename = "reason")]
    pub reason: LoadedSourceEventReason,

    /// The new, changed, or removed source.
    #[serde(rename = "source")]
    pub source: Source,
}

/// The reason for the event.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum LoadedSourceEventReason {
    #[serde(rename = "new")]
    New,

    #[serde(rename = "changed")]
    Changed,

    #[serde(rename = "removed")]
    Removed,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ModuleEventBody {
    /// The reason for the event.
    #[serde(rename = "reason")]
    pub reason: ModuleEventReason,

    /// The new, changed, or removed module. In case of 'removed' only the module id is used.
    #[serde(rename = "module")]
    pub module: Module,
}

/// The reason for the event.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum ModuleEventReason {
    #[serde(rename = "new")]
    New,

    #[serde(rename = "changed")]
    Changed,

    #[serde(rename = "removed")]
    Removed,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct OutputEventBody {
    /// The output category. If not specified, 'console' is assumed.
    #[serde(rename = "category", default, skip_serializing_if = "eq_default")]
    pub category: OutputCategory,

    /// The output to report.
    #[serde(rename = "output")]
    pub output: String,

    /// Support for keeping an output log organized by grouping related messages.
    #[serde(rename = "group", skip_serializing_if = "Option::is_none")]
    pub group: Option<OutputGroup>,

    /// If an attribute 'variablesReference' exists and its value is > 0, the output contains objects which can be retrieved by passing 'variablesReference' to the 'variables' request. The value should be less than or equal to 2147483647 (2^31-1).
    #[serde(rename = "variablesReference", skip_serializing_if = "Option::is_none")]
    pub variables_reference: Option<i32>,

    /// An optional source location where the output was produced.
    #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
    pub source: Option<Source>,

    /// An optional source location line where the output was produced.
    #[serde(rename = "line", skip_serializing_if = "Option::is_none")]
    pub line: Option<i32>,

    /// An optional source location column where the output was produced.
    #[serde(rename = "column", skip_serializing_if = "Option::is_none")]
    pub column: Option<i32>,

    /// Optional data to report. For the 'telemetry' category the data will be sent to telemetry, for the other categories the data is shown in JSON format.
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<Value>,
}

/// The output category. If not specified, 'console' is assumed.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum OutputCategory {
    /// Show the output in the client's default message UI, e.g. a 'debug console'. This category should only be used for informational output from the debugger (as opposed to the debuggee).
    #[serde(rename = "console")]
    Console,

    /// A hint for the client to show the ouput in the client's UI for important and highly visible information, e.g. as a popup notification. This category should only be used for important messages from the debugger (as opposed to the debuggee). Since this category value is a hint, clients might ignore the hint and assume the 'console' category.
    #[serde(rename = "important")]
    Important,

    /// Show the output as normal program output from the debuggee.
    #[serde(rename = "stdout")]
    Stdout,

    /// Show the output as error program output from the debuggee.
    #[serde(rename = "stderr")]
    Stderr,

    /// Send the output to telemetry instead of showing it to the user.
    #[serde(rename = "telemetry")]
    Telemetry,
}

impl Default for OutputCategory {
    fn default() -> Self {
        OutputCategory::Console
    }
}

/// Support for keeping an output log organized by grouping related messages.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum OutputGroup {
    /// Start a new group in expanded mode. Subsequent output events are members of the group and should be shown indented.
    ///
    /// The 'output' attribute becomes the name of the group and is not indented.
    #[serde(rename = "start")]
    Start,

    /// Start a new group in collapsed mode. Subsequent output events are members of the group and should be shown indented (as soon as the group is expanded).
    ///
    /// The 'output' attribute becomes the name of the group and is not indented.
    #[serde(rename = "startCollapsed")]
    StartCollapsed,

    /// End the current group and decreases the indentation of subsequent output events.
    ///
    /// A non empty 'output' attribute is shown as the unindented end of the group.
    #[serde(rename = "end")]
    End,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ProcessEventBody {
    /// The logical name of the process. This is usually the full path to process's executable file. Example: /home/example/myproj/program.js.
    #[serde(rename = "name")]
    pub name: String,

    /// The system process id of the debugged process. This property will be missing for non-system processes.
    #[serde(rename = "systemProcessId", skip_serializing_if = "Option::is_none")]
    pub system_process_id: Option<i32>,

    /// If true, the process is running on the same computer as the debug adapter.
    #[serde(rename = "isLocalProcess", skip_serializing_if = "Option::is_none")]
    pub is_local_process: Option<bool>,

    /// Describes how the debug engine started debugging this process.
    #[serde(rename = "startMethod", skip_serializing_if = "Option::is_none")]
    pub start_method: Option<ProcessStartMethod>,

    /// The size of a pointer or address for this process, in bits. This value may be used by clients when formatting addresses for display.
    #[serde(rename = "pointerSize", skip_serializing_if = "Option::is_none")]
    pub pointer_size: Option<i32>,
}

/// Describes how the debug engine started debugging this process.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum ProcessStartMethod {
    /// Process was launched under the debugger.
    #[serde(rename = "launch")]
    Launch,

    /// Debugger attached to an existing process.
    #[serde(rename = "attach")]
    Attach,

    /// A project launcher component has launched a new process in a suspended state and then asked the debugger to attach.
    #[serde(rename = "attachForSuspendedLaunch")]
    AttachForSuspendedLaunch,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ProgressEndEventBody {
    /// The ID that was introduced in the initial 'ProgressStartEvent'.
    #[serde(rename = "progressId")]
    pub progress_id: String,

    /// Optional, more detailed progress message. If omitted, the previous message (if any) is used.
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ProgressStartEventBody {
    /// An ID that must be used in subsequent 'progressUpdate' and 'progressEnd' events to make them refer to the same progress reporting.
    ///
    /// IDs must be unique within a debug session.
    #[serde(rename = "progressId")]
    pub progress_id: String,

    /// Mandatory (short) title of the progress reporting. Shown in the UI to describe the long running operation.
    #[serde(rename = "title")]
    pub title: String,

    /// The request ID that this progress report is related to. If specified a debug adapter is expected to emit
    ///
    /// progress events for the long running request until the request has been either completed or cancelled.
    ///
    /// If the request ID is omitted, the progress report is assumed to be related to some general activity of the debug adapter.
    #[serde(rename = "requestId", skip_serializing_if = "Option::is_none")]
    pub request_id: Option<i32>,

    /// If true, the request that reports progress may be canceled with a 'cancel' request.
    ///
    /// So this property basically controls whether the client should use UX that supports cancellation.
    ///
    /// Clients that don't support cancellation are allowed to ignore the setting.
    #[serde(rename = "cancellable", default, skip_serializing_if = "eq_default")]
    pub cancellable: bool,

    /// Optional, more detailed progress message.
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,

    /// Optional progress percentage to display (value range: 0 to 100). If omitted no percentage will be shown.
    #[serde(rename = "percentage", skip_serializing_if = "Option::is_none")]
    pub percentage: Option<u8>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ProgressUpdateEventBody {
    /// The ID that was introduced in the initial 'progressStart' event.
    #[serde(rename = "progressId")]
    pub progress_id: String,

    /// Optional, more detailed progress message. If omitted, the previous message (if any) is used.
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,

    /// Optional progress percentage to display (value range: 0 to 100). If omitted no percentage will be shown.
    #[serde(rename = "percentage", skip_serializing_if = "Option::is_none")]
    pub percentage: Option<u8>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct StoppedEventBody {
    /// The reason for the event.
    ///
    /// For backward compatibility this string is shown in the UI if the 'description' attribute is missing (but it must not be translated).
    #[serde(rename = "reason")]
    pub reason: StoppedEventReason,

    /// The full reason for the event, e.g. 'Paused on exception'. This string is shown in the UI as is and must be translated.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// The thread which was stopped.
    #[serde(rename = "threadId", skip_serializing_if = "Option::is_none")]
    pub thread_id: Option<i32>,

    /// A value of true hints to the frontend that this event should not change the focus.
    #[serde(
        rename = "preserveFocusHint",
        default,
        skip_serializing_if = "eq_default"
    )]
    pub preserve_focus_hint: bool,

    /// Additional information. E.g. if reason is 'exception', text contains the exception name. This string is shown in the UI.
    #[serde(rename = "text", skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,

    /// If 'allThreadsStopped' is true, a debug adapter can announce that all threads have stopped.
    ///
    /// - The client should use this information to enable that all threads can be expanded to access their stacktraces.
    ///
    /// - If the attribute is missing or false, only the thread with the given threadId can be expanded.
    #[serde(
        rename = "allThreadsStopped",
        default,
        skip_serializing_if = "eq_default"
    )]
    pub all_threads_stopped: bool,

    /// Ids of the breakpoints that triggered the event. In most cases there will be only a single breakpoint but here are some examples for multiple breakpoints:
    ///
    /// - Different types of breakpoints map to the same location.
    ///
    /// - Multiple source breakpoints get collapsed to the same instruction by the compiler/runtime.
    ///
    /// - Multiple function breakpoints with different function names map to the same location.
    #[serde(
        rename = "hitBreakpointIds",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub hit_breakpoint_ids: Vec<i32>,
}

/// The reason for the event.
///
/// For backward compatibility this string is shown in the UI if the 'description' attribute is missing (but it must not be translated).
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum StoppedEventReason {
    #[serde(rename = "step")]
    Step,

    #[serde(rename = "breakpoint")]
    Breakpoint,

    #[serde(rename = "exception")]
    Exception,

    #[serde(rename = "pause")]
    Pause,

    #[serde(rename = "entry")]
    Entry,

    #[serde(rename = "goto")]
    Goto,

    #[serde(rename = "function breakpoint")]
    FunctionBreakpoint,

    #[serde(rename = "data breakpoint")]
    DataBreakpoint,

    #[serde(rename = "instruction breakpoint")]
    InstructionBreakpoint,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct TerminatedEventBody {
    /// A debug adapter may set 'restart' to true (or to an arbitrary object) to request that the front end restarts the session.
    ///
    /// The value is not interpreted by the client and passed unmodified as an attribute '__restart' to the 'launch' and 'attach' requests.
    #[serde(rename = "restart", skip_serializing_if = "Option::is_none")]
    pub restart: Option<Value>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ThreadEventBody {
    /// The reason for the event.
    #[serde(rename = "reason")]
    pub reason: ThreadEventReason,

    /// The identifier of the thread.
    #[serde(rename = "threadId")]
    pub thread_id: i32,
}

/// The reason for the event.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum ThreadEventReason {
    #[serde(rename = "started")]
    Started,

    #[serde(rename = "exited")]
    Exited,
}
