use crate::event::Event;
use crate::prelude::*;

pub mod file;

pub use file::{File, Format, Grouping};

pub trait Log {
    fn log(&self, event: &Event) -> Result<()>;
}
