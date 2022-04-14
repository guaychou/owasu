use crate::domain::seatalk::SeatalkText;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UptimeKumaRequestBody {
    pub msg: String,
}

impl SeatalkText for UptimeKumaRequestBody {
    fn generate_seatalk_text(&self) -> String {
        format!("ALERT 🔥 🔥 🔥 \n\nKuma said... \n{}\n", self.msg)
    }
}
