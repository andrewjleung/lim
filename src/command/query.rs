use crate::{Config, command::Run, event::EventPath, prelude::*};
use clap::Args;

#[derive(Args)]
pub struct Query {
    // TODO: Query by path
    #[arg(value_hint = clap::ValueHint::Other)]
    path: EventPath,
    // TODO: Query by timestamp
    // TODO: Query by attribute
}

impl Run for Query {
    fn run(self, config: &Config) -> Result<()> {
        unimplemented!()
    }
}
