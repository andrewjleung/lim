use crate::{
    Config, Event, Log,
    command::Run,
    event::EventPath,
    log::{self},
    prelude::*,
};
use clap::Args;
use std::collections::HashMap;

/// Log an event
#[derive(Args)]
pub struct Add {
    #[arg(value_hint = clap::ValueHint::Other)]
    path: EventPath,
    message: String,

    #[arg(value_parser = parse_attributes)]
    attributes: Vec<(EventPath, String)>,
}

fn parse_attributes(s: &str) -> Result<(EventPath, String)> {
    let mut elements = s.splitn(2, '=');
    let key = elements.next();
    let value = elements.next();

    match (key, value) {
        (Some(k), Some(v)) => Ok((EventPath(k.to_string()), v.to_string())),
        _ => Err(anyhow!("Received malformed attribute pair: {}", s)),
    }
}

impl Run for Add {
    fn run(self, config: &Config) -> Result<()> {
        let attributes: HashMap<EventPath, String> = self.attributes.into_iter().collect();
        let event = Event::now(self.path, &self.message, attributes);

        let log = log::File::new(
            &config.log_dir.as_ref().0,
            log::Format::default(),
            log::Grouping::default(),
        );
        log.log(&event)
    }
}
