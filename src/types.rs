use crate::utils::eq_default;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Capabilities {
    /// The debug adapter supports the 'configurationDone' request.
    #[serde(default, skip_serializing_if = "eq_default")]
    pub supports_configuration_done_request: bool,

    /// The debug adapter supports function breakpoints.
    #[serde(default, skip_serializing_if = "eq_default")]
    pub supports_function_breakpoints: bool,

    /// The debug adapter supports conditional breakpoints.
    #[serde(default, skip_serializing_if = "eq_default")]
    pub supports_conditional_breakpoints: bool,

    /// The debug adapter supports breakpoints that break execution after a
    /// specified number of hits.
    #[serde(default, skip_serializing_if = "eq_default")]
    pub supports_hit_conditional_breakpoints: bool,

    /// The debug adapter supports a (side effect free) evaluate request for data
    /// hovers.
    #[serde(default, skip_serializing_if = "eq_default")]
    pub supports_evaluate_for_hovers: bool,

    //   /// Available exception filter options for the 'setExceptionBreakpoints'
    //    /// request.
    //   exceptionBreakpointFilters?: ExceptionBreakpointsFilter[];
    /// The debug adapter supports stepping back via the 'stepBack' and
    /// 'reverseContinue' requests.
    #[serde(default, skip_serializing_if = "eq_default")]
    pub supports_step_back: bool,

    /// The debug adapter supports setting a variable to a value.
    #[serde(default, skip_serializing_if = "eq_default")]
    pub supports_set_variable: bool,

    /// The debug adapter supports restarting a frame.
    #[serde(default, skip_serializing_if = "eq_default")]
    pub supports_restart_frame: bool,

    /// The debug adapter supports the 'gotoTargets' request.
    #[serde(default, skip_serializing_if = "eq_default")]
    pub supports_goto_targets_request: bool,

    /// The debug adapter supports the 'stepInTargets' request.
    #[serde(default, skip_serializing_if = "eq_default")]
    pub supports_step_in_targets_request: bool,

    /// The debug adapter supports the 'completions' request.
    #[serde(default, skip_serializing_if = "eq_default")]
    pub supports_completions_request: bool,

    //   /// The set of characters that should trigger completion in a REPL. If not
    //    /// specified, the UI should assume the '.' character.
    //   completionTriggerCharacters?: string[];
    /// The debug adapter supports the 'modules' request.
    #[serde(default, skip_serializing_if = "eq_default")]
    pub supports_modules_requeststst: bool,

    //   /// The set of additional module information exposed by the debug adapter.
    //   additionalModuleColumns?: ColumnDescriptor[];

    //  /// Checksum algorithms supported by the debug adapter.
    //   supportedChecksumAlgorithms?: ChecksumAlgorithm[];
    /// The debug adapter supports the 'restart' request. In this case a client
    /// should not implement 'restart' by terminating and relaunching the adapter
    /// but by calling the RestartRequest.
    #[serde(default, skip_serializing_if = "eq_default")]
    pub supports_restart_request: bool,

    /// The debug adapter supports 'exceptionOptions' on the
    /// setExceptionBreakpoints request.
    #[serde(default, skip_serializing_if = "eq_default")]
    pub supports_exception_options: bool,

    /// The debug adapter supports a 'format' attribute on the stackTraceRequest,
    /// variablesRequest, and evaluateRequest.
    #[serde(default, skip_serializing_if = "eq_default")]
    pub supports_value_formatting_options: bool,

    /// The debug adapter supports the 'exceptionInfo' request.
    #[serde(default, skip_serializing_if = "eq_default")]
    pub supports_exception_info_request: bool,

    /// The debug adapter supports the 'terminateDebuggee' attribute on the
    /// 'disconnect' request.
    #[serde(default, skip_serializing_if = "eq_default")]
    pub support_terminate_debuggee: bool,

    /// The debug adapter supports the 'suspendDebuggee' attribute on the
    /// 'disconnect' request.
    #[serde(default, skip_serializing_if = "eq_default")]
    pub support_suspend_debuggee: bool,

    /// The debug adapter supports the delayed loading of parts of the stack, which
    /// requires that both the 'startFrame' and 'levels' arguments and an optional
    /// 'totalFrames' result of the 'StackTrace' request are supported.
    #[serde(default, skip_serializing_if = "eq_default")]
    pub supports_delayed_stack_trace_loading: bool,

    /// The debug adapter supports the 'loadedSources' request.
    #[serde(default, skip_serializing_if = "eq_default")]
    pub supports_loaded_sources_request: bool,

    /// The debug adapter supports logpoints by interpreting the 'logMessage'
    /// attribute of the SourceBreakpoint.
    #[serde(default, skip_serializing_if = "eq_default")]
    pub supports_log_points: bool,

    /// The debug adapter supports the 'terminateThreads' request.
    #[serde(default, skip_serializing_if = "eq_default")]
    pub supports_terminate_threads_request: bool,

    /// The debug adapter supports the 'setExpression' request.
    #[serde(default, skip_serializing_if = "eq_default")]
    pub supports_set_expression: bool,

    /// The debug adapter supports the 'terminate' request.
    #[serde(default, skip_serializing_if = "eq_default")]
    pub supports_terminate_request: bool,

    /// The debug adapter supports data breakpoints.
    #[serde(default, skip_serializing_if = "eq_default")]
    pub supports_data_breakpoints: bool,

    /// The debug adapter supports the 'readMemory' request.
    #[serde(default, skip_serializing_if = "eq_default")]
    pub supports_read_memory_request: bool,

    /// The debug adapter supports the 'disassemble' request.
    #[serde(default, skip_serializing_if = "eq_default")]
    pub supports_disassemble_request: bool,

    /// The debug adapter supports the 'cancel' request.
    #[serde(default, skip_serializing_if = "eq_default")]
    pub supports_cancel_request: bool,

    /// The debug adapter supports the 'breakpointLocations' request.
    #[serde(default, skip_serializing_if = "eq_default")]
    pub supports_breakpoint_locations_request: bool,

    /// The debug adapter supports the 'clipboard' context value in the 'evaluate'
    /// request.
    #[serde(default, skip_serializing_if = "eq_default")]
    pub supports_clipboard_context: bool,

    /// The debug adapter supports stepping granularities (argument 'granularity')
    /// for the stepping requests.
    #[serde(default, skip_serializing_if = "eq_default")]
    pub supports_stepping_granularity: bool,

    /// The debug adapter supports adding breakpoints based on instruction
    /// references.
    #[serde(default, skip_serializing_if = "eq_default")]
    pub supports_instruction_breakpoints: bool,

    /// The debug adapter supports 'filterOptions' as an argument on the
    /// 'setExceptionBreakpoints' request.
    #[serde(default, skip_serializing_if = "eq_default")]
    pub supports_exception_filter_options: bool,
}
