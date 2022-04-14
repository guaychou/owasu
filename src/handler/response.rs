use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Response {
    #[serde(rename(serialize = "apiVersion"))]
    api_version: String,
    success: bool,
}

impl Response {
    pub fn default() -> Self {
        Self {
            api_version: String::from("v1"),
            success: true,
        }
    }
}
