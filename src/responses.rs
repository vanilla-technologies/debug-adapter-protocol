use crate::types::Capabilities;
use serde::{
    de::{Error, Unexpected},
    Deserialize, Deserializer, Serialize, Serializer,
};
use serde_json::{Number, Value};

#[derive(Debug, PartialEq)]
pub enum ResponseType {
    Success {
        /// The command requested.
        command: ResponseCommand,
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

        /// An optional, structured error message.
        body: Option<Value>,
    },
}

/// Contains request result if success is true and optional error details if
/// success is false.
#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase", tag = "command", content = "body")]
pub enum ResponseCommand {
    Initialize(Capabilities),
}

// Workaround from https://stackoverflow.com/a/65576570
// for https://github.com/serde-rs/serde/issues/745
impl<'de> Deserialize<'de> for ResponseType {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let value = Value::deserialize(d)?;

        let success = value
            .get("success")
            .ok_or_else(|| Error::missing_field("success"))?
            .as_bool()
            .ok_or_else(|| Error::invalid_type(unexpected_value(&value), &"success bool"))?;

        Ok(if success {
            let command =
                Deserialize::deserialize(value).map_err(|e| Error::custom(e.to_string()))?;
            ResponseType::Success { command }
        } else {
            #[derive(Debug, Deserialize, PartialEq, Serialize)]
            struct ResponseTypeError {
                command: String,
                message: String,
                body: Option<Value>,
            }

            let response =
                ResponseTypeError::deserialize(value).map_err(|e| Error::custom(e.to_string()))?;
            ResponseType::Error {
                command: response.command,
                message: response.message,
                body: response.body,
            }
        })
    }
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

impl Serialize for ResponseType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        #[derive(Serialize)]
        #[serde(untagged)]
        enum ResponseTypeContent<'l> {
            Success {
                #[serde(flatten)]
                command: &'l ResponseCommand,
            },
            Error {
                command: &'l String,
                message: &'l String,
                body: &'l Option<Value>,
            },
        }

        #[derive(Serialize)]
        struct TaggedResponseType<'l> {
            success: bool,
            #[serde(flatten)]
            content: ResponseTypeContent<'l>,
        }

        let serializable = match self {
            ResponseType::Success { command } => TaggedResponseType {
                success: true,
                content: ResponseTypeContent::Success { command },
            },
            ResponseType::Error {
                command,
                message,
                body,
            } => TaggedResponseType {
                success: false,
                content: ResponseTypeContent::Error {
                    command,
                    message,
                    body,
                },
            },
        };
        serializable.serialize(serializer)
    }
}
