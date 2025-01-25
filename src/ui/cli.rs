use std::io::{stdout, Write};

use tracing::{trace, warn};

use super::{DebuggerUI, Status};
use crate::errors::Result;
use crate::feedback::{self, Feedback};

pub struct CliUi;

impl CliUi {
    pub fn get_line(&self) -> Result<String> {
        let mut buf = String::new();
        let mut stdout = std::io::stdout();
        let stdin = std::io::stdin();
        print!("cm> ");
        stdout.flush()?;
        stdin.read_line(&mut buf)?;

        buf = buf.trim().to_string();

        Ok(buf)
    }
}

impl DebuggerUI for CliUi {
    fn process(&self, feedback: &Feedback) -> crate::errors::Result<Status> {
        if let Feedback::Error(e) = feedback {
            warn!("{e}");
        }

        let line = self.get_line()?;
        let line_lower = line.to_lowercase();
        trace!("processing '{line_lower}'");

        if line_lower.starts_with("continue") {
            Ok(Status::Continue)
        } else {
            Ok(Status::Nothing)
        }
    }
}
