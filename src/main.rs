mod pnpm_log_line;

use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::Read;

use chrono::{DateTime, Utc};
use clap::Parser;

use crate::pnpm_log_line::{PackageId, PnpmFetchingProgress, PnpmLogEvent, PnpmLogLine};

#[derive(Debug, Parser)]
#[clap(author, version, about, long_about = None)]
struct Args {
    input: String,
}

fn looks_like_log_line(line: &str) -> bool {
    use serde::Deserialize;

    #[derive(Debug, Deserialize)]
    struct LogLine {
        #[allow(dead_code)]
        pub time: i64,
    }

    serde_json::from_str::<LogLine>(line).is_ok()
}

#[derive(Debug)]
struct FetchStatus {
    pub started_at: DateTime<Utc>,

    pub size: i32,

    pub finished_at: Option<DateTime<Utc>>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let mut file = File::open(args.input)?;
    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?;

    let mut fetching_statuses = HashMap::<PackageId, FetchStatus>::new();

    for line in buffer.lines().filter(|line| looks_like_log_line(line)) {
        let line: PnpmLogLine = serde_json::from_str(line)?;

        match line.event {
            PnpmLogEvent::FetchingProgress(progress) => match progress {
                PnpmFetchingProgress::Started {
                    attempt,
                    package_id,
                    size,
                } => {
                    fetching_statuses.insert(
                        package_id,
                        FetchStatus {
                            started_at: line.time,
                            size: size,
                            finished_at: None,
                        },
                    );
                }
                PnpmFetchingProgress::InProgress {
                    package_id,
                    downloaded,
                } => {
                    if let Some(status) = fetching_statuses.get_mut(&package_id) {
                        if downloaded == status.size {
                            status.finished_at = Some(line.time);
                        }
                    } else {
                        println!(
                            "Fetching progress for a package that never started: {:?}",
                            package_id
                        );
                    }
                }
            },
            _ => {}
        };
    }

    for (package_id, status) in fetching_statuses {
        match status.finished_at {
            Some(finished_at) => {
                println!(
                    "{:?} finished fetching in {}s ({}ms)",
                    package_id,
                    (finished_at - status.started_at).num_seconds(),
                    (finished_at - status.started_at).num_milliseconds()
                );
            }
            None => {}
        }
    }

    Ok(())
}
