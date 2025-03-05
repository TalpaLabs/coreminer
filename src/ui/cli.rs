//! # Command-Line Interface
//!
//! Provides a basic command-line interface for interacting with the Coreminer debugger.
//!
//! This module implements the [`DebuggerUI`] trait to provide an interactive
//! command-line interface with features such as:
//!
//! - Command history and recall
//! - Parsing hex values for addresses and register values
//! - Command validation and error messages
//! - Automatic execution via stepper functionality
//!
//! The CLI interface accepts commands for controlling program execution,
//! setting breakpoints, examining memory and registers, and other debugging tasks.

use std::ffi::CString;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::str::FromStr;

use dialoguer::BasicHistory;
use tracing::{error, info, trace, warn};

use super::{DebuggerUI, Register, Status};
use crate::errors::Result;
use crate::feedback::Feedback;
use crate::Addr;

/// Command-line interface for the debugger
///
/// Implements the [`DebuggerUI`] trait to provide an interactive
/// command-line interface for the debugger.
///
/// # Examples
///
/// ```no_run
/// use coreminer::ui::cli::CliUi;
/// use coreminer::ui::DebuggerUI;
/// use coreminer::feedback::Feedback;
/// use std::path::Path;
///
/// // Create a CLI UI with no default executable
/// let mut ui = CliUi::build(None).unwrap();
///
/// // Process feedback from the debugger with user input
/// let status = ui.process(Feedback::Ok).unwrap();
/// ```
pub struct CliUi {
    buf: String,
    buf_preparsed: Vec<String>,
    history: BasicHistory,
    stepper: usize,
    default_executable: Option<PathBuf>,
}

impl CliUi {
    /// Creates a new CLI UI instance
    ///
    /// # Parameters
    ///
    /// * `default_executable` - Optional path to a default executable to run
    ///
    /// # Returns
    ///
    /// * `Ok(CliUi)` - A new CLI UI instance
    /// * `Err(DebuggerError)` - If creation failed
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    ///
    /// - The path of the executable does not exist
    /// - The path of the executable is not a file
    /// - The path of the executable is not executable
    ///
    /// # Examples
    ///
    /// ```
    /// use coreminer::ui::cli::CliUi;
    /// use std::path::Path;
    ///
    /// // Create a CLI UI with no default executable
    /// let ui = CliUi::build(None).unwrap();
    ///
    /// // Create a CLI UI with a default executable
    /// let path = Path::new("/bin/ls");
    /// let ui = CliUi::build(Some(path)).unwrap();
    /// ```
    pub fn build(default_executable: Option<&Path>) -> Result<Self> {
        if let Some(exe) = default_executable {
            if !exe.exists() {
                return Err(crate::errors::DebuggerError::ExecutableDoesNotExist);
            }
            if !exe.is_file() {
                return Err(crate::errors::DebuggerError::ExecutableIsNotAFile);
            }
            // check if it has the executable permission set
            if !std::fs::metadata(exe)?.permissions().mode() & 0o111 != 0 {
                return Err(crate::errors::DebuggerError::ExecutableIsNotExecutable);
            }
        }
        let ui = CliUi {
            buf_preparsed: Vec::new(),
            buf: String::new(),
            history: BasicHistory::new(),
            stepper: 0,
            default_executable: default_executable.map(std::borrow::ToOwned::to_owned),
        };
        Ok(ui)
    }

    /// Gets input from the user
    ///
    /// Uses the [`dialoguer`] crate to get input with history support.
    ///
    /// # Returns
    ///
    /// * `Ok(())` - If input was successfully read
    /// * `Err(DebuggerError)` - If input could not be read
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    ///
    /// - The dialoguer library fails to get input
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use coreminer::ui::cli::CliUi;
    /// # use std::path::Path;
    /// # let mut ui = CliUi::build(None).unwrap();
    /// if let Err(e) = ui.get_input() {
    ///     eprintln!("Failed to get input: {}", e);
    /// }
    /// ```
    pub fn get_input(&mut self) -> Result<()> {
        self.buf = dialoguer::Input::with_theme(&dialoguer::theme::ColorfulTheme::default())
            .history_with(&mut self.history)
            .interact_text()?;
        trace!("processing '{}'", self.buf);
        self.buf_preparsed = self
            .buf
            .split_whitespace()
            .map(std::string::ToString::to_string)
            .collect();
        trace!("preparsed: {:?}", self.buf_preparsed);
        Ok(())
    }

