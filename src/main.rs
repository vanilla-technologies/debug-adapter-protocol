use convert_case::{Case, Casing};
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use std::{
    collections::{BTreeMap, HashSet},
    fs::File,
    io::{BufWriter, Write},
};

fn main() -> Result<(), serde_json::Error> {
    let json = include_str!("debugAdapterProtocol.json");
    let schema = serde_json::from_str::<Schema>(json)?;

    let definitions = &schema.definitions;
    let mut events = Vec::new();
    let mut event_types = Vec::new();
    let mut responses = Vec::new();
    let mut response_types = Vec::new();
    let mut requests = Vec::new();
    let mut request_types = Vec::new();
    let mut types = Vec::new();
    let mut todos = Vec::new();
    for (key, value) in definitions {
        let r = single_definition(key, value, &definitions);
        if let Some(v) = r {
            for (category, code) in v {
                match category {
                    Category::Event => events.push(code),
                    Category::EventType => event_types.push(code),
                    Category::Response => responses.push(code),
                    Category::ResponseType => response_types.push(code),
                    Category::Request => requests.push(code),
                    Category::RequestType => request_types.push(code),
                    Category::Type => {
                        if !code.is_empty() {
                            types.push(code)
                        }
                    }
                }
            }
        } else {
            todos.push(key);
        }
        // println!("{}", key)
    }

    let file = File::create("src/generated_events.rs").unwrap();
    let mut writer = BufWriter::new(file);
    writer
        .write("/// Event-specific information.\n".as_bytes())
        .unwrap();
    writer
        .write("#[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]\n".as_bytes())
        .unwrap();
    writer
        .write(
            "#[serde(rename_all = \"camelCase\", tag = \"event\", content = \"body\")]\n"
                .as_bytes(),
        )
        .unwrap();
    writer.write("pub enum Event {\n".as_bytes()).unwrap();
    for mut event in events {
        event.push('\n');
        writer.write(event.as_bytes()).unwrap();
    }
    writer.write("}\n\n".as_bytes()).unwrap();
    for event_type in event_types {
        writer.write(event_type.as_bytes()).unwrap();
    }

    let file = File::create("src/generated_requests.rs").unwrap();
    let mut writer = BufWriter::new(file);
    writer
        .write("/// Object containing arguments for the command.\n".as_bytes())
        .unwrap();
    writer
        .write("#[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]\n".as_bytes())
        .unwrap();
    writer
        .write(
            "#[serde(rename_all = \"camelCase\", tag = \"command\", content = \"arguments\")]\n"
                .as_bytes(),
        )
        .unwrap();
    writer
        .write("pub enum RequestCommand {\n".as_bytes())
        .unwrap();
    for mut request in requests {
        request.push('\n');
        writer.write(request.as_bytes()).unwrap();
    }
    writer.write("}\n\n".as_bytes()).unwrap();
    for request_type in request_types {
        writer.write(request_type.as_bytes()).unwrap();
    }

    let file = File::create("src/generated_responses.rs").unwrap();
    let mut writer = BufWriter::new(file);
    writer
        .write("/// Contains request result if success is true and optional error details if success is false.\n".as_bytes())
        .unwrap();
    writer
        .write("#[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]\n".as_bytes())
        .unwrap();
    writer
        .write(
            "#[serde(rename_all = \"camelCase\", tag = \"command\", content = \"body\")]\n"
                .as_bytes(),
        )
        .unwrap();
    writer
        .write("pub enum ResponseCommand {\n".as_bytes())
        .unwrap();
    for mut response in responses {
        response.push('\n');
        writer.write(response.as_bytes()).unwrap();
    }
    writer.write("}\n\n".as_bytes()).unwrap();
    for response_type in response_types {
        writer.write(response_type.as_bytes()).unwrap();
    }

    let file = File::create("src/generated_types.rs").unwrap();
    let mut writer = BufWriter::new(file);
    writer
        .write("use serde::{Deserialize, Serialize};\n\n".as_bytes())
        .unwrap();
    for mut type_ in types {
        type_.push('\n');
        writer.write(type_.as_bytes()).unwrap();
    }
    // for todo in todos {
    //     println!("// TODO: {}", todo)
    // }

    Ok(())
}

