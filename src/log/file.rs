use std::iter::once;

use chrono::{DateTime, Utc};
use serde_jsonlines::append_json_lines;

use crate::{event::Event, log::Log, prelude::*};

#[derive(Default)]
pub enum Format {
    #[default]
    Jsonl,
}

#[derive(Default)]
pub enum Grouping {
    #[default]
    Monthly,
}

#[derive(Default)]
pub struct File {
    dir: PathBuf,
    format: Format,
    grouping: Grouping,
}

impl File {
    pub fn name(&self, timestamp: &DateTime<Utc>) -> String {
        match self.grouping {
            Grouping::Monthly => timestamp.format("%Y_%m").to_string(),
        }
    }

    pub fn extension(&self) -> String {
        match self.format {
            Format::Jsonl => String::from("jsonl"),
        }
    }

    pub fn path(&self, timestamp: &DateTime<Utc>) -> PathBuf {
        self.dir.join(PathBuf::from(format!(
            "{}.{}",
            self.name(timestamp),
            self.extension()
        )))
    }

    pub fn new(dir: &Path, format: Format, grouping: Grouping) -> Self {
        Self {
            dir: dir.to_path_buf(),
            format,
            grouping,
        }
    }
}

impl Log for File {
    fn log(&self, event: &Event) -> Result<()> {
        let path = self.path(&event.timestamp);

        append_json_lines(&path, once(event))
            .context(anyhow!("Could not append to file log {}", path))
    }
}
