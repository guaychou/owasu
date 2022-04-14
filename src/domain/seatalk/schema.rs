use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Message {
    pub tag: String,
    pub text: Text,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Text {
    pub content: String,
    #[serde(rename = "mentioned_list")]
    pub mentioned_list: Option<Vec<String>>,
    #[serde(rename = "mentioned_email_list")]
    pub mentioned_email_list: Option<Vec<String>>,
    #[serde(rename = "at_all")]
    pub at_all: Option<bool>,
}

impl Message {
    pub fn new(text: String) -> Message {
        let text = Text {
            content: text,
            mentioned_list: None,
            mentioned_email_list: None,
            at_all: Some(false),
        };
        Message {
            tag: "text".to_string(),
            text,
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SeatalkResponse {
    pub code: i64,
    pub msg: String,
}

impl SeatalkResponse {
    pub fn error_check(&self) -> bool {
        self.code.is_positive()
    }
}
