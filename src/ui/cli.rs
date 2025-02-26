use std::ffi::CString;
use std::path::PathBuf;
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
    stepper: usize,
}

impl CliUi {
    pub fn build() -> Result<Self> {
        let ui = CliUi {
            buf_preparsed: Vec::new(),
            buf: String::new(),
            history: BasicHistory::new(),
            stepper: 0,
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

    fn get_number(&self, index: usize) -> Option<u64> {
        if index >= self.buf_preparsed.len() {
            return None;
        }

        let mut raw = self.buf_preparsed[index].clone();
        if raw.starts_with("0x") {
            raw = raw.strip_prefix("0x").unwrap().to_string();
        }
        trace!("raw number: {raw}");

        match u64::from_str_radix(&raw, 16) {
            Ok(val) => Some(val),
            Err(e) => {
                warn!("Failed to parse number '{}': {}", raw, e);
                None
            }
        }
    }

    fn ensure_args(&self, cmd: &str, expected: usize) -> bool {
        if self.buf_preparsed.len() < expected + 1 {
            error!("{} requires {} argument(s)", cmd, expected);
            return false;
        }
        true
    }
}

impl DebuggerUI for CliUi {
    fn process(&mut self, feedback: Feedback) -> crate::errors::Result<Status> {
        if let Feedback::Error(e) = feedback {
            warn!("{e}");
        } else if let Feedback::Text(t) = feedback {
            info!("\n{t}");
        } else if let Feedback::Disassembly(d) = feedback {
            info!("\n{d}");
        } else {
            info!("{feedback}");
        }

        if self.stepper > 0 {
            self.stepper -= 1;
            return Ok(Status::StepSingle);
        }

        loop {
            if let Err(e) = self.get_input() {
                error!("Error getting input: {}", e);
                continue;
            }

            if self.buf_preparsed.is_empty() {
                continue;
            }

            let cmd = &self.buf_preparsed[0].to_lowercase();

            if string_matches(cmd, &["cont", "c"]) {
                return Ok(Status::Continue);
            } else if string_matches(cmd, &["delbreak", "dbp"]) {
                if !self.ensure_args("delbreak", 1) {
                    continue;
                }

                match self.get_number(1) {
                    Some(addr_raw) => {
                        let addr: Addr = Addr::from(addr_raw as usize);
                        return Ok(Status::DelBreakpoint(addr));
                    }
                    None => {
                        error!("Invalid address for delbreak");
                        continue;
                    }
                }
            } else if string_matches(cmd, &["d", "dis"]) {
                if !self.ensure_args("disassemble", 2) {
                    continue;
                }

                let addr_raw = match self.get_number(1) {
                    Some(val) => val as usize,
                    None => {
                        error!("Invalid address for disassemble");
                        continue;
                    }
                };

                let len = match self.get_number(2) {
                    Some(val) => val as usize,
                    None => {
                        error!("Invalid length for disassemble");
                        continue;
                    }
                };

                let addr = Addr::from(addr_raw);
                let literal = self.buf_preparsed.get(3).is_some_and(|s| s == "--literal");
                return Ok(Status::DisassembleAt(addr, len, literal));
            } else if string_matches(cmd, &["break", "bp"]) {
                if !self.ensure_args("break", 1) {
                    continue;
                }

                match self.get_number(1) {
                    Some(addr_raw) => {
                        let addr: Addr = Addr::from(addr_raw as usize);
                        return Ok(Status::SetBreakpoint(addr));
                    }
                    None => {
                        error!("Invalid address for breakpoint");
                        continue;
                    }
                }
            } else if string_matches(cmd, &["set"]) {
                if !self.ensure_args("set", 2) {
                    continue;
                }

                if self.buf_preparsed[1] == "stepper" {
                    match self.get_number(2) {
                        Some(steps) => {
                            self.stepper = steps as usize;
                        }
                        None => {
                            error!("Invalid number for stepper");
                        }
                    }
                } else {
                    error!("Unknown subcommand for set")
                }
                continue;
            } else if string_matches(cmd, &["sym", "gsym"]) {
                if !self.ensure_args("symbol", 1) {
                    continue;
                }

                let symbol_name: String = self.buf_preparsed[1].to_string();
                return Ok(Status::GetSymbolsByName(symbol_name));
            } else if string_matches(cmd, &["var"]) {
                if !self.ensure_args("var", 1) {
                    continue;
                }

                let symbol_name: String = self.buf_preparsed[1].to_string();
                return Ok(Status::ReadVariable(symbol_name));
            } else if string_matches(cmd, &["vars"]) {
                if !self.ensure_args("vars", 2) {
                    continue;
                }

                let symbol_name: String = self.buf_preparsed[1].to_string();

                match self.get_number(2) {
                    Some(value) => {
                        return Ok(Status::WriteVariable(symbol_name, value as usize));
                    }
                    None => {
                        error!("Invalid value for variable");
                        continue;
                    }
                }
            } else if string_matches(cmd, &["run"]) {
                if !self.ensure_args("run", 1) {
                    continue;
                }

                match PathBuf::from_str(self.buf_preparsed[1].as_str()) {
                    Ok(path) => {
                        let mut args: Vec<CString> = Vec::new();

                        // Try to create CStrings for the arguments
                        args.push(match CString::new(self.buf_preparsed[0].clone()) {
                            Ok(cs) => cs,
                            Err(e) => {
                                error!("Error creating CString: {}", e);
                                continue;
                            }
                        });

                        for arg in self.buf_preparsed.iter().skip(2) {
                            match CString::new(arg.clone()) {
                                Ok(cs) => args.push(cs),
                                Err(e) => {
                                    error!("Error creating CString for argument '{}': {}", arg, e);
                                    break;
                                }
                            }
                        }

                        return Ok(Status::Run(path, args));
                    }
                    Err(e) => {
                        error!("Invalid path: {}", e);
                    }
                }
                continue;
            } else if string_matches(cmd, &["bt"]) {
                return Ok(Status::Backtrace);
            } else if string_matches(cmd, &["so"]) {
                return Ok(Status::StepOut);
            } else if string_matches(cmd, &["su", "sov"]) {
                return Ok(Status::StepOver);
            } else if string_matches(cmd, &["si"]) {
                return Ok(Status::StepInto);
            } else if string_matches(cmd, &["s", "step"]) {
                return Ok(Status::StepSingle);
            } else if string_matches(cmd, &["info"]) {
                return Ok(Status::Infos);
            } else if string_matches(cmd, &["stack"]) {
                return Ok(Status::GetStack);
            } else if string_matches(cmd, &["pm"]) {
                return Ok(Status::ProcMap);
            } else if string_matches(cmd, &["rmem"]) {
                if !self.ensure_args("rmem", 1) {
                    continue;
                }

                match self.get_number(1) {
                    Some(addr_raw) => {
                        let addr: Addr = Addr::from(addr_raw as usize);
                        return Ok(Status::ReadMem(addr));
                    }
                    None => {
                        error!("Invalid address for rmem");
                        continue;
                    }
                }
            } else if string_matches(cmd, &["wmem"]) {
                if !self.ensure_args("wmem", 2) {
                    continue;
                }

                let addr_raw = match self.get_number(1) {
                    Some(val) => val as usize,
                    None => {
                        error!("Invalid address for wmem");
                        continue;
                    }
                };

                let value = match self.get_number(2) {
                    Some(val) => val as i64,
                    None => {
                        error!("Invalid value for wmem");
                        continue;
                    }
                };

                let addr: Addr = Addr::from(addr_raw);
                return Ok(Status::WriteMem(addr, value));
            } else if string_matches(cmd, &["regs"]) {
                if !self.ensure_args("regs", 1) {
                    continue;
                }

                if self.buf_preparsed[1] == "get" {
                    return Ok(Status::DumpRegisters);
                } else if self.buf_preparsed[1] == "set" {
                    if !self.ensure_args("regs set", 3) {
                        continue;
                    }

                    match Register::from_str(&self.buf_preparsed[2]) {
                        Ok(register) => match self.get_number(3) {
                            Some(value) => {
                                return Ok(Status::SetRegister(register, value));
                            }
                            None => {
                                error!("Invalid value for register");
                                continue;
                            }
                        },
                        Err(e) => {
                            error!("Invalid register: {}", e);
                            continue;
                        }
                    }
                } else {
                    error!("Only 'set' and 'get' are valid subcommands for 'regs'");
                }
                continue;
            } else if string_matches(cmd, &["help", "h", "?"]) {
                show_help();
                continue;
            } else if string_matches(cmd, &["q", "quit", "exit"]) {
                return Ok(Status::DebuggerQuit);
            } else {
                error!("Unknown command: {}", cmd);
                info!("Type 'help' for available commands");
            }
        }
    }
}

fn string_matches(cmd: &str, prefixes: &[&str]) -> bool {
    prefixes.iter().any(|a| cmd == *a)
}

fn show_help() {
    println!("\nCoreminer Debugger Help:\n");
    println!("  run PATH [ARGS]                    - Run program at PATH with optional arguments");
    println!("  c, cont                            - Continue execution");
    println!("  s, step                            - Step one instruction");
    println!("  si                                 - Step into function call");
    println!("  su, sov                            - Step over function call");
    println!("  so                                 - Step out of current function");
    println!("  bp, break ADDR                     - Set breakpoint at address (hex)");
    println!("  dbp, delbreak ADDR                 - Delete breakpoint at address (hex)");
    println!("  d, dis ADDR LEN [--literal]        - Disassemble LEN bytes at ADDR");
    println!("  bt                                 - Show backtrace");
    println!("  stack                              - Show stack");
    println!("  info                               - Show debugger info");
    println!("  pm                                 - Show process memory map");
    println!("  regs get                           - Show register values");
    println!("  regs set REG VAL                   - Set register REG to value VAL (hex)");
    println!("  rmem ADDR                          - Read memory at address (hex)");
    println!("  wmem ADDR VAL                      - Write value to memory at address (hex)");
    println!("  sym, gsym NAME                     - Look up symbol by name");
    println!("  var NAME                           - Read variable value");
    println!("  vars NAME VAL                      - Write value to variable");
    println!("  set stepper N                      - Set stepper to auto-step N times");
    println!("  q, quit, exit                      - Exit the debugger");
    println!("  help, h, ?                         - Show this help");
    println!("\nAddresses and values should be in hexadecimal (with or without 0x prefix)");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_string_matches() {
        assert!(string_matches("help", &["help", "h", "?"]));
        assert!(string_matches("h", &["help", "h", "?"]));
        assert!(!string_matches("hello", &["help", "h", "?"]));
    }

    #[test]
    fn test_get_number() {
        let mut ui = CliUi {
            buf: String::new(),
            buf_preparsed: vec![
                "cmd".to_string(),
                "19".to_string(),
                "0x19".to_string(),
                "00019".to_string(),
            ],
            history: BasicHistory::new(),
            stepper: 0,
        };

        assert_eq!(ui.get_number(1), Some(0x19));
        assert_eq!(ui.get_number(2), Some(0x19));
        assert_eq!(ui.get_number(3), Some(0x19));
        assert_eq!(ui.get_number(4), None); // Out of bounds

        // Test with invalid input
        ui.buf_preparsed = vec!["cmd".to_string(), "ZZ".to_string()];
        assert_eq!(ui.get_number(1), None);
    }
}
