use std::process::{Command, Stdio};

use serde::{Deserialize, Serialize};

use crate::operations::{Error, Operable, Result};

#[derive(Deserialize, Serialize)]
pub(crate) struct Run {
    command: String,
    arguments: Vec<String>,
}

impl Operable for &Run {
    fn operate(&self) -> Result<()> {
        Command::new(&self.command)
            .args(&self.arguments)
            .stdin(Stdio::inherit())
            .stdout(Stdio::inherit())
            .output()
            .map(|_| ())
            .map_err(Error::IO)
    }
}
