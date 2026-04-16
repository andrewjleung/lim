use crate::{Config, Event, command::Run, event::EventPath, prelude::*};
use clap::Args;

/// Re-path log events using prefix matching across all JSONL log files
#[derive(Args)]
pub struct Repath {
    from: EventPath,
    to: EventPath,
    #[arg(long)]
    dry_run: bool,
}

impl Run for Repath {
    fn run(self, config: &Config) -> Result<()> {
        let log_dir = &config.log_dir.0;

        if !log_dir.exists() {
            println!("No events matched path {}", self.from.0);
            return Ok(());
        }

        let mut files: Vec<_> = log_dir
            .read_dir_utf8()?
            .filter_map(|entry| entry.ok())
            .filter(|entry| {
                entry
                    .path()
                    .extension()
                    .map(|ext| ext == "jsonl")
                    .unwrap_or(false)
            })
            .collect();

        files.sort_by(|a, b| b.file_name().cmp(&a.file_name()));

        let mut total_changed = 0;

        for entry in files {
            let file_path = entry.path();

            let contents = std::fs::read_to_string(file_path)
                .with_context(|| format!("Failed to read {}", file_path))?;

            let mut file_changed = false;
            let mut new_lines: Vec<String> = Vec::new();

            for line in contents.lines() {
                if line.trim().is_empty() {
                    continue;
                }

                let mut event: Event = serde_json::from_str(line)
                    .with_context(|| format!("Failed to parse event from {}", file_path))?;

                if let Some(new_path) = apply_repath(&event.path, &self.from, &self.to) {
                    println!("{} -> {}", event.path.0, new_path.0);
                    event.path = new_path;
                    file_changed = true;
                    total_changed += 1;
                    new_lines.push(serde_json::to_string(&event)?);
                } else {
                    new_lines.push(line.to_string());
                }
            }

            if file_changed && !self.dry_run {
                let new_contents = new_lines.join("\n") + "\n";
                std::fs::write(file_path, new_contents)
                    .with_context(|| format!("Failed to write {}", file_path))?;
            }
        }

        if total_changed == 0 {
            println!("No events matched path {}", self.from.0);
        } else if self.dry_run {
            println!("Would repath {} events", total_changed);
        } else {
            println!("Repathed {} events", total_changed);
        }

        Ok(())
    }
}

fn apply_repath(path: &EventPath, from: &EventPath, to: &EventPath) -> Option<EventPath> {
    if path.0 == from.0 {
        Some(to.clone())
    } else if path.0.starts_with(&format!("{}.", from.0)) {
        Some(EventPath(format!("{}{}", to.0, &path.0[from.0.len()..])))
    } else {
        None
    }
}
