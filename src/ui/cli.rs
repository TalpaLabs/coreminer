use dialoguer::BasicHistory;
use tracing::{error, info, trace, warn};

use super::{DebuggerUI, Status};
use crate::breakpoint::Addr;
use crate::errors::Result;
use crate::feedback::Feedback;

pub struct CliUi {
    buf: String,
    buf_preparsed: Vec<String>,
    history: BasicHistory,
}

impl CliUi {
    pub fn build() -> Result<Self> {
        let ui = CliUi {
            buf_preparsed: Vec::new(),
            buf: String::new(),
            history: BasicHistory::new(),
        };
        Ok(ui)
    }

    pub fn get_input(&mut self) -> Result<()> {
        self.buf = dialoguer::Input::with_theme(&dialoguer::theme::ColorfulTheme::default())
            .history_with(&mut self.history)
            .interact_text()?;
        trace!("processing '{}'", self.buf);
        self.buf_preparsed = self.buf.split_whitespace().map(|a| a.to_string()).collect();
        Ok(())
    }
}

impl DebuggerUI for CliUi {
    fn process(&mut self, feedback: &Feedback) -> crate::errors::Result<Status> {
        if let Feedback::Error(e) = feedback {
            warn!("{e}");
        } else {
            info!("{feedback}");
        }

        loop {
            self.get_input()?;

            if starts_with_any(&self.buf_preparsed[0], &["cont", "c"]) {
                return Ok(Status::Continue);
            } else if starts_with_any(&self.buf_preparsed[0], &["break", "bp"]) {
                let addr_raw: usize = usize::from_str_radix(&self.buf_preparsed[1], 16)?;
                let addr: Addr = Addr::from(addr_raw);
                return Ok(Status::SetBreakpoint(addr));
            } else if starts_with_any(&self.buf_preparsed[0], &["delbreak", "dbp"]) {
                let addr_raw: usize = usize::from_str_radix(&self.buf_preparsed[1], 16)?;
                let addr: Addr = Addr::from(addr_raw);
                return Ok(Status::DelBreakpoint(addr));
            } else if starts_with_any(&self.buf_preparsed[0], &["regs"]) {
                return Ok(Status::DumpRegisters);
            } else {
                error!("bad input, use help if we already bothered to implement that");
            }
        }
    }
}

fn starts_with_any(cmd: &str, prefixes: &[&str]) -> bool {
    prefixes.iter().any(|a| cmd.starts_with(a))
}
