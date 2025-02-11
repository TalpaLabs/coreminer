use std::str::FromStr;

use crate::errors::{DebuggerError, Result};
use crate::feedback::Feedback;
use crate::{Addr, Word};

pub mod cli;

pub enum Status {
    Backtrace,
    StepOver,
    StepInto,
    StepOut,
    StepSingle,
    GetSymbolsByName(String),
    DisassembleAt(Addr, usize),
    DebuggerQuit,
    Continue,
    SetBreakpoint(Addr),
    DelBreakpoint(Addr),
    DumpRegisters,
    SetRegister(Register, u64),
    WriteMem(Addr, Word),
    ReadMem(Addr),
    Infos,
    ReadVariable(String),
}

pub trait DebuggerUI {
    fn process(&mut self, feedback: Feedback) -> Result<Status>;
}
