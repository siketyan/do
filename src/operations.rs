mod run;

use serde::{Deserialize, Serialize};

#[derive(Debug, thiserror::Error)]
pub(crate) enum Error {
    #[error("I/O Error: {0}")]
    IO(#[from] std::io::Error),
}

type Result<T> = std::result::Result<T, Error>;

#[derive(Deserialize, Serialize)]
#[serde(tag = "type")]
pub(crate) enum Operation {
    #[serde(rename = "run")]
    Run(run::Run),
}

impl Operation {
    pub(crate) fn to_operable(&self) -> Box<dyn Operable + '_> {
        match self {
            Self::Run(r) => Box::new(r),
        }
    }
}

pub(crate) trait Operable {
    fn operate(&self) -> Result<()>;
}
