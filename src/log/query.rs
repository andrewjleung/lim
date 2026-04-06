use std::fs;

use zlob::Zlob;

use crate::{Event, command::Run, prelude::*};

pub struct Glob(Zlob);

pub struct Query {
    dir: PathBuf,
    glob: Glob,
}

// TODO: Make this config-aware.
impl IntoIterator for Query {
    type Item = Event;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        let foo = fs::read_dir(&self.dir)
            .expect("Can read query directory")
            .filter_map(|entry| entry.ok())
            .filter_map(|entry| serde_jsonlines::json_lines(entry.path()).ok());
    }
}
