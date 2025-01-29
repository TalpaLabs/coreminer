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

// Für alle values die wir brauchen
//  1. abfragen von basis infos
//  2. coreminer macht was mit der abfrage
//  3. process wird wieder mit dem passenden feedback gecalled
//  4. infos updaten
// 5. ERST JETZT UI

impl DebuggerUI for CliUi {
    fn process(&mut self, feedback: &Feedback) -> crate::errors::Result<Status> {
        if let Feedback::Error(e) = feedback {
            warn!("{e}");
        } else if let Feedback::Text(t) = feedback {
            info!("\n{t}");
        } else {
            info!("{feedback}");
        }

        loop {
            self.get_input()?;

            if starts_with_any(&self.buf_preparsed[0], &["cont", "c"]) {
                return Ok(Status::Continue);
            } else if starts_with_any(&self.buf_preparsed[0], &["d", "dis"]) {
                let addr_raw: usize = get_number(&self.buf_preparsed[1])? as usize;
                let addr = Addr::from(addr_raw);
                return Ok(Status::DisassembleAt(addr));
            } else if starts_with_any(&self.buf_preparsed[0], &["break", "bp"]) {
                let addr_raw: usize = get_number(&self.buf_preparsed[1])? as usize;
                let addr: Addr = Addr::from(addr_raw);
                return Ok(Status::SetBreakpoint(addr));
            } else if starts_with_any(&self.buf_preparsed[0], &["delbreak", "dbp"]) {
                let addr_raw: usize = get_number(&self.buf_preparsed[1])? as usize;
                let addr: Addr = Addr::from(addr_raw);
                return Ok(Status::DelBreakpoint(addr));
            } else if starts_with_any(&self.buf_preparsed[0], &["rmem"]) {
                if self.buf_preparsed.len() < 2 {
                    error!("rmem ADDR");
                    continue;
                }
                let addr_raw: usize = get_number(&self.buf_preparsed[1])? as usize;
                let addr: Addr = Addr::from(addr_raw);
                return Ok(Status::ReadMem(addr));
            } else if starts_with_any(&self.buf_preparsed[0], &["wmem"]) {
                if self.buf_preparsed.len() < 3 {
                    error!("wmem ADDR VAL");
                    continue;
                }
                let addr_raw: usize = get_number(&self.buf_preparsed[1])? as usize;
                let addr: Addr = Addr::from(addr_raw);
                let value: i64 = get_number(&self.buf_preparsed[1])? as i64;
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
                    let value: u64 = get_number(&self.buf_preparsed[1])?;
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

fn get_number(mut raw: &str) -> Result<u64> {
    if raw.starts_with("0x") {
        raw = raw.strip_prefix("0x").unwrap();
    }

    Ok(u64::from_str_radix(raw, 16)?)
}

#[cfg(test)]
mod test {
    use crate::ui::cli::get_number;

    #[test]
    fn test_get_number() {
        assert_eq!(0x19u64, get_number("19").unwrap());
        assert_eq!(0x19u64, get_number("0x19").unwrap());
        assert_eq!(0x19u64, get_number("0x00019").unwrap());
        assert_eq!(0x19u64, get_number("00019").unwrap());
        assert_eq!(0x19usize, get_number("19").unwrap() as usize);
    }
}
