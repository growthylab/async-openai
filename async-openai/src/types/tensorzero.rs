use serde::{ser::SerializeMap, Deserialize, Deserializer, Serialize, Serializer};
use serde_json::{Map, Value};

#[derive(Debug, Clone, PartialEq)]
// Two mutually exclusive modes - the standard OpenAI text, and our special TensorZero mode
pub enum TextContent {
    /// A normal openai text content block: `{"type": "text", "text": "Some content"}`. The `type` key comes from the parent `OpenAICompatibleContentBlock`
    Text { text: String },
    /// A special TensorZero mode: `{"type": "text", "tensorzero::arguments": {"custom_key": "custom_val"}}`.
    TensorZeroArguments {
        tensorzero_arguments: Map<String, Value>,
    },
}

impl Serialize for TextContent {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(Some(1))?;
        match self {
            TextContent::Text { text } => {
                map.serialize_key("text")?;
                map.serialize_value(text)?;
            }
            TextContent::TensorZeroArguments {
                tensorzero_arguments,
            } => {
                map.serialize_key("tensorzero::arguments")?;
                map.serialize_value(tensorzero_arguments)?;
            }
        }
        map.end()
    }
}

impl<'de> Deserialize<'de> for TextContent {
    fn deserialize<D: Deserializer<'de>>(de: D) -> Result<Self, D::Error> {
        let mut object: Map<String, Value> = Map::deserialize(de)?;
        let text = object.remove("text");
        let arguments = object.remove("tensorzero::arguments");
        match (text, arguments) {
            (Some(text), None) => Ok(TextContent::Text {
                text: match text {
                    Value::String(text) => text,
                    _ => return Err(serde::de::Error::custom(
                        "`text` must be a string when using `\"type\": \"text\"`",
                    )),
                },
            }),
            (None, Some(arguments)) => Ok(TextContent::TensorZeroArguments {
                tensorzero_arguments: match arguments {
                    Value::Object(arguments) => arguments,
                    _ => return Err(serde::de::Error::custom(
                        "`tensorzero::arguments` must be an object when using `\"type\": \"text\"`",
                    )),
                },
            }),
            (Some(_), Some(_)) => Err(serde::de::Error::custom(
                "Only one of `text` or `tensorzero::arguments` can be set when using `\"type\": \"text\"`",
            )),
            (None, None) => Err(serde::de::Error::custom(
                "Either `text` or `tensorzero::arguments` must be set when using `\"type\": \"text\"`",
            )),
        }
    }
}

impl From<String> for TextContent {
    fn from(value: String) -> Self {
        TextContent::Text { text: value }
    }
}

impl From<&str> for TextContent {
    fn from(value: &str) -> Self {
        TextContent::Text {
            text: value.to_string(),
        }
    }
}
