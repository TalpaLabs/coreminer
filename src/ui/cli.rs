use std::str::FromStr;

use dialoguer::BasicHistory;
use tracing::{error, info, trace, warn};

use super::{DebuggerUI, Register, Status};
use crate::errors::Result;
use crate::feedback::Feedback;
use crate::Addr;

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
        trace!("preparsed: {:?}", self.buf_preparsed);
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
            } else if starts_with_any(&self.buf_preparsed[0], &["rmem"]) {
                if self.buf_preparsed.len() < 2 {
                    error!("rmem ADDR");
                    continue;
                }
                let addr_raw: usize = usize::from_str_radix(&self.buf_preparsed[1], 16)?;
                let addr: Addr = Addr::from(addr_raw);
                return Ok(Status::ReadMem(addr));
            } else if starts_with_any(&self.buf_preparsed[0], &["wmem"]) {
                if self.buf_preparsed.len() < 3 {
                    error!("wmem ADDR VAL");
                    continue;
                }
                let addr_raw: usize = usize::from_str_radix(&self.buf_preparsed[1], 16)?;
                let addr: Addr = Addr::from(addr_raw);
                let value = i64::from_str_radix(&self.buf_preparsed[2], 16)?;
                return Ok(Status::WriteMem(addr, value));
            } else if starts_with_any(&self.buf_preparsed[0], &["regs"]) {
                if self.buf_preparsed.len() < 2 {
                    error!("need to give a subcommand");
                    continue;
                }
                if self.buf_preparsed[1] == "get" {
                    return Ok(Status::DumpRegisters);
                } else if self.buf_preparsed[1] == "set" {
                    if self.buf_preparsed.len() != 4 {
                        error!("regs set REGISTER VALUE");
                        continue;
                    }
                    let register = Register::from_str(&self.buf_preparsed[2])?;
                    let value = u64::from_str_radix(&self.buf_preparsed[3], 16)?;
                    return Ok(Status::SetRegister(register, value));
                } else {
                    error!("only set and get is possible")
                }
            } else {
                error!("bad input, use help if we already bothered to implement that");
            }
        }
    }
}

fn starts_with_any(cmd: &str, prefixes: &[&str]) -> bool {
    prefixes.iter().any(|a| cmd.starts_with(a))
}
