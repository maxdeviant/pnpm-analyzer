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

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize)]
pub struct PackageId(pub String);

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

    #[serde(rename = "pnpm:package-manifest")]
    PackageManifest,

    #[serde(rename = "pnpm:context")]
    Context,

    #[serde(rename = "pnpm:stage")]
    Stage,

    #[serde(rename = "pnpm:hook")]
    Hook,

    #[serde(rename = "pnpm:deprecation")]
    Deprecation,

    #[serde(rename = "pnpm:stats")]
    Stats,

    #[serde(rename = "pnpm:package-import-method")]
    PackageImportMethod,

    #[serde(rename = "pnpm:root")]
    Root,

    #[serde(rename = "pnpm:_dependency_resolved")]
    DependencyResolved(PnpmDependencyResolved),

    #[serde(rename = "pnpm:progress")]
    Progress,

    #[serde(rename = "pnpm:fetching-progress")]
    FetchingProgress(PnpmFetchingProgress),

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

#[derive(Debug, Deserialize)]
pub struct PnpmDependencyResolved {
    pub resolution: String,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum PnpmFetchingProgress {
    #[serde(rename_all = "camelCase")]
    Started {
        attempt: i32,
        package_id: PackageId,
        size: i32,
    },

    #[serde(rename_all = "camelCase")]
    InProgress {
        package_id: PackageId,
        downloaded: i32,
    },
}
