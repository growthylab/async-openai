use crate::error::OpenAIError;
use crate::types::chat::ChatCompletionRequestMessageContentPartText;
use serde::{ser::SerializeMap, Deserialize, Deserializer, Serialize, Serializer};
use serde_json::{Map, Value};

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Template {
    pub name: String,
    pub arguments: Map<String, Value>,
}

impl Serialize for ChatCompletionRequestMessageContentPartText {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(Some(1))?;
        match self {
            ChatCompletionRequestMessageContentPartText::Text { text } => {
                map.serialize_key("text")?;
                map.serialize_value(text)?;
            }
            ChatCompletionRequestMessageContentPartText::TensorZeroArguments {
                tensorzero_arguments,
            } => {
                map.serialize_key("tensorzero::arguments")?;
                map.serialize_value(tensorzero_arguments)?;
            }
        }
        map.end()
    }
}

impl<'de> Deserialize<'de> for ChatCompletionRequestMessageContentPartText {
    fn deserialize<D: Deserializer<'de>>(de: D) -> Result<Self, D::Error> {
        let mut object: Map<String, Value> = Map::deserialize(de)?;
        let text = object.remove("text");
        let arguments = object.remove("tensorzero::arguments");
        match (text, arguments) {
            (Some(text), None) => Ok(ChatCompletionRequestMessageContentPartText::Text {
                text: match text {
                    Value::String(text) => text,
                    _ => return Err(serde::de::Error::custom(
                        "`text` must be a string when using `\"type\": \"text\"`",
                    )),
                },
            }),
            (None, Some(arguments)) => Ok(ChatCompletionRequestMessageContentPartText::TensorZeroArguments {
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

impl Default for ChatCompletionRequestMessageContentPartText {
    fn default() -> Self {
        Self::Text {
            text: String::default(),
        }
    }
}

/// Builder for ChatCompletionRequestMessageContentPartText
#[derive(Debug, Clone, Default)]
pub struct ChatCompletionRequestMessageContentPartTextArgs {
    text: Option<String>,
    tensorzero_arguments: Option<Map<String, Value>>,
}

impl ChatCompletionRequestMessageContentPartTextArgs {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn text<T: Into<String>>(mut self, text: T) -> Self {
        self.text = Some(text.into());
        self.tensorzero_arguments = None;
        self
    }

    pub fn tensorzero_arguments(mut self, args: Map<String, Value>) -> Self {
        self.tensorzero_arguments = Some(args);
        self.text = None;
        self
    }

    pub fn build(self) -> Result<ChatCompletionRequestMessageContentPartText, OpenAIError> {
        if let Some(text) = self.text {
            Ok(ChatCompletionRequestMessageContentPartText::Text { text })
        } else if let Some(tensorzero_arguments) = self.tensorzero_arguments {
            Ok(
                ChatCompletionRequestMessageContentPartText::TensorZeroArguments {
                    tensorzero_arguments,
                },
            )
        } else {
            Ok(ChatCompletionRequestMessageContentPartText::default())
        }
    }
}
