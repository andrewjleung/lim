use crate::event::Event;
use crate::prelude::*;

pub mod file;

pub use file::{File, Format, Grouping};

pub trait Log {
    fn log(&self, event: &Event) -> Result<()>;
}

pub fn sorted_jsonl_files(log_dir: &Path, reverse: bool) -> Result<Vec<camino::Utf8DirEntry>> {
    let mut files: Vec<_> = log_dir
        .read_dir_utf8()?
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().map(|ext| ext == "jsonl").unwrap_or(false))
        .collect();
    files.sort_by(|a, b| {
        if reverse {
            b.file_name().cmp(&a.file_name())
        } else {
            a.file_name().cmp(b.file_name())
        }
    });
    Ok(files)
}
