use crate::utils::eq_default;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

/// Information about a Breakpoint created in setBreakpoints, setFunctionBreakpoints, setInstructionBreakpoints, or setDataBreakpoints.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Breakpoint {
    /// An optional identifier for the breakpoint. It is needed if breakpoint events are used to update or remove breakpoints.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,

    /// If true breakpoint could be set (but not necessarily at the desired location).
    #[serde(rename = "verified")]
    pub verified: bool,

    /// An optional message about the state of the breakpoint.
    ///
    /// This is shown to the user and can be used to explain why a breakpoint could not be verified.
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,

    /// The source where the breakpoint is located.
    #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
    pub source: Option<Source>,

    /// The start line of the actual range covered by the breakpoint.
    #[serde(rename = "line", skip_serializing_if = "Option::is_none")]
    pub line: Option<i32>,

    /// An optional start column of the actual range covered by the breakpoint.
    #[serde(rename = "column", skip_serializing_if = "Option::is_none")]
    pub column: Option<i32>,

    /// An optional end line of the actual range covered by the breakpoint.
    #[serde(rename = "endLine", skip_serializing_if = "Option::is_none")]
    pub end_line: Option<i32>,

    /// An optional end column of the actual range covered by the breakpoint.
    ///
    /// If no end line is given, then the end column is assumed to be in the start line.
    #[serde(rename = "endColumn", skip_serializing_if = "Option::is_none")]
    pub end_column: Option<i32>,

    /// An optional memory reference to where the breakpoint is set.
    #[serde(
        rename = "instructionReference",
        skip_serializing_if = "Option::is_none"
    )]
    pub instruction_reference: Option<String>,

    /// An optional offset from the instruction reference.
    ///
    /// This can be negative.
    #[serde(rename = "offset", skip_serializing_if = "Option::is_none")]
    pub offset: Option<i32>,
}

/// Properties of a breakpoint location returned from the 'breakpointLocations' request.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct BreakpointLocation {
    /// Start line of breakpoint location.
    #[serde(rename = "line")]
    pub line: i32,

    /// Optional start column of breakpoint location.
    #[serde(rename = "column", skip_serializing_if = "Option::is_none")]
    pub column: Option<i32>,

    /// Optional end line of breakpoint location if the location covers a range.
    #[serde(rename = "endLine", skip_serializing_if = "Option::is_none")]
    pub end_line: Option<i32>,

    /// Optional end column of breakpoint location if the location covers a range.
    #[serde(rename = "endColumn", skip_serializing_if = "Option::is_none")]
    pub end_column: Option<i32>,
}

