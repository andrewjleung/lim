use crate::{Config, Event, command::Run, prelude::*};
use clap::Args;

/// Remove and print the most recent logged event
#[derive(Args)]
pub struct Pop;

impl Run for Pop {
    fn run(self, config: &Config) -> Result<()> {
        let log_dir = &config.log_dir.0;

        match pop_most_recent_event(log_dir)? {
            None => println!("no logs to pop."),
            Some(event) => {
                let json = serde_json::to_string(&event)?;
                println!("{}", json);
            }
        }

        Ok(())
    }
}

fn pop_most_recent_event(log_dir: &Path) -> Result<Option<Event>> {
    if !log_dir.exists() {
        return Ok(None);
    }

    let files = crate::log::sorted_jsonl_files(log_dir, true)?;

    for entry in files {
        let file_path = entry.path();

        let contents = std::fs::read_to_string(file_path)
            .with_context(|| format!("Failed to read {}", file_path))?;

        let mut lines: Vec<&str> = contents.lines().collect();

        while let Some(line) = lines.pop() {
            let line = line.trim();

            if line.is_empty() {
                continue;
            }

            let event: Event = serde_json::from_str(line)
                .with_context(|| format!("Failed to parse event from {}", file_path))?;

            if lines.is_empty() {
                std::fs::remove_file(file_path)
                    .with_context(|| format!("Failed to delete {}", file_path))?;
            } else {
                let new_contents = lines.join("\n") + "\n";
                std::fs::write(file_path, new_contents)
                    .with_context(|| format!("Failed to write {}", file_path))?;
            }

            return Ok(Some(event));
        }
    }

    Ok(None)
}
