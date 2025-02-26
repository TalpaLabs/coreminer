use std::ffi::CString;
use std::path::PathBuf;

use crate::errors::Result;
use crate::feedback::Feedback;
use crate::{Addr, Register, Word};

pub mod cli;

pub enum Status {
    Backtrace,
    StepOver,
    StepInto,
    StepOut,
    StepSingle,
    GetSymbolsByName(String),
    /// the bool is if the disassembly should be literal (include
    /// int3 from breakpoints), or not
    DisassembleAt(Addr, usize, bool),
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
    WriteVariable(String, usize),
    GetStack,
    ProcMap,
    Run(PathBuf, Vec<CString>),
}

pub trait DebuggerUI {
    fn process(&mut self, feedback: Feedback) -> Result<Status>;
}
