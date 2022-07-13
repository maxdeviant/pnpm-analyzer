mod pnpm_log_line;

use std::{error::Error, fs::File, io::Read};

use clap::Parser;
use pnpm_log_line::PnpmLogLine;

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

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let mut file = File::open(args.input)?;
    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?;

    for line in buffer.lines().filter(|line| looks_like_log_line(line)) {
        let line: PnpmLogLine = serde_json::from_str(line)?;

        println!("{:?}", line);
    }

    Ok(())
}
