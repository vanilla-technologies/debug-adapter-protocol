use crate::utils::{eq_default, true_};
use serde::{Deserialize, Serialize};

/// Object containing arguments for the command.
#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase", tag = "command", content = "arguments")]
pub enum RequestCommand {
    Attach(AttachRequestArguments),
    Initialize(InitializeRequestArguments),
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AttachRequestArguments {
    /// Optional data from the previous, restarted session.
    /// The data is sent as the 'restart' attribute of the 'terminated' event.
    /// The client should leave the data intact.
    #[serde(skip_serializing_if = "Option::is_none")]
    __restart: Option<String>,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InitializeRequestArguments {
    /// The ID of the (frontend) client using this adapter.
    #[serde(skip_serializing_if = "Option::is_none", rename = "clientID")]
    pub client_id: Option<String>,

    /// The human readable name of the (frontend) client using this adapter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_name: Option<String>,

    /// The ID of the debug adapter.
    #[serde(rename = "adapterID")]
    pub adapter_id: String,

    /// The ISO-639 locale of the (frontend) client using this adapter, e.g. en-US
    /// or de-CH.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,

    /// If true all line numbers are 1-based (default).
    #[serde(default = "true_")]
    pub lines_start_at1: bool,

    /// If true all column numbers are 1-based (default).
    #[serde(default = "true_")]
    pub columns_start_at1: bool,

    /// Determines in what format paths are specified. The default is 'path', which
    /// is the native format.
    /// Values: 'path', 'uri', etc.
    #[serde(default)]
    pub path_format: PathFormat,

    /// Client supports the optional type attribute for variables.
    #[serde(default, skip_serializing_if = "eq_default")]
    pub supports_variable_type: bool,

    /// Client supports the paging of variables.
    #[serde(default, skip_serializing_if = "eq_default")]
    pub supports_variable_paging: bool,

    /// Client supports the runInTerminal request.
    #[serde(default, skip_serializing_if = "eq_default")]
    pub supports_run_in_terminal_request: bool,

    /// Client supports memory references.
    #[serde(default, skip_serializing_if = "eq_default")]
    pub supports_memory_references: bool,

    /// Client supports progress reporting.
    #[serde(default, skip_serializing_if = "eq_default")]
    pub supports_progress_reporting: bool,

    /// Client supports the invalidated event.
    #[serde(default, skip_serializing_if = "eq_default")]
    pub supports_invalidated_event: bool,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
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