/// Information about the capabilities of a debug adapter.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct Capabilities {
    /// The debug adapter supports the 'configurationDone' request.
    #[serde(
        rename = "supportsConfigurationDoneRequest",
        default,
        skip_serializing_if = "eq_default"
    )]
    pub supports_configuration_done_request: bool,

    /// The debug adapter supports function breakpoints.
    #[serde(
        rename = "supportsFunctionBreakpoints",
        default,
        skip_serializing_if = "eq_default"
    )]
    pub supports_function_breakpoints: bool,

    /// The debug adapter supports conditional breakpoints.
    #[serde(
        rename = "supportsConditionalBreakpoints",
        default,
        skip_serializing_if = "eq_default"
    )]
    pub supports_conditional_breakpoints: bool,

    /// The debug adapter supports breakpoints that break execution after a specified number of hits.
    #[serde(
        rename = "supportsHitConditionalBreakpoints",
        default,
        skip_serializing_if = "eq_default"
    )]
    pub supports_hit_conditional_breakpoints: bool,

    /// The debug adapter supports a (side effect free) evaluate request for data hovers.
    #[serde(
        rename = "supportsEvaluateForHovers",
        default,
        skip_serializing_if = "eq_default"
    )]
    pub supports_evaluate_for_hovers: bool,

    /// Available exception filter options for the 'setExceptionBreakpoints' request.
    #[serde(
        rename = "exceptionBreakpointFilters",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub exception_breakpoint_filters: Vec<ExceptionBreakpointsFilter>,

    /// The debug adapter supports stepping back via the 'stepBack' and 'reverseContinue' requests.
    #[serde(
        rename = "supportsStepBack",
        default,
        skip_serializing_if = "eq_default"
    )]
    pub supports_step_back: bool,

    /// The debug adapter supports setting a variable to a value.
    #[serde(
        rename = "supportsSetVariable",
        default,
        skip_serializing_if = "eq_default"
    )]
    pub supports_set_variable: bool,

    /// The debug adapter supports restarting a frame.
    #[serde(
        rename = "supportsRestartFrame",
        default,
        skip_serializing_if = "eq_default"
    )]
    pub supports_restart_frame: bool,

    /// The debug adapter supports the 'gotoTargets' request.
    #[serde(
        rename = "supportsGotoTargetsRequest",
        default,
        skip_serializing_if = "eq_default"
    )]
    pub supports_goto_targets_request: bool,

    /// The debug adapter supports the 'stepInTargets' request.
    #[serde(
        rename = "supportsStepInTargetsRequest",
        default,
        skip_serializing_if = "eq_default"
    )]
    pub supports_step_in_targets_request: bool,

    /// The debug adapter supports the 'completions' request.
    #[serde(
        rename = "supportsCompletionsRequest",
        default,
        skip_serializing_if = "eq_default"
    )]
    pub supports_completions_request: bool,

    /// The set of characters that should trigger completion in a REPL. If not specified, the UI should assume the '.' character.
    #[serde(
        rename = "completionTriggerCharacters",
        skip_serializing_if = "Option::is_none"
    )]
    pub completion_trigger_characters: Option<Vec<String>>,

    /// The debug adapter supports the 'modules' request.
    #[serde(
        rename = "supportsModulesRequest",
        default,
        skip_serializing_if = "eq_default"
    )]
    pub supports_modules_request: bool,

    /// The set of additional module information exposed by the debug adapter.
    #[serde(
        rename = "additionalModuleColumns",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub additional_module_columns: Vec<ColumnDescriptor>,

    /// Checksum algorithms supported by the debug adapter.
    #[serde(
        rename = "supportedChecksumAlgorithms",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub supported_checksum_algorithms: Vec<ChecksumAlgorithm>,

    /// The debug adapter supports the 'restart' request. In this case a client should not implement 'restart' by terminating and relaunching the adapter but by calling the RestartRequest.
    #[serde(
        rename = "supportsRestartRequest",
        default,
        skip_serializing_if = "eq_default"
    )]
    pub supports_restart_request: bool,

    /// The debug adapter supports 'exceptionOptions' on the setExceptionBreakpoints request.
    #[serde(
        rename = "supportsExceptionOptions",
        default,
        skip_serializing_if = "eq_default"
    )]
    pub supports_exception_options: bool,

    /// The debug adapter supports a 'format' attribute on the stackTraceRequest, variablesRequest, and evaluateRequest.
    #[serde(
        rename = "supportsValueFormattingOptions",
        default,
        skip_serializing_if = "eq_default"
    )]
    pub supports_value_formatting_options: bool,

    /// The debug adapter supports the 'exceptionInfo' request.
    #[serde(
        rename = "supportsExceptionInfoRequest",
        default,
        skip_serializing_if = "eq_default"
    )]
    pub supports_exception_info_request: bool,

    /// The debug adapter supports the 'terminateDebuggee' attribute on the 'disconnect' request.
    #[serde(
        rename = "supportTerminateDebuggee",
        default,
        skip_serializing_if = "eq_default"
    )]
    pub support_terminate_debuggee: bool,

    /// The debug adapter supports the 'suspendDebuggee' attribute on the 'disconnect' request.
    #[serde(
        rename = "supportSuspendDebuggee",
        default,
        skip_serializing_if = "eq_default"
    )]
    pub support_suspend_debuggee: bool,

    /// The debug adapter supports the delayed loading of parts of the stack, which requires that both the 'startFrame' and 'levels' arguments and an optional 'totalFrames' result of the 'StackTrace' request are supported.
    #[serde(
        rename = "supportsDelayedStackTraceLoading",
        default,
        skip_serializing_if = "eq_default"
    )]
    pub supports_delayed_stack_trace_loading: bool,

    /// The debug adapter supports the 'loadedSources' request.
    #[serde(
        rename = "supportsLoadedSourcesRequest",
        default,
        skip_serializing_if = "eq_default"
    )]
    pub supports_loaded_sources_request: bool,

    /// The debug adapter supports logpoints by interpreting the 'logMessage' attribute of the SourceBreakpoint.
    #[serde(
        rename = "supportsLogPoints",
        default,
        skip_serializing_if = "eq_default"
    )]
    pub supports_log_points: bool,

    /// The debug adapter supports the 'terminateThreads' request.
    #[serde(
        rename = "supportsTerminateThreadsRequest",
        default,
        skip_serializing_if = "eq_default"
    )]
    pub supports_terminate_threads_request: bool,

    /// The debug adapter supports the 'setExpression' request.
    #[serde(
        rename = "supportsSetExpression",
        default,
        skip_serializing_if = "eq_default"
    )]
    pub supports_set_expression: bool,

    /// The debug adapter supports the 'terminate' request.
    #[serde(
        rename = "supportsTerminateRequest",
        default,
        skip_serializing_if = "eq_default"
    )]
    pub supports_terminate_request: bool,

    /// The debug adapter supports data breakpoints.
    #[serde(
        rename = "supportsDataBreakpoints",
        default,
        skip_serializing_if = "eq_default"
    )]
    pub supports_data_breakpoints: bool,

    /// The debug adapter supports the 'readMemory' request.
    #[serde(
        rename = "supportsReadMemoryRequest",
        default,
        skip_serializing_if = "eq_default"
    )]
    pub supports_read_memory_request: bool,

    /// The debug adapter supports the 'disassemble' request.
    #[serde(
        rename = "supportsDisassembleRequest",
        default,
        skip_serializing_if = "eq_default"
    )]
    pub supports_disassemble_request: bool,

    /// The debug adapter supports the 'cancel' request.
    #[serde(
        rename = "supportsCancelRequest",
        default,
        skip_serializing_if = "eq_default"
    )]
    pub supports_cancel_request: bool,

    /// The debug adapter supports the 'breakpointLocations' request.
    #[serde(
        rename = "supportsBreakpointLocationsRequest",
        default,
        skip_serializing_if = "eq_default"
    )]
    pub supports_breakpoint_locations_request: bool,

    /// The debug adapter supports the 'clipboard' context value in the 'evaluate' request.
    #[serde(
        rename = "supportsClipboardContext",
        default,
        skip_serializing_if = "eq_default"
    )]
    pub supports_clipboard_context: bool,

    /// The debug adapter supports stepping granularities (argument 'granularity') for the stepping requests.
    #[serde(
        rename = "supportsSteppingGranularity",
        default,
        skip_serializing_if = "eq_default"
    )]
    pub supports_stepping_granularity: bool,

    /// The debug adapter supports adding breakpoints based on instruction references.
    #[serde(
        rename = "supportsInstructionBreakpoints",
        default,
        skip_serializing_if = "eq_default"
    )]
    pub supports_instruction_breakpoints: bool,

    /// The debug adapter supports 'filterOptions' as an argument on the 'setExceptionBreakpoints' request.
    #[serde(
        rename = "supportsExceptionFilterOptions",
        default,
        skip_serializing_if = "eq_default"
    )]
    pub supports_exception_filter_options: bool,
}

/// The checksum of an item calculated by the specified algorithm.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Checksum {
    /// The algorithm used to calculate this checksum.
    #[serde(rename = "algorithm")]
    pub algorithm: ChecksumAlgorithm,

    /// Value of the checksum.
    #[serde(rename = "checksum")]
    pub checksum: String,
}

/// Names of checksum algorithms that may be supported by a debug adapter.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum ChecksumAlgorithm {
    #[serde(rename = "MD5")]
    MD5,

    #[serde(rename = "SHA1")]
    SHA1,

    #[serde(rename = "SHA256")]
    SHA256,

    #[serde(rename = "timestamp")]
    Timestamp,
}

