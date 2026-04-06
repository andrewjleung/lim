use crate::{Config, command::Run, event::EventPath, prelude::*};
use clap::Args;
use zlob::Zlob;

#[derive(Args)]
pub struct Query {
    path: Zlob,
    // TODO: Query by timestamp
    // TODO: Query by attribute
}

impl Run for Query {
    fn run(self, config: &Config) -> Result<()> {
        // Go through every log file (for now)
        // Read the log file, filter all entries by matching path with given glob
        unimplemented!()
    }
}
