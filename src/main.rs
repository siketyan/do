mod dofile;
mod operations;

use std::env::args;
use std::fs::File;

use crate::dofile::Dofile;

#[derive(Debug, thiserror::Error)]
pub(crate) enum Error {
    #[error("I/O Error: {0}")]
    IO(#[from] std::io::Error),

    #[error("Deserialize Error: {0}")]
    Deserialize(#[from] serde_yaml::Error),

    #[error("Dofile Error: {0}")]
    Dofile(#[from] dofile::Error),
}

type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    let file = File::open("./.do.yaml").map_err(Error::IO)?;
    let dofile: Dofile = serde_yaml::from_reader(file).map_err(Error::Deserialize)?;
    let target = args().nth(1).unwrap_or_default();

    match dofile.find_task(&target) {
        Some(t) => t.run().map_err(Error::Dofile),
        _ => Ok(()),
    }
}
