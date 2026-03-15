use crate::{command::add::Add, prelude::*};
use clap::{Parser, Subcommand};

mod add;

pub trait Run {
    fn run(self) -> Result<()>;
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
}

impl Lim {
    pub fn cli() -> Result<()> {
        Self::parse().run()
    }
}

impl Run for Lim {
    fn run(self) -> Result<()> {
        match self.command {
            LimCommand::Add(cmd) => cmd.run(),
        }
    }
}