    /// Parses a number from the command line arguments
    ///
    /// Supports both `0x` prefixed and non-prefixed hexadecimal values.
    ///
    /// Returns `None` if parsing did not work or the given `index` is more than the length of the
    /// internal buffer [`Self::buf_preparsed`].
    ///
    /// # Parameters
    ///
    /// * `index` - The index of the argument to parse
    ///
    /// # Returns
    ///
    /// * `Some(u64)` - The parsed number
    /// * `None` - If the number could not be parsed or the index is out of bounds
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

    /// Ensures a command has the correct number of arguments
    ///
    /// # Parameters
    ///
    /// * `cmd` - The command name for error reporting
    /// * `expected` - The expected number of arguments (not including the command itself)
    ///
    /// # Returns
    ///
    /// * `true` - If the command has enough arguments
    /// * `false` - If the command does not have enough arguments
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

                if let Some(addr_raw) = self.get_number(1) {
                    let addr: Addr = Addr::from(addr_raw as usize);
                    return Ok(Status::DelBreakpoint(addr));
                } else {
                    error!("Invalid address for delbreak");
                    continue;
                }
            } else if string_matches(cmd, &["d", "dis"]) {
                if !self.ensure_args("disassemble", 2) {
                    continue;
                }

                let addr_raw = if let Some(val) = self.get_number(1) {
                    val as usize
                } else {
                    error!("Invalid address for disassemble");
                    continue;
                };

                let len = if let Some(val) = self.get_number(2) {
                    val as usize
                } else {
                    error!("Invalid length for disassemble");
                    continue;
                };

                let addr = Addr::from(addr_raw);
                let literal = self.buf_preparsed.get(3).is_some_and(|s| s == "--literal");
                return Ok(Status::DisassembleAt(addr, len, literal));
            } else if string_matches(cmd, &["break", "bp"]) {
                if !self.ensure_args("break", 1) {
                    continue;
                }

                if let Some(addr_raw) = self.get_number(1) {
                    let addr: Addr = Addr::from(addr_raw as usize);
                    return Ok(Status::SetBreakpoint(addr));
                } else {
                    error!("Invalid address for breakpoint");
                    continue;
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
                    error!("Unknown subcommand for set");
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

                if let Some(value) = self.get_number(2) {
                    return Ok(Status::WriteVariable(symbol_name, value as usize));
                } else {
                    error!("Invalid value for variable");
                    continue;
                }
            } else if string_matches(cmd, &["run"]) {
                if self.buf_preparsed.len() == 1 && self.default_executable.is_some() {
                    return Ok(Status::Run(
                        self.default_executable.clone().unwrap(),
                        Vec::new(),
                    ));
                }
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

                if let Some(addr_raw) = self.get_number(1) {
                    let addr: Addr = Addr::from(addr_raw as usize);
                    return Ok(Status::ReadMem(addr));
                } else {
                    error!("Invalid address for rmem");
                    continue;
                }
            } else if string_matches(cmd, &["wmem"]) {
                if !self.ensure_args("wmem", 2) {
                    continue;
                }

                let addr_raw = if let Some(val) = self.get_number(1) {
                    val as usize
                } else {
                    error!("Invalid address for wmem");
                    continue;
                };

                let value = if let Some(val) = self.get_number(2) {
                    val as i64
                } else {
                    error!("Invalid value for wmem");
                    continue;
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
                        Ok(register) => {
                            if let Some(value) = self.get_number(3) {
                                return Ok(Status::SetRegister(register, value));
                            } else {
                                error!("Invalid value for register");
                                continue;
                            }
                        }
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

/// Checks if a command matches any of the provided prefixes
///
/// # Parameters
///
/// * `cmd` - The command to check
/// * `prefixes` - The prefixes to match against
///
/// # Returns
///
/// * `true` - If the command matches any prefix
/// * `false` - If the command does not match any prefix
fn string_matches(cmd: &str, prefixes: &[&str]) -> bool {
    prefixes.iter().any(|a| cmd == *a)
}

/// Shows help information for the debugger commands
///
/// Prints a list of all available commands and their usage to stdout.
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
            default_executable: None,
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
