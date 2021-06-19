# Nacharbeiten:
* oneOf
* HashMap<String, String>
* Option<bool> -> bool mit default
  * Falls default false: `default, skip_serializing_if = "eq_default"`
  * Falls defailt true: `default = "true_"`
* Enums
* Defaults `default|miss|omit|assum|specif|Option<Vec|Option<bool>`

```rust
#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[serde(untagged)]
pub enum ModuleId {
  Integer(i32),
  String(String),
}
```