enum Category {
    Request,
    RequestType,
    Response,
    ResponseType,
    Event,
    EventType,
    Type,
}

fn single_definition(
    key: &str,
    value: &Type,
    definitions: &BTreeMap<String, Type>,
) -> Option<Vec<(Category, String)>> {
    let definition = value.resolve_definition(definitions);
    let parent_name = match value {
        Type::AllOf { all_of, .. } => match all_of.split_first() {
            Some((Type::Reference { reference, .. }, tail)) => reference,
            _ => "",
        },
        _ => "",
    };

    match definition {
        TypeDefinition::Object {
            description,
            properties,
            required,
        } => {
            let category = match parent_name {
                "#/definitions/Event" => Category::Event,
                "#/definitions/Request" => Category::Request,
                "#/definitions/Response" => Category::Response,
                _ => Category::Type,
            };
            let (tag, content_name) = match category {
                Category::Request => {
                    let tag = get_tag(properties, "command");
                    (tag, "arguments")
                }
                Category::Response => {
                    let tag = description
                        .as_ref()?
                        .strip_prefix("Response to '")?
                        .split_once('\'')?
                        .0;
                    (tag, "body")
                }
                Category::Event => {
                    let tag = get_tag(properties, "event");
                    (tag, "body")
                }
                Category::Type
                    if key != "ProtocolMessage"
                        && key != "Request"
                        && key != "Event"
                        && key != "Response"
                        && !key.ends_with("Arguments") =>
                {
                    return Some(vec![(Category::Type, single_type(key, definition))])
                }
                _ => return Some(vec![(category, "".to_string())]),
            };

            let mut code = String::new();
            if let Some(description) = &description {
                code.push_str(&transform_description(&description));
                code.push('\n');
            }
            let variant_name = capitalize(tag);
            code.push_str(&variant_name);

            let mut result = Vec::new();

            if let Some(content) = properties.get(content_name) {
                let category_suffix = match category {
                    Category::Request => "Request",
                    Category::RequestType => "",
                    Category::Response => "Response",
                    Category::ResponseType => "",
                    Category::Event => "Event",
                    Category::EventType => "",
                    Category::Type => "",
                };

                let mut content_definition = None;
                match content {
                    Type::Reference {
                        reference,
                        description,
                    } => {
                        if let Category::Request = category {
                            content_definition = Some(content.resolve_definition(definitions));
                        } else {
                            code.push('(');
                            code.push_str(&reference.strip_prefix("#/definitions/").unwrap());
                            code.push(')');
                        }
                    }
                    Type::Definition(content_def) => content_definition = Some(content_def),
                    _ => todo!(),
                };
                if let Some(content_definition) = content_definition {
                    code.push('(');
                    code.push_str(&variant_name);
                    code.push_str(category_suffix);
                    code.push_str(&capitalize(content_name));
                    code.push(')');
                    let mut content_code = String::new();
                    content_code.push_str(
                        "#[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]\npub struct ",
                    );
                    content_code.push_str(&variant_name);
                    content_code.push_str(category_suffix);
                    content_code.push_str(&capitalize(content_name));
                    content_code.push_str(" {");

                    match content_definition {
                        TypeDefinition::Object {
                            description,
                            properties,
                            required,
                        } => {
                            content_code.push_str(&properties_to_string(properties, required));
                        }
                        _ => todo!(),
                    }
                    content_code.push_str("}\n\n");
                    let type_category = match category {
                        Category::Request => Category::RequestType,
                        Category::Response => Category::ResponseType,
                        Category::Event => Category::EventType,
                        _ => todo!(),
                    };
                    result.push((type_category, content_code));
                }
            }
            code.push_str(",\n");
            result.push((category, code));
            Some(result)
        }
        TypeDefinition::String {
            description,
            enum_values,
            enum_descriptions,
        } => Some(vec![(Category::Type, single_type(key, definition))]),
        _ => todo!(),
    }
}

