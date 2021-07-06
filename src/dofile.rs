use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::operations::Operation;

#[derive(Debug, thiserror::Error)]
pub(crate) enum Error {
    #[error("Operation Error: {0}")]
    Operation(#[from] crate::operations::Error),

    #[error("Operation not found.")]
    NotFound,
}

pub(crate) type Result<T> = std::result::Result<T, Error>;

#[derive(Deserialize, Serialize)]
pub(crate) struct Dofile {
    tasks: HashMap<String, Vec<Operation>>,
}

impl Dofile {
    pub(crate) fn operations(&self, name: &str) -> Option<&Vec<Operation>> {
        self.tasks.get(name)
    }

    pub(crate) fn run(&self, name: &str) -> Result<()> {
        match self
            .operations(name)
            .ok_or(Error::NotFound)?
            .iter()
            .find_map(|o| o.to_operable().operate().err())
        {
            Some(e) => Err(Error::Operation(e)),
            _ => Ok(()),
        }
    }
}
