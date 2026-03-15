use crate::{
    Event, Log,
    command::Run,
    event::EventPath,
    log::{self},
    prelude::*,
};
use clap::Args;
use std::{collections::HashMap, env};

#[derive(Args)]
pub struct Add {
    path: EventPath,

    #[arg(short, long)]
    message: String,

    #[arg(value_parser = parse_attributes)]
    attributes: Vec<(EventPath, String)>,
}

fn parse_attributes(s: &str) -> Result<(EventPath, String)> {
    let mut elements = s.split("=");
    let key = elements.next();
    let value = elements.next();
    let after = elements.next();

    match (key, value, after) {
        (Some(k), Some(v), None) => Ok((EventPath(k.to_string()), v.to_string())),
        _ => Err(anyhow!("Received malformed attribute pair: {}", s)),
    }
}

impl Run for Add {
    fn run(self) -> Result<()> {
        let attributes: HashMap<EventPath, String> = self.attributes.into_iter().collect();
        let event = Event::now(self.path, &self.message, attributes);

        // TODO: Make log directory configurable
        let dir = env::current_dir().context("Could not determine current directory")?;
        let dir = PathBuf::from_path_buf(dir);

        match dir {
            Ok(dir) => {
                let log = log::File::new(&dir, log::Format::default(), log::Grouping::default());
                log.log(&event)
            }
            Err(dir) => Err(anyhow!(
                "Path is not valid UTF-8: {}",
                dir.to_string_lossy()
            )),
        }
    }
}
