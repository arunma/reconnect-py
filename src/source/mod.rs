pub mod csv_source;

use crate::{Record};
use anyhow::Result;

pub trait Source {
    fn read (&self) -> Result<Vec<Record>>;
}
