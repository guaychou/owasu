use crate::domain::seatalk::SeatalkText;
use chrono::DateTime;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AlertManagerRequestBody {
    pub receiver: String,
    pub status: String,
    pub alerts: Vec<Alert>,
    pub group_labels: GroupLabels,
    pub common_labels: CommonLabels,
    pub common_annotations: CommonAnnotations,
    #[serde(rename = "externalURL")]
    pub external_url: String,
    pub version: String,
    pub group_key: String,
    pub truncated_alerts: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Alert {
    pub status: String,
    pub labels: Labels,
    pub annotations: Annotations,
    pub starts_at: String,
    pub ends_at: String,
    #[serde(rename = "generatorURL")]
    pub generator_url: String,
    pub fingerprint: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Labels {
    pub alertname: String,
    pub instance: String,
    pub severity: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Annotations {
    pub description: String,
    pub summary: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GroupLabels {
    pub alertname: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommonLabels {
    pub alertname: String,
    pub severity: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommonAnnotations {
    pub summary: String,
}

impl SeatalkText for AlertManagerRequestBody {
    fn generate_seatalk_text(&self) -> String {
        let alert: String = self.alerts.iter().map(|s|{
            if s.status.to_uppercase() == *"FIRING"{
                format!(
                    "Name: {} \nInstance: {}\nStart: {}\nStatus: {}\nSummary: {}\nDescription: {}\n\n",
                    s.labels.alertname, s.labels.instance, s.starts_at, s.status.to_uppercase(), s.annotations.summary, s.annotations.description
            )}
            else if s.status.to_uppercase() == *"RESOLVED"{
                let duration = calculate_duration(s.starts_at.as_ref(), s.ends_at.as_ref());
                    format!(
                        "Name: {} \nInstance: {}\nStart At: {}\nEnd At: {}\nStatus: {}\nSummary: {}\nDescription: {}\nDuration: {}\n\n",
                        s.labels.alertname, s.labels.instance, s.starts_at, s.ends_at, s.status.to_uppercase(), s.annotations.summary, s.annotations.description, duration
                )
            }
            else {
                tracing::error!("Did not match any alertmanager status, status: {:?}", s.status.to_uppercase());
                "null".to_string()
            }
        }).collect();
        format!("ALERT 🔥 🔥 🔥 \n\n{}", alert)
    }
}

fn calculate_duration(start: &str, end: &str) -> String {
    let duration =
        DateTime::parse_from_rfc3339(end).unwrap() - DateTime::parse_from_rfc3339(start).unwrap();
    let duration_hour = duration.num_hours();
    let duration_minute = duration.num_minutes() - (duration_hour * 60);
    let duration_second = duration.num_seconds() - (duration_hour * 3600) - (duration_minute * 60);
    let hour_string = if duration_hour > 1 { "hours" } else { "hour" };
    let minute_string = if duration_minute > 1 {
        "minutes"
    } else {
        "minute"
    };
    let second_string = if duration_second > 1 {
        "seconds"
    } else {
        "second"
    };
    format!(
        "{:?} {hour_string} {:?} {minute_string} {:?} {second_string}",
        duration_hour, duration_minute, duration_second
    )
}