/// A ColumnDescriptor specifies what module attribute to show in a column of the ModulesView, how to format it,
///
/// and what the column's label should be.
///
/// It is only used if the underlying UI actually supports this level of customization.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ColumnDescriptor {
    /// Name of the attribute rendered in this column.
    #[serde(rename = "attributeName")]
    pub attribute_name: String,

    /// Header UI label of column.
    #[serde(rename = "label")]
    pub label: String,

    /// Format to use for the rendered values in this column. TBD how the format strings looks like.
    #[serde(rename = "format", skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,

    /// Datatype of values in this column.  Defaults to 'string' if not specified.
    #[serde(rename = "type", default, skip_serializing_if = "eq_default")]
    pub type_: ColumnDescriptorType,

    /// Width of this column in characters (hint only).
    #[serde(rename = "width", skip_serializing_if = "Option::is_none")]
    pub width: Option<i32>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum ColumnDescriptorType {
    #[serde(rename = "string")]
    String,

    #[serde(rename = "number")]
    Number,

    #[serde(rename = "boolean")]
    Boolean,

    #[serde(rename = "unixTimestampUTC")]
    UnixTimestampUTC,
}

impl Default for ColumnDescriptorType {
    fn default() -> Self {
        ColumnDescriptorType::String
    }
}

/// CompletionItems are the suggestions returned from the CompletionsRequest.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct CompletionItem {
    /// The label of this completion item. By default this is also the text that is inserted when selecting this completion.
    #[serde(rename = "label")]
    pub label: String,

    /// If text is not falsy then it is inserted instead of the label.
    #[serde(rename = "text", skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,

    /// A string that should be used when comparing this item with other items. When `falsy` the label is used.
    #[serde(rename = "sortText", skip_serializing_if = "Option::is_none")]
    pub sort_text: Option<String>,

    /// The item's type. Typically the client uses this information to render the item in the UI with an icon.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<CompletionItemType>,

    /// This value determines the location (in the CompletionsRequest's 'text' attribute) where the completion text is added.
    ///
    /// If missing the text is added at the location specified by the CompletionsRequest's 'column' attribute.
    #[serde(rename = "start", skip_serializing_if = "Option::is_none")]
    pub start: Option<i32>,

    /// This value determines how many characters are overwritten by the completion text.
    ///
    /// If missing the value 0 is assumed which results in the completion text being inserted.
    #[serde(rename = "length", default, skip_serializing_if = "eq_default")]
    pub length: i32,

    /// Determines the start of the new selection after the text has been inserted (or replaced).
    ///
    /// The start position must in the range 0 and length of the completion text.
    ///
    /// If omitted the selection starts at the end of the completion text.
    #[serde(rename = "selectionStart", skip_serializing_if = "Option::is_none")]
    pub selection_start: Option<i32>,

    /// Determines the length of the new selection after the text has been inserted (or replaced).
    ///
    /// The selection can not extend beyond the bounds of the completion text.
    ///
    /// If omitted the length is assumed to be 0.
    #[serde(
        rename = "selectionLength",
        default,
        skip_serializing_if = "eq_default"
    )]
    pub selection_length: i32,
}

/// Some predefined types for the CompletionItem. Please note that not all clients have specific icons for all of them.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum CompletionItemType {
    #[serde(rename = "method")]
    Method,

    #[serde(rename = "function")]
    Function,

    #[serde(rename = "constructor")]
    Constructor,

    #[serde(rename = "field")]
    Field,

    #[serde(rename = "variable")]
    Variable,

    #[serde(rename = "class")]
    Class,

    #[serde(rename = "interface")]
    Interface,

    #[serde(rename = "module")]
    Module,

    #[serde(rename = "property")]
    Property,

    #[serde(rename = "unit")]
    Unit,

    #[serde(rename = "value")]
    Value,

    #[serde(rename = "enum")]
    Enum,

    #[serde(rename = "keyword")]
    Keyword,

    #[serde(rename = "snippet")]
    Snippet,

    #[serde(rename = "text")]
    Text,

    #[serde(rename = "color")]
    Color,

    #[serde(rename = "file")]
    File,

    #[serde(rename = "reference")]
    Reference,

    #[serde(rename = "customcolor")]
    Customcolor,
}

/// Properties of a data breakpoint passed to the setDataBreakpoints request.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct DataBreakpoint {
    /// An id representing the data. This id is returned from the dataBreakpointInfo request.
    #[serde(rename = "dataId")]
    pub data_id: String,

    /// The access type of the data.
    #[serde(rename = "accessType", skip_serializing_if = "Option::is_none")]
    pub access_type: Option<DataBreakpointAccessType>,

    /// An optional expression for conditional breakpoints.
    #[serde(rename = "condition", skip_serializing_if = "Option::is_none")]
    pub condition: Option<String>,

    /// An optional expression that controls how many hits of the breakpoint are ignored.
    ///
    /// The backend is expected to interpret the expression as needed.
    #[serde(rename = "hitCondition", skip_serializing_if = "Option::is_none")]
    pub hit_condition: Option<String>,
}

/// This enumeration defines all possible access types for data breakpoints.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum DataBreakpointAccessType {
    #[serde(rename = "read")]
    Read,

    #[serde(rename = "write")]
    Write,

    #[serde(rename = "readWrite")]
    ReadWrite,
}

