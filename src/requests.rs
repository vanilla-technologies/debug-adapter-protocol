use crate::utils::{eq_default, true_};
use serde::{Deserialize, Serialize};

/// Object containing arguments for the command.
#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase", tag = "command", content = "arguments")]
pub enum RequestCommand {
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
pub enum PathFormat {
    Path,
    URI,
}

impl Default for PathFormat {
    fn default() -> Self {
        PathFormat::Path
    }
}