fn properties_to_string(properties: &IndexMap<String, Type>, required: &HashSet<String>) -> String {
    let mut code = String::new();
    for (field_name, field_def) in properties {
        code.push('\n');
        let required = required.contains(field_name);
        if let Some(description) = field_def.description() {
            code.push_str("  ");
            code.push_str(&transform_description(description).replace('\n', "\n  "));
            code.push('\n');
        }
        code.push_str("  #[serde(rename=\"");
        code.push_str(field_name);
        code.push_str("\"");
        if !required {
            code.push_str(", skip_serializing_if = \"Option::is_none\"");
        }
        code.push_str(")]\n  pub ");
        code.push_str(&field_name.to_case(Case::Snake));
        code.push_str(": ");
        if !required {
            code.push_str("Option<");
        }
        code.push_str(&field_def.type_str());
        if !required {
            code.push('>');
        }
        code.push_str(",\n");
    }
    code
}

fn single_type(name: &str, definition: &TypeDefinition) -> String {
    match definition {
        TypeDefinition::Object {
            description,
            properties,
            required,
        } => {
            let mut code = String::new();
            if let Some(description) = description {
                code.push_str(&transform_description(&description));
                code.push('\n');
            }
            code.push_str("#[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]\npub struct ");
            code.push_str(name);
            code.push_str(" {");
            code.push_str(&properties_to_string(properties, required));
            code.push_str("}\n");
            code
        }
        TypeDefinition::String {
            description,
            enum_values,
            enum_descriptions,
        } => {
            let mut code = String::new();
            if let Some(description) = description {
                code.push_str(&transform_description(&description));
            }
            code.push_str("\n#[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]\npub enum ");
            code.push_str(name);
            code.push_str(" {");

            let mut val_iter = enum_values.iter();
            let mut desc_iter = enum_descriptions.iter();
            while let Some(value) = val_iter.next() {
                if let Some(description) = desc_iter.next() {
                    code.push_str("\n  ");
                    code.push_str(&transform_description(&description).replace('\n', "\n  "));
                }
                code.push_str("\n  #[serde(rename=\"");
                code.push_str(value);
                code.push_str("\")]\n  ");
                code.push_str(&capitalize(&value));
                code.push_str(",\n");
            }
            code.push_str("}\n");
            code
        }
        TypeDefinition::Number { description } => todo!(),
        TypeDefinition::Integer { description } => todo!(),
        TypeDefinition::Boolean { description } => todo!(),
        TypeDefinition::Array { description, items } => todo!(),
        TypeDefinition::Other => todo!(),
    }
}

