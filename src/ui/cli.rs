use std::fmt::Write;
use std::io::{stdout, Write as _};

use tracing::{error, info, trace, warn};

use super::{DebuggerUI, Status};
use crate::breakpoint::Addr;
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
        } else {
            info!("{feedback}");
        }

        loop {
            let line = self.get_line()?;
            let line_lower = line.to_lowercase();
            let words: Vec<&str> = line_lower.split_whitespace().collect();
            if words.is_empty() {
                return Ok(Status::DebuggerQuit);
            }
            trace!("processing '{line_lower}'");

            if starts_with_any(words[0], &["cont", "c"]) {
                return Ok(Status::Continue);
            } else if starts_with_any(words[0], &["break", "bp"]) {
                let addr_raw: usize = usize::from_str_radix(words[1], 16)?;
                let addr: Addr = Addr::from(addr_raw);
                return Ok(Status::SetBreakpoint(addr));
            } else if starts_with_any(words[0], &["regs"]) {
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
