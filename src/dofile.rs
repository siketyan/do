use serde::{Deserialize, Serialize};

use crate::operations::Operation;

#[derive(Debug, thiserror::Error)]
pub(crate) enum Error {
    #[error("Operation Error: {0}")]
    Operation(#[from] crate::operations::Error),
}

pub(crate) type Result<T> = std::result::Result<T, Error>;

#[derive(Deserialize, Serialize)]
pub(crate) struct Task {
    name: String,
    operations: Vec<Operation>,
}

impl Task {
    pub(crate) fn run(&self) -> Result<()> {
        match self
            .operations
            .iter()
            .find_map(|o| o.to_operable().operate().err())
        {
            Some(e) => Err(Error::Operation(e)),
            _ => Ok(()),
        }
    }

    fn is_name_of(&self, name: &str) -> bool {
        self.name == name
    }
}

#[derive(Deserialize, Serialize)]
pub(crate) struct Dofile {
    tasks: Vec<Task>,
}

impl Dofile {
    pub(crate) fn find_task(&self, name: &str) -> Option<&Task> {
        self.tasks.iter().find(|t| t.is_name_of(name))
    }
}
