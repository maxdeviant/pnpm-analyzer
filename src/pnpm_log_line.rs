use chrono::serde::ts_milliseconds;
use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LogLevel {
    Debug,
    Info,
    Warn,
    Error,
}

#[derive(Debug, Deserialize)]
pub struct PnpmLogLine {
    #[serde(with = "ts_milliseconds")]
    pub time: DateTime<Utc>,

    pub hostname: String,

    pub pid: i32,

    pub level: LogLevel,

    #[serde(flatten)]
    pub event: PnpmLogEvent,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "name")]
pub enum PnpmLogEvent {
    #[serde(rename = "pnpm")]
    Message {
        message: String,
        prefix: Option<String>,
    },

    #[serde(rename = "pnpm:global")]
    Global { message: String },

    #[serde(rename = "pnpm:scope")]
    Scope { selected: i32 },

    #[serde(rename = "pnpm:_dependency_resolved")]
    DependencyResolved,

    #[serde(rename = "pnpm:progress")]
    Progress,

    #[serde(rename = "pnpm:fetching-progress")]
    FetchingProgress,

    #[serde(rename = "pnpm:install-check")]
    InstallCheck,

    #[serde(rename = "pnpm:skipped-optional-dependency")]
    SkippedOptionalDependency,

    #[serde(rename = "pnpm:link")]
    Link,

    #[serde(rename = "pnpm:lifecycle")]
    Lifecycle,

    #[serde(rename = "pnpm:lockfile")]
    Lockfile,

    #[serde(rename = "pnpm:summary")]
    Summary,

    #[serde(other)]
    Unknown,
}