fn capitalize(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

#[derive(Deserialize)]
struct Schema {
    #[serde(rename = "$schema")]
    url: String,
    title: String,
    description: String,
    definitions: BTreeMap<String, Type>,
}

#[derive(Deserialize, Serialize)]
#[serde(untagged)]
enum Type {
    #[serde(rename_all = "camelCase")]
    AllOf {
        all_of: Vec<Type>,
        description: Option<String>,
    },
    #[serde(rename_all = "camelCase")]
    OneOf {
        one_of: Vec<Type>,
        description: Option<String>,
    },
    Reference {
        #[serde(rename = "$ref")]
        reference: String,
        description: Option<String>,
    },
    Definition(TypeDefinition),
    MultiType {
        #[serde(rename = "type")]
        type_: Vec<String>,
        description: Option<String>,
    },
}

impl Type {
    fn resolve_definition<'l>(
        &'l self,
        definitions: &'l BTreeMap<String, Type>,
    ) -> &'l TypeDefinition {
        match self {
            Type::AllOf { all_of, .. } => match all_of.as_slice() {
                [_, Type::Definition(definition)] => definition,
                _ => panic!("Malformed allOf"),
            },
            Type::OneOf { .. } => todo!(),
            Type::Definition(definition) => definition,
            Type::Reference { reference, .. } => definitions
                .get(reference.strip_prefix("#/definitions/").unwrap())
                .ok_or(format!("Missing reference: '{}'", reference))
                .unwrap()
                .resolve_definition(definitions),
            Type::MultiType { .. } => todo!(),
        }
    }

    fn type_str(&self) -> String {
        match self {
            Type::Reference { reference, .. } => reference
                .strip_prefix("#/definitions/")
                .unwrap()
                .to_string(),
            Type::Definition(definition) => definition.type_str(),
            Type::AllOf { .. } => todo!(),
            Type::OneOf { .. } => "TODO oneOf".to_string(),
            Type::MultiType { type_, .. }
                if type_
                    == &vec![
                        "array".to_string(),
                        "boolean".to_string(),
                        "integer".to_string(),
                        "null".to_string(),
                        "number".to_string(),
                        "object".to_string(),
                        "string".to_string(),
                    ] =>
            {
                "Value".to_string()
            }
            Type::MultiType { type_, .. } => format!("TODO MultiType {:?}", type_),
        }
    }

    fn description(&self) -> Option<&str> {
        match self {
            Type::AllOf { description, .. } => description.as_deref(),
            Type::OneOf { description, .. } => description.as_deref(),
            Type::Reference { description, .. } => description.as_deref(),
            Type::Definition(definition) => definition.description(),
            Type::MultiType { description, .. } => description.as_deref(),
        }
    }
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase", tag = "type")]
enum TypeDefinition {
    Number {
        description: Option<String>,
    },
    Integer {
        description: Option<String>,
    },
    Boolean {
        description: Option<String>,
    },
    #[serde(rename_all = "camelCase")]
    String {
        description: Option<String>,
        #[serde(default, rename = "enum", alias = "_enum")]
        enum_values: Vec<String>,
        #[serde(default, rename = "enumDescriptions")]
        enum_descriptions: Vec<String>,
    },
    Array {
        description: Option<String>,
        items: Box<Type>,
    },
    Object {
        description: Option<String>,
        #[serde(default)]
        properties: IndexMap<String, Type>,
        #[serde(default)]
        required: HashSet<String>,
    },
    #[serde(other)]
    Other,
}

impl TypeDefinition {
    fn description(&self) -> Option<&str> {
        match self {
            TypeDefinition::String { description, .. } => description.as_deref(),
            TypeDefinition::Object { description, .. } => description.as_deref(),
            TypeDefinition::Number { description, .. } => description.as_deref(),
            TypeDefinition::Integer { description, .. } => description.as_deref(),
            TypeDefinition::Array { description, .. } => description.as_deref(),
            TypeDefinition::Boolean { description, .. } => description.as_deref(),
            TypeDefinition::Other => Some("TODO unknown type"),
        }
    }
    fn type_str(&self) -> String {
        match self {
            TypeDefinition::Number { .. } => "TODO Number type".to_string(),
            TypeDefinition::Boolean { .. } => "bool".to_string(),
            TypeDefinition::Integer { .. } => "i32".to_string(),
            TypeDefinition::String { enum_values, .. } => {
                if enum_values.is_empty() {
                    "String".to_string()
                } else {
                    single_type("TODO", self)
                }
            }
            TypeDefinition::Array { items, .. } => format!("Vec<{}>", items.type_str()),
            TypeDefinition::Object { .. } => "TODO Object type".to_string(),
            TypeDefinition::Other => "TODO unknown type".to_string(),
        }
    }
}

fn transform_description(description: &str) -> String {
    "/// ".to_string() + &description.replace('\n', "\n///\n/// ")
}

fn get_tag<'l>(properties: &'l IndexMap<String, Type>, tag_name: &str) -> &'l str {
    match properties.get(tag_name) {
        Some(Type::Definition(TypeDefinition::String { enum_values, .. })) => {
            enum_values[0].as_str()
        }
        _ => todo!(),
    }
}

// #[serde(rename_all = "camelCase")]
// Exited {
//     /// The exit code returned from the debuggee.
//     exit_code: i32,
// },
