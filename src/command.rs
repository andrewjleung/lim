use std::fs;

use crate::{
    Config,
    command::{add::Add, query::Query, data_dir::DataDir},
    config,
    prelude::*,
};
use clap::{Parser, Subcommand};

mod add;
mod query;
mod data_dir;

pub trait Run {
    fn run(self, config: &Config) -> Result<()>;
}

#[derive(Parser)]
#[command(version, about)]
pub struct Lim {
    #[command(subcommand)]
    command: LimCommand,
}

// TODO: Add commands for fetching config path, data directory path, etc.
#[derive(Subcommand)]
enum LimCommand {
    Add(Add),

    #[clap(alias = "q")]
    Query(Query),

    DataDir(DataDir),
}

impl Lim {
    pub fn cli() -> Result<()> {
        let config = config()?;
        Self::parse().run(&config)
    }
}

impl Run for Lim {
    fn run(self, config: &Config) -> Result<()> {
        // TODO: Move this somewhere else...
        fs::create_dir_all(&config.log_dir.0).context(anyhow!(
            "Could not create log directory at {}",
            config.log_dir.0
        ))?;

        match self.command {
            LimCommand::Add(cmd) => cmd.run(config),
            LimCommand::Query(cmd) => cmd.run(config),
            LimCommand::DataDir(cmd) => cmd.run(config),
        }
    }
}