/// Represents a single disassembled instruction.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct DisassembledInstruction {
    /// The address of the instruction. Treated as a hex value if prefixed with '0x', or as a decimal value otherwise.
    #[serde(rename = "address")]
    pub address: String,

    /// Optional raw bytes representing the instruction and its operands, in an implementation-defined format.
    #[serde(rename = "instructionBytes", skip_serializing_if = "Option::is_none")]
    pub instruction_bytes: Option<String>,

    /// Text representing the instruction and its operands, in an implementation-defined format.
    #[serde(rename = "instruction")]
    pub instruction: String,

    /// Name of the symbol that corresponds with the location of this instruction, if any.
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,

    /// Source location that corresponds to this instruction, if any.
    ///
    /// Should always be set (if available) on the first instruction returned,
    ///
    /// but can be omitted afterwards if this instruction maps to the same source file as the previous instruction.
    #[serde(rename = "location", skip_serializing_if = "Option::is_none")]
    pub location: Option<Source>,

    /// The line within the source location that corresponds to this instruction, if any.
    #[serde(rename = "line", skip_serializing_if = "Option::is_none")]
    pub line: Option<i32>,

    /// The column within the line that corresponds to this instruction, if any.
    #[serde(rename = "column", skip_serializing_if = "Option::is_none")]
    pub column: Option<i32>,

    /// The end line of the range that corresponds to this instruction, if any.
    #[serde(rename = "endLine", skip_serializing_if = "Option::is_none")]
    pub end_line: Option<i32>,

    /// The end column of the range that corresponds to this instruction, if any.
    #[serde(rename = "endColumn", skip_serializing_if = "Option::is_none")]
    pub end_column: Option<i32>,
}

/// This enumeration defines all possible conditions when a thrown exception should result in a break.
///
/// never: never breaks,
///
/// always: always breaks,
///
/// unhandled: breaks when exception unhandled,
///
/// userUnhandled: breaks if the exception is not handled by user code.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum ExceptionBreakMode {
    #[serde(rename = "never")]
    Never,

    #[serde(rename = "always")]
    Always,

    #[serde(rename = "unhandled")]
    Unhandled,

    #[serde(rename = "userUnhandled")]
    UserUnhandled,
}

/// An ExceptionBreakpointsFilter is shown in the UI as an filter option for configuring how exceptions are dealt with.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ExceptionBreakpointsFilter {
    /// The internal ID of the filter option. This value is passed to the 'setExceptionBreakpoints' request.
    #[serde(rename = "filter")]
    pub filter: String,

    /// The name of the filter option. This will be shown in the UI.
    #[serde(rename = "label")]
    pub label: String,

    /// An optional help text providing additional information about the exception filter. This string is typically shown as a hover and must be translated.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Initial value of the filter option. If not specified a value 'false' is assumed.
    #[serde(rename = "default", default, skip_serializing_if = "eq_default")]
    pub default: bool,

    /// Controls whether a condition can be specified for this filter option. If false or missing, a condition can not be set.
    #[serde(
        rename = "supportsCondition",
        default,
        skip_serializing_if = "eq_default"
    )]
    pub supports_condition: bool,

    /// An optional help text providing information about the condition. This string is shown as the placeholder text for a text box and must be translated.
    #[serde(
        rename = "conditionDescription",
        skip_serializing_if = "Option::is_none"
    )]
    pub condition_description: Option<String>,
}

/// Detailed information about an exception that has occurred.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ExceptionDetails {
    /// Message contained in the exception.
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,

    /// Short type name of the exception object.
    #[serde(rename = "typeName", skip_serializing_if = "Option::is_none")]
    pub type_name: Option<String>,

    /// Fully-qualified type name of the exception object.
    #[serde(rename = "fullTypeName", skip_serializing_if = "Option::is_none")]
    pub full_type_name: Option<String>,

    /// Optional expression that can be evaluated in the current scope to obtain the exception object.
    #[serde(rename = "evaluateName", skip_serializing_if = "Option::is_none")]
    pub evaluate_name: Option<String>,

    /// Stack trace at the time the exception was thrown.
    #[serde(rename = "stackTrace", skip_serializing_if = "Option::is_none")]
    pub stack_trace: Option<String>,

    /// Details of the exception contained by this exception, if any.
    #[serde(
        rename = "innerException",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub inner_exception: Vec<ExceptionDetails>,
}

/// An ExceptionFilterOptions is used to specify an exception filter together with a condition for the setExceptionsFilter request.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ExceptionFilterOptions {
    /// ID of an exception filter returned by the 'exceptionBreakpointFilters' capability.
    #[serde(rename = "filterId")]
    pub filter_id: String,

    /// An optional expression for conditional exceptions.
    ///
    /// The exception will break into the debugger if the result of the condition is true.
    #[serde(rename = "condition", skip_serializing_if = "Option::is_none")]
    pub condition: Option<String>,
}

/// An ExceptionOptions assigns configuration options to a set of exceptions.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ExceptionOptions {
    /// A path that selects a single or multiple exceptions in a tree. If 'path' is missing, the whole tree is selected.
    ///
    /// By convention the first segment of the path is a category that is used to group exceptions in the UI.
    #[serde(rename = "path", default, skip_serializing_if = "Vec::is_empty")]
    pub path: Vec<ExceptionPathSegment>,

    /// Condition when a thrown exception should result in a break.
    #[serde(rename = "breakMode")]
    pub break_mode: ExceptionBreakMode,
}

/// An ExceptionPathSegment represents a segment in a path that is used to match leafs or nodes in a tree of exceptions.
///
/// If a segment consists of more than one name, it matches the names provided if 'negate' is false or missing or
///
/// it matches anything except the names provided if 'negate' is true.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ExceptionPathSegment {
    /// If false or missing this segment matches the names provided, otherwise it matches anything except the names provided.
    #[serde(rename = "negate", default, skip_serializing_if = "eq_default")]
    pub negate: bool,

    /// Depending on the value of 'negate' the names that should match or not match.
    #[serde(rename = "names")]
    pub names: Vec<String>,
}

/// Properties of a breakpoint passed to the setFunctionBreakpoints request.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct FunctionBreakpoint {
    /// The name of the function.
    #[serde(rename = "name")]
    pub name: String,

    /// An optional expression for conditional breakpoints.
    ///
    /// It is only honored by a debug adapter if the capability 'supportsConditionalBreakpoints' is true.
    #[serde(rename = "condition", skip_serializing_if = "Option::is_none")]
    pub condition: Option<String>,

    /// An optional expression that controls how many hits of the breakpoint are ignored.
    ///
    /// The backend is expected to interpret the expression as needed.
    ///
    /// The attribute is only honored by a debug adapter if the capability 'supportsHitConditionalBreakpoints' is true.
    #[serde(rename = "hitCondition", skip_serializing_if = "Option::is_none")]
    pub hit_condition: Option<String>,
}

