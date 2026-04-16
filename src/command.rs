use std::fs;

use crate::{
    Config,
    command::{add::Add, completions::Completions, data_dir::DataDir, paths::Paths, peek::Peek, pop::Pop, query::Query},
    config,
    prelude::*,
};
use clap::{Parser, Subcommand};

mod add;
mod completions;
mod data_dir;
mod paths;
mod peek;
mod pop;
mod query;

pub trait Run {
    fn run(self, config: &Config) -> Result<()>;
}

#[derive(Parser)]
#[command(version, about)]
pub struct Lim {
    #[command(subcommand)]
    command: LimCommand,
}

#[derive(Subcommand)]
enum LimCommand {
    Add(Add),

    #[clap(alias = "q")]
    Query(Query),

    DataDir(DataDir),
    Peek(Peek),
    Pop(Pop),
    #[clap(name = "_paths", hide = true)]
    Paths(Paths),
    Completions(Completions),
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
            LimCommand::Peek(cmd) => cmd.run(config),
            LimCommand::Pop(cmd) => cmd.run(config),
            LimCommand::Paths(cmd) => cmd.run(config),
            LimCommand::Completions(cmd) => cmd.run(config),
        }
    }
}
