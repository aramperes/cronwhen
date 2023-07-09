use std::process::Command;

use anyhow::bail;
use anyhow::Context;
use anyhow::Result;
use chrono::DateTime;
use chrono::Utc;
use regex::Regex;

fn main() -> Result<()> {
    let now = Utc::now();
    let crons = list_crons(&now)?;

    if crons.is_empty() {
        bail!("There are no crons to list");
    }

    for cron in crons {
        println!("{}", cron.line);

        let duration = cron.next - now;
        let duration = duration.num_seconds();

        let seconds = duration % 60;
        let minutes = (duration / 60) % 60;
        let hours = (duration / 60) / 60;

        let mut duration = format!("{} sec", seconds);
        if minutes > 0 {
            duration = format!("{} min {}", minutes, duration);
        }
        if hours > 0 {
            duration = format!("{} hours {}", hours, duration);
        }

        println!("Next iteration in {} ({})", duration, cron.next);
        println!();
    }

    Ok(())
}

struct CronItem {
    pub next: DateTime<Utc>,
    pub line: String,
}

impl CronItem {
    pub fn parse(line: &str, now: &DateTime<Utc>) -> anyhow::Result<Option<Self>> {
        let re = Regex::new(r"^((((\d+,)+\d+|(\d+(\/|-)\d+)|\d+|\*)(\/\d+)? ?){5})").unwrap();
        let line = line.trim();

        let schedule = re.find_iter(line).next().map(|c| c.as_str().trim());

        if let Some(schedule) = schedule {
            let next = cron_parser::parse(schedule, now).with_context(|| {
                format!(
                    "Failed to parse cron: '{}' (extracted: '{}')",
                    line, schedule
                )
            })?;
            Ok(Some(Self {
                next,
                line: line.to_owned(),
            }))
        } else {
            Ok(None)
        }
    }
}

fn list_crons(now: &DateTime<Utc>) -> Result<Vec<CronItem>> {
    let output = Command::new("crontab")
        .arg("-l")
        .output()
        .with_context(|| "Failed to list crons with command `crontab -l`")?;

    if output.status.success() {
        let output = String::from_utf8(output.stdout)
            .with_context(|| "Failed to read crontab output (decoding error)")?;
        Ok(output
            .lines()
            .filter_map(|line| {
                CronItem::parse(line, now).unwrap_or_else(|e| {
                    eprintln!("[parse error] {:?}", e);
                    None
                })
            })
            .collect())
    } else {
        Ok(Vec::default())
    }
}