/// A GotoTarget describes a code location that can be used as a target in the 'goto' request.
///
/// The possible goto targets can be determined via the 'gotoTargets' request.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct GotoTarget {
    /// Unique identifier for a goto target. This is used in the goto request.
    #[serde(rename = "id")]
    pub id: i32,

    /// The name of the goto target (shown in the UI).
    #[serde(rename = "label")]
    pub label: String,

    /// The line of the goto target.
    #[serde(rename = "line")]
    pub line: i32,

    /// An optional column of the goto target.
    #[serde(rename = "column", skip_serializing_if = "Option::is_none")]
    pub column: Option<i32>,

    /// An optional end line of the range covered by the goto target.
    #[serde(rename = "endLine", skip_serializing_if = "Option::is_none")]
    pub end_line: Option<i32>,

    /// An optional end column of the range covered by the goto target.
    #[serde(rename = "endColumn", skip_serializing_if = "Option::is_none")]
    pub end_column: Option<i32>,

    /// Optional memory reference for the instruction pointer value represented by this target.
    #[serde(
        rename = "instructionPointerReference",
        skip_serializing_if = "Option::is_none"
    )]
    pub instruction_pointer_reference: Option<String>,
}

/// Properties of a breakpoint passed to the setInstructionBreakpoints request
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct InstructionBreakpoint {
    /// The instruction reference of the breakpoint.
    ///
    /// This should be a memory or instruction pointer reference from an EvaluateResponse, Variable, StackFrame, GotoTarget, or Breakpoint.
    #[serde(rename = "instructionReference")]
    pub instruction_reference: String,

    /// An optional offset from the instruction reference.
    ///
    /// This can be negative.
    #[serde(rename = "offset", skip_serializing_if = "Option::is_none")]
    pub offset: Option<i32>,

    /// An optional expression for conditional breakpoints.
    ///
    /// It is only honored by a debug adapter if the capability 'supportsConditionalBreakpoints' is true.
    #[serde(rename = "condition", skip_serializing_if = "Option::is_none")]
    pub condition: Option<String>,

    /// An optional expression that controls how many hits of the breakpoint are ignored.
    ///
    /// The backend is expected to interpret the expression as needed.
    ///
    /// The attribute is only honored by a debug adapter if the capability 'supportsHitConditionalBreakpoints' is true.
    #[serde(rename = "hitCondition", skip_serializing_if = "Option::is_none")]
    pub hit_condition: Option<String>,
}

/// Logical areas that can be invalidated by the 'invalidated' event.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum InvalidatedAreas {
    /// All previously fetched data has become invalid and needs to be refetched.
    #[serde(rename = "all")]
    All,

    /// Previously fetched stack related data has become invalid and needs to be refetched.
    #[serde(rename = "stacks")]
    Stacks,

    /// Previously fetched thread related data has become invalid and needs to be refetched.
    #[serde(rename = "threads")]
    Threads,

    /// Previously fetched variable data has become invalid and needs to be refetched.
    #[serde(rename = "variables")]
    Variables,
}

/// A structured message object. Used to return errors from requests.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Message {
    /// Unique identifier for the message.
    #[serde(rename = "id")]
    pub id: i32,

    /// A format string for the message. Embedded variables have the form '{name}'.
    ///
    /// If variable name starts with an underscore character, the variable does not contain user data (PII) and can be safely used for telemetry purposes.
    #[serde(rename = "format")]
    pub format: String,

    /// An object used as a dictionary for looking up the variables in the format string.
    #[serde(
        rename = "variables",
        default,
        skip_serializing_if = "HashMap::is_empty"
    )]
    pub variables: HashMap<String, String>,

    /// If true send to telemetry.
    #[serde(rename = "sendTelemetry", default, skip_serializing_if = "eq_default")]
    pub send_telemetry: bool,

    /// If true show user.
    #[serde(rename = "showUser", default, skip_serializing_if = "eq_default")]
    pub show_user: bool,

    /// An optional url where additional information about this message can be found.
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,

    /// An optional label that is presented to the user as the UI for opening the url.
    #[serde(rename = "urlLabel", skip_serializing_if = "Option::is_none")]
    pub url_label: Option<String>,
}

/// A Module object represents a row in the modules view.
///
/// Two attributes are mandatory: an id identifies a module in the modules view and is used in a ModuleEvent for identifying a module for adding, updating or deleting.
///
/// The name is used to minimally render the module in the UI.
///
///
///
/// Additional attributes can be added to the module. They will show up in the module View if they have a corresponding ColumnDescriptor.
///
///
///
/// To avoid an unnecessary proliferation of additional attributes with similar semantics but different names
///
/// we recommend to re-use attributes from the 'recommended' list below first, and only introduce new attributes if nothing appropriate could be found.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Module {
    /// Unique identifier for the module.
    #[serde(rename = "id")]
    pub id: ModuleId,

    /// A name of the module.
    #[serde(rename = "name")]
    pub name: String,

    /// optional but recommended attributes.
    ///
    /// always try to use these first before introducing additional attributes.
    ///
    ///
    ///
    /// Logical full path to the module. The exact definition is implementation defined, but usually this would be a full path to the on-disk file for the module.
    #[serde(rename = "path", skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,

    /// True if the module is optimized.
    #[serde(rename = "isOptimized", skip_serializing_if = "Option::is_none")]
    pub is_optimized: Option<bool>,

    /// True if the module is considered 'user code' by a debugger that supports 'Just My Code'.
    #[serde(rename = "isUserCode", skip_serializing_if = "Option::is_none")]
    pub is_user_code: Option<bool>,

    /// Version of Module.
    #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,

    /// User understandable description of if symbols were found for the module (ex: 'Symbols Loaded', 'Symbols not found', etc.
    #[serde(rename = "symbolStatus", skip_serializing_if = "Option::is_none")]
    pub symbol_status: Option<String>,

    /// Logical full path to the symbol file. The exact definition is implementation defined.
    #[serde(rename = "symbolFilePath", skip_serializing_if = "Option::is_none")]
    pub symbol_file_path: Option<String>,

    /// Module created or modified.
    #[serde(rename = "dateTimeStamp", skip_serializing_if = "Option::is_none")]
    pub date_time_stamp: Option<String>,

    /// Address range covered by this module.
    #[serde(rename = "addressRange", skip_serializing_if = "Option::is_none")]
    pub address_range: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(untagged)]
