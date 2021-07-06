mod console;
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
    console::running("Finding Dofile");
    console::success("Found: .do.yaml");

    let file = File::open("./.do.yaml").map_err(Error::IO)?;
    let dofile: Dofile = serde_yaml::from_reader(file).map_err(Error::Deserialize)?;
    let target = args().nth(1).unwrap_or_default();

    console::running("Doing");
    dofile.run(&target).map_err(Error::Dofile)?;
    console::success("Done");

    Ok(())
}
