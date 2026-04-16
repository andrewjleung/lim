use crate::{Config, command::Run, prelude::*};
use clap::Args;

/// Print the path to the log data directory
#[derive(Args)]
pub struct DataDir;

impl Run for DataDir {
    fn run(self, config: &Config) -> Result<()> {
        println!("{}", config.log_dir.as_ref().0);
        Ok(())
    }
}