pub enum ModuleId {
    Integer(i32),
    String(String),
}

/// The ModulesViewDescriptor is the container for all declarative configuration options of a ModuleView.
///
/// For now it only specifies the columns to be shown in the modules view.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ModulesViewDescriptor {
    #[serde(rename = "columns")]
    pub columns: Vec<ColumnDescriptor>,
}

/// A Scope is a named container for variables. Optionally a scope can map to a source or a range within a source.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Scope {
    /// Name of the scope such as 'Arguments', 'Locals', or 'Registers'. This string is shown in the UI as is and can be translated.
    #[serde(rename = "name")]
    pub name: String,

    /// An optional hint for how to present this scope in the UI. If this attribute is missing, the scope is shown with a generic UI.
    #[serde(rename = "presentationHint", skip_serializing_if = "Option::is_none")]
    pub presentation_hint: Option<ScopePresentationHint>,

    /// The variables of this scope can be retrieved by passing the value of variablesReference to the VariablesRequest.
    #[serde(rename = "variablesReference")]
    pub variables_reference: i32,

    /// The number of named variables in this scope.
    ///
    /// The client can use this optional information to present the variables in a paged UI and fetch them in chunks.
    #[serde(rename = "namedVariables", skip_serializing_if = "Option::is_none")]
    pub named_variables: Option<i32>,

    /// The number of indexed variables in this scope.
    ///
    /// The client can use this optional information to present the variables in a paged UI and fetch them in chunks.
    #[serde(rename = "indexedVariables", skip_serializing_if = "Option::is_none")]
    pub indexed_variables: Option<i32>,

    /// If true, the number of variables in this scope is large or expensive to retrieve.
    #[serde(rename = "expensive")]
    pub expensive: bool,

    /// Optional source for this scope.
    #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
    pub source: Option<Source>,

    /// Optional start line of the range covered by this scope.
    #[serde(rename = "line", skip_serializing_if = "Option::is_none")]
    pub line: Option<i32>,

    /// Optional start column of the range covered by this scope.
    #[serde(rename = "column", skip_serializing_if = "Option::is_none")]
    pub column: Option<i32>,

    /// Optional end line of the range covered by this scope.
    #[serde(rename = "endLine", skip_serializing_if = "Option::is_none")]
    pub end_line: Option<i32>,

    /// Optional end column of the range covered by this scope.
    #[serde(rename = "endColumn", skip_serializing_if = "Option::is_none")]
    pub end_column: Option<i32>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum ScopePresentationHint {
    /// Scope contains method arguments.
    #[serde(rename = "arguments")]
    Arguments,

    /// Scope contains local variables.
    #[serde(rename = "locals")]
    Locals,

    /// Scope contains registers. Only a single 'registers' scope should be returned from a 'scopes' request.
    #[serde(rename = "registers")]
    Registers,
}

/// A Source is a descriptor for source code.
///
/// It is returned from the debug adapter as part of a StackFrame and it is used by clients when specifying breakpoints.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Source {
    /// The short name of the source. Every source returned from the debug adapter has a name.
    ///
    /// When sending a source to the debug adapter this name is optional.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// The path of the source to be shown in the UI.
    ///
    /// It is only used to locate and load the content of the source if no sourceReference is specified (or its value is 0).
    #[serde(rename = "path", skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,

    /// If sourceReference > 0 the contents of the source must be retrieved through the SourceRequest (even if a path is specified).
    ///
    /// A sourceReference is only valid for a session, so it must not be used to persist a source.
    ///
    /// The value should be less than or equal to 2147483647 (2^31-1).
    #[serde(rename = "sourceReference", skip_serializing_if = "Option::is_none")]
    pub source_reference: Option<i32>,

    /// An optional hint for how to present the source in the UI.
    ///
    /// A value of 'deemphasize' can be used to indicate that the source is not available or that it is skipped on stepping.
    #[serde(rename = "presentationHint", skip_serializing_if = "Option::is_none")]
    pub presentation_hint: Option<SourcePresentationHint>,

    /// The (optional) origin of this source: possible values 'internal module', 'inlined content from source map', etc.
    #[serde(rename = "origin", skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,

    /// An optional list of sources that are related to this source. These may be the source that generated this source.
    #[serde(rename = "sources", default, skip_serializing_if = "Vec::is_empty")]
    pub sources: Vec<Source>,

    /// Optional data that a debug adapter might want to loop through the client.
    ///
    /// The client should leave the data intact and persist it across sessions. The client should not interpret the data.
    #[serde(rename = "adapterData", skip_serializing_if = "Option::is_none")]
    pub adapter_data: Option<Value>,

    /// The checksums associated with this file.
    #[serde(rename = "checksums", default, skip_serializing_if = "Vec::is_empty")]
    pub checksums: Vec<Checksum>,
}

/// An optional hint for how to present the source in the UI.
///
/// A value of 'deemphasize' can be used to indicate that the source is not available or that it is skipped on stepping.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum SourcePresentationHint {
    #[serde(rename = "normal")]
    Normal,

    #[serde(rename = "emphasize")]
    Emphasize,

    #[serde(rename = "deemphasize")]
    Deemphasize,
}

