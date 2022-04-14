pub mod schema;

use {
    crate::configuration::SeatalkConfig, crate::error::AppError, reqwest::Client, schema::Message,
    schema::SeatalkResponse, serde_json::json, tokio::time::Duration, tracing::instrument,
};

#[derive(Debug, Clone)]
pub struct Seatalk {
    client: Client,
    base_url: String,
}

pub trait SeatalkText {
    fn generate_seatalk_text(&self) -> String;
}

impl Seatalk {
    pub fn new(config: SeatalkConfig) -> Seatalk {
        static APP_USER_AGENT: &str =
            concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);
        let timeout_duration = Duration::new(5, 0);
        let client = reqwest::Client::builder()
            .user_agent(APP_USER_AGENT)
            .connect_timeout(timeout_duration)
            .build()
            .unwrap_or_default();
        let base_url = format!(
            "https://openapi.seatalk.io/webhook/group/{}",
            config.get_chat_id()
        );
        Self { client, base_url }
    }

    #[instrument(name = "send_alert" skip(payload))]
    pub async fn send_alert(&self, payload: impl SeatalkText) -> Result<SeatalkResponse, AppError> {
        let text = payload.generate_seatalk_text();
        let message: Message = Message::new(text);
        let response = self
            .client
            .post(self.base_url.to_string())
            .json(&json!(message))
            .send()
            .await?
            .json::<SeatalkResponse>()
            .await?;
        Ok(response)
    }
}
