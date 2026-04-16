use crate::{Config, Event, command::Run, event::EventPath, prelude::*};
use clap::Args;
use globset::GlobBuilder;

/// Query logged events
#[derive(Args)]
pub struct Query {
    #[arg(value_hint = clap::ValueHint::Other)]
    path: EventPath,
}

impl Run for Query {
    fn run(self, config: &Config) -> Result<()> {
        let log_dir = &config.log_dir.0;

        if !log_dir.exists() {
            return Ok(());
        }

        // Build glob: convert dots to slashes so * respects segment boundaries
        let pattern = self.path.0.replace('.', "/");
        let glob = GlobBuilder::new(&pattern)
            .literal_separator(true)
            .build()
            .context("Invalid glob pattern")?
            .compile_matcher();

        let files = crate::log::sorted_jsonl_files(log_dir, false)?;

        for entry in files {
            let file_path = entry.path();
            let contents = std::fs::read_to_string(file_path)
                .with_context(|| format!("Failed to read {}", file_path))?;

            for line in contents.lines() {
                let line = line.trim();
                if line.is_empty() {
                    continue;
                }
                let event: Event = serde_json::from_str(line)
                    .with_context(|| format!("Failed to parse event from {}", file_path))?;

                let normalized_path = event.path.0.replace('.', "/");
                if glob.is_match(&normalized_path) {
                    println!("{}", serde_json::to_string(&event)?);
                }
            }
        }

        Ok(())
    }
}