/// Properties of a breakpoint or logpoint passed to the setBreakpoints request.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct SourceBreakpoint {
    /// The source line of the breakpoint or logpoint.
    #[serde(rename = "line")]
    pub line: i32,

    /// An optional source column of the breakpoint.
    #[serde(rename = "column", skip_serializing_if = "Option::is_none")]
    pub column: Option<i32>,

    /// An optional expression for conditional breakpoints.
    ///
    /// It is only honored by a debug adapter if the capability 'supportsConditionalBreakpoints' is true.
    #[serde(rename = "condition", skip_serializing_if = "Option::is_none")]
    pub condition: Option<String>,

    /// An optional expression that controls how many hits of the breakpoint are ignored.
    ///
    /// The backend is expected to interpret the expression as needed.
    ///
    /// The attribute is only honored by a debug adapter if the capability 'supportsHitConditionalBreakpoints' is true.
    #[serde(rename = "hitCondition", skip_serializing_if = "Option::is_none")]
    pub hit_condition: Option<String>,

    /// If this attribute exists and is non-empty, the backend must not 'break' (stop)
    ///
    /// but log the message instead. Expressions within {} are interpolated.
    ///
    /// The attribute is only honored by a debug adapter if the capability 'supportsLogPoints' is true.
    #[serde(rename = "logMessage", skip_serializing_if = "Option::is_none")]
    pub log_message: Option<String>,
}

/// A Stackframe contains the source location.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct StackFrame {
    /// An identifier for the stack frame. It must be unique across all threads.
    ///
    /// This id can be used to retrieve the scopes of the frame with the 'scopesRequest' or to restart the execution of a stackframe.
    #[serde(rename = "id")]
    pub id: i32,

    /// The name of the stack frame, typically a method name.
    #[serde(rename = "name")]
    pub name: String,

    /// The optional source of the frame.
    #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
    pub source: Option<Source>,

    /// The line within the file of the frame. If source is null or doesn't exist, line is 0 and must be ignored.
    #[serde(rename = "line")]
    pub line: i32,

    /// The column within the line. If source is null or doesn't exist, column is 0 and must be ignored.
    #[serde(rename = "column")]
    pub column: i32,

    /// An optional end line of the range covered by the stack frame.
    #[serde(rename = "endLine", skip_serializing_if = "Option::is_none")]
    pub end_line: Option<i32>,

    /// An optional end column of the range covered by the stack frame.
    #[serde(rename = "endColumn", skip_serializing_if = "Option::is_none")]
    pub end_column: Option<i32>,

    /// Indicates whether this frame can be restarted with the 'restart' request. Clients should only use this if the debug adapter supports the 'restart' request (capability 'supportsRestartRequest' is true).
    #[serde(rename = "canRestart", skip_serializing_if = "Option::is_none")]
    pub can_restart: Option<bool>,

    /// Optional memory reference for the current instruction pointer in this frame.
    #[serde(
        rename = "instructionPointerReference",
        skip_serializing_if = "Option::is_none"
    )]
    pub instruction_pointer_reference: Option<String>,

    /// The module associated with this frame, if any.
    #[serde(rename = "moduleId", skip_serializing_if = "Option::is_none")]
    pub module_id: Option<ModuleId>,

    /// An optional hint for how to present this frame in the UI.
    ///
    /// A value of 'label' can be used to indicate that the frame is an artificial frame that is used as a visual label or separator. A value of 'subtle' can be used to change the appearance of a frame in a 'subtle' way.
    #[serde(rename = "presentationHint", skip_serializing_if = "Option::is_none")]
    pub presentation_hint: Option<StackFramePresentationHint>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum StackFramePresentationHint {
    #[serde(rename = "normal")]
    Normal,

    #[serde(rename = "label")]
    Label,

    #[serde(rename = "subtle")]
    Subtle,
}

/// Provides formatting information for a stack frame.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct StackFrameFormat {
    /// Displays parameters for the stack frame.
    #[serde(rename = "parameters", skip_serializing_if = "Option::is_none")]
    pub parameters: Option<bool>,

    /// Displays the types of parameters for the stack frame.
    #[serde(rename = "parameterTypes", skip_serializing_if = "Option::is_none")]
    pub parameter_types: Option<bool>,

    /// Displays the names of parameters for the stack frame.
    #[serde(rename = "parameterNames", skip_serializing_if = "Option::is_none")]
    pub parameter_names: Option<bool>,

    /// Displays the values of parameters for the stack frame.
    #[serde(rename = "parameterValues", skip_serializing_if = "Option::is_none")]
    pub parameter_values: Option<bool>,

    /// Displays the line number of the stack frame.
    #[serde(rename = "line", skip_serializing_if = "Option::is_none")]
    pub line: Option<bool>,

    /// Displays the module of the stack frame.
    #[serde(rename = "module", skip_serializing_if = "Option::is_none")]
    pub module: Option<bool>,

    /// Includes all stack frames, including those the debug adapter might otherwise hide.
    #[serde(rename = "includeAll", skip_serializing_if = "Option::is_none")]
    pub include_all: Option<bool>,
}

/// A StepInTarget can be used in the 'stepIn' request and determines into which single target the stepIn request should step.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct StepInTarget {
    /// Unique identifier for a stepIn target.
    #[serde(rename = "id")]
    pub id: i32,

    /// The name of the stepIn target (shown in the UI).
    #[serde(rename = "label")]
    pub label: String,
}

/// The granularity of one 'step' in the stepping requests 'next', 'stepIn', 'stepOut', and 'stepBack'.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum SteppingGranularity {
    /// The step should allow the program to run until the current statement has finished executing.
    ///
    /// The meaning of a statement is determined by the adapter and it may be considered equivalent to a line.
    ///
    /// For example 'for(int i = 0; i < 10; i++) could be considered to have 3 statements 'int i = 0', 'i < 10', and 'i++'.
    #[serde(rename = "statement")]
    Statement,

    /// The step should allow the program to run until the current source line has executed.
    #[serde(rename = "line")]
    Line,

    /// The step should allow one instruction to execute (e.g. one x86 instruction).
    #[serde(rename = "instruction")]
    Instruction,
}

impl Default for SteppingGranularity {
    fn default() -> Self {
        SteppingGranularity::Statement
    }
}

/// A Thread
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Thread {
    /// Unique identifier for the thread.
    #[serde(rename = "id")]
    pub id: i32,

    /// A name of the thread.
    #[serde(rename = "name")]
    pub name: String,
}

