use chrono::serde::ts_milliseconds;
use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct PnpmLogLine {
    #[serde(with = "ts_milliseconds")]
    pub time: DateTime<Utc>,
}
