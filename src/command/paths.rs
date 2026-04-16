use crate::{Config, Event, command::Run, prelude::*};
use clap::Args;
use std::collections::HashSet;

#[derive(Args)]
pub struct Paths;

impl Run for Paths {
    fn run(self, config: &Config) -> Result<()> {
        let log_dir = &config.log_dir.0;

        if !log_dir.exists() {
            return Ok(());
        }

        let mut paths: HashSet<String> = HashSet::new();

        for entry in crate::log::sorted_jsonl_files(log_dir, false)? {
            let file_path = entry.path();
            let contents = std::fs::read_to_string(file_path)
                .with_context(|| format!("Failed to read {}", file_path))?;

            for line in contents.lines() {
                let line = line.trim();
                if line.is_empty() {
                    continue;
                }
                if let Ok(event) = serde_json::from_str::<Event>(line) {
                    paths.insert(event.path.0);
                }
            }
        }

        let mut sorted: Vec<String> = paths.into_iter().collect();
        sorted.sort();
        for path in sorted {
            println!("{}", path);
        }

        Ok(())
    }
}