/// Provides formatting information for a value.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ValueFormat {
    /// Display the value in hex.
    #[serde(rename = "hex", skip_serializing_if = "Option::is_none")]
    pub hex: Option<bool>,
}

/// A Variable is a name/value pair.
///
/// Optionally a variable can have a 'type' that is shown if space permits or when hovering over the variable's name.
///
/// An optional 'kind' is used to render additional properties of the variable, e.g. different icons can be used to indicate that a variable is public or private.
///
/// If the value is structured (has children), a handle is provided to retrieve the children with the VariablesRequest.
///
/// If the number of named or indexed children is large, the numbers should be returned via the optional 'namedVariables' and 'indexedVariables' attributes.
///
/// The client can use this optional information to present the children in a paged UI and fetch them in chunks.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Variable {
    /// The variable's name.
    #[serde(rename = "name")]
    pub name: String,

    /// The variable's value. This can be a multi-line text, e.g. for a function the body of a function.
    #[serde(rename = "value")]
    pub value: String,

    /// The type of the variable's value. Typically shown in the UI when hovering over the value.
    ///
    /// This attribute should only be returned by a debug adapter if the client has passed the value true for the 'supportsVariableType' capability of the 'initialize' request.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,

    /// Properties of a variable that can be used to determine how to render the variable in the UI.
    #[serde(rename = "presentationHint", skip_serializing_if = "Option::is_none")]
    pub presentation_hint: Option<VariablePresentationHint>,

    /// Optional evaluatable name of this variable which can be passed to the 'EvaluateRequest' to fetch the variable's value.
    #[serde(rename = "evaluateName", skip_serializing_if = "Option::is_none")]
    pub evaluate_name: Option<String>,

    /// If variablesReference is > 0, the variable is structured and its children can be retrieved by passing variablesReference to the VariablesRequest.
    #[serde(rename = "variablesReference")]
    pub variables_reference: i32,

    /// The number of named child variables.
    ///
    /// The client can use this optional information to present the children in a paged UI and fetch them in chunks.
    #[serde(rename = "namedVariables", skip_serializing_if = "Option::is_none")]
    pub named_variables: Option<i32>,

    /// The number of indexed child variables.
    ///
    /// The client can use this optional information to present the children in a paged UI and fetch them in chunks.
    #[serde(rename = "indexedVariables", skip_serializing_if = "Option::is_none")]
    pub indexed_variables: Option<i32>,

    /// Optional memory reference for the variable if the variable represents executable code, such as a function pointer.
    ///
    /// This attribute is only required if the client has passed the value true for the 'supportsMemoryReferences' capability of the 'initialize' request.
    #[serde(rename = "memoryReference", skip_serializing_if = "Option::is_none")]
    pub memory_reference: Option<String>,
}

/// Optional properties of a variable that can be used to determine how to render the variable in the UI.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct VariablePresentationHint {
    /// The kind of variable. Before introducing additional values, try to use the listed values.
    #[serde(rename = "kind", skip_serializing_if = "Option::is_none")]
    pub kind: Option<VariableKind>,

    /// Set of attributes represented as an array of strings. Before introducing additional values, try to use the listed values.
    #[serde(rename = "attributes", default, skip_serializing_if = "Vec::is_empty")]
    pub attributes: Vec<VariableAttribute>,

    /// Visibility of variable. Before introducing additional values, try to use the listed values.
    #[serde(rename = "visibility", skip_serializing_if = "Option::is_none")]
    pub visibility: Option<VariableVisibility>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum VariableKind {
    /// Indicates that the object is a property.
    #[serde(rename = "property")]
    Property,

    /// Indicates that the object is a method.
    #[serde(rename = "method")]
    Method,

    /// Indicates that the object is a class.
    #[serde(rename = "class")]
    Class,

    /// Indicates that the object is data.
    #[serde(rename = "data")]
    Data,

    /// Indicates that the object is an event.
    #[serde(rename = "event")]
    Event,

    /// Indicates that the object is a base class.
    #[serde(rename = "baseClass")]
    BaseClass,

    /// Indicates that the object is an inner class.
    #[serde(rename = "innerClass")]
    InnerClass,

    /// Indicates that the object is an interface.
    #[serde(rename = "interface")]
    Interface,

    /// Indicates that the object is the most derived class.
    #[serde(rename = "mostDerivedClass")]
    MostDerivedClass,

    /// Indicates that the object is virtual, that means it is a synthetic object introducedby the
    ///
    /// adapter for rendering purposes, e.g. an index range for large arrays.
    #[serde(rename = "virtual")]
    Virtual,

    /// Deprecated: Indicates that a data breakpoint is registered for the object. The 'hasDataBreakpoint' attribute should generally be used instead.
    #[serde(rename = "dataBreakpoint")]
    DataBreakpoint,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum VariableAttribute {
    /// Indicates that the object is static.
    #[serde(rename = "static")]
    Static,

    /// Indicates that the object is a constant.
    #[serde(rename = "constant")]
    Constant,

    /// Indicates that the object is read only.
    #[serde(rename = "readOnly")]
    ReadOnly,

    /// Indicates that the object is a raw string.
    #[serde(rename = "rawString")]
    RawString,

    /// Indicates that the object can have an Object ID created for it.
    #[serde(rename = "hasObjectId")]
    HasObjectId,

    /// Indicates that the object has an Object ID associated with it.
    #[serde(rename = "canHaveObjectId")]
    CanHaveObjectId,

    /// Indicates that the evaluation had side effects.
    #[serde(rename = "hasSideEffects")]
    HasSideEffects,

    /// Indicates that the object has its value tracked by a data breakpoint.
    #[serde(rename = "hasDataBreakpoint")]
    HasDataBreakpoint,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum VariableVisibility {
    #[serde(rename = "public")]
    Public,

    #[serde(rename = "private")]
    Private,

    #[serde(rename = "protected")]
    Protected,

    #[serde(rename = "internal")]
    Internal,

    #[serde(rename = "final")]
    Final,
}
