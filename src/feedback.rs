//! # Feedback Module
//!
//! Provides types for communicating between the debugger and user interface.
//!
//! This module defines the [`Feedback`] enum, which is used to represent
//! the results of debugging operations in a structured way that can be
//! presented to the user. It serves as the primary communication channel
//! between the debugger core and the user interface.
//!
//! The different variants of the [`Feedback`] enum represent different types
//! of information that might be returned from debugging operations, such as
//! register values, memory contents, disassembly, and error conditions.
//!
//!
//! This module also defines the [`Status`] enum, which represents commands
//! from the [`DebuggerUI`](crate::ui::DebuggerUI) or from [Plugins](crate::plugins)
//! to the [`Debugger`](crate::debugger::Debugger).

use std::ffi::CString;
use std::fmt::Display;
use std::path::PathBuf;

use nix::libc::user_regs_struct;
use serde::{Deserialize, Serialize};
#[cfg(feature = "plugins")]
use steckrs::PluginIDOwned;

use crate::breakpoint::Breakpoint;
use crate::dbginfo::OwnedSymbol;
use crate::disassemble::Disassembly;
use crate::errors::DebuggerError;
use crate::memorymap::ProcessMemoryMap;
use crate::unwind::Backtrace;
use crate::variable::VariableValue;
use crate::{Addr, Register, Word};

/// Represents a command from the UI to the debugger
///
/// [`Status`] encapsulates commands that can be sent from the user interface
/// to the debugger, such as setting breakpoints, stepping, continuing execution,
/// and inspecting memory or registers.
///
/// # Examples
///
/// ```
/// use coreminer::feedback::Status;
/// use coreminer::addr::Addr;
/// use coreminer::Register;
/// use std::path::Path;
///
/// // Command to set a breakpoint at address 0x1000
/// let status = Status::SetBreakpoint(Addr::from(0x1000usize));
///
/// // Command to continue execution
/// let status = Status::Continue;
///
/// // Command to set a register value
/// let status = Status::SetRegister(Register::rax, 0x42);
///
/// // Command to run a executable in the debugger
/// let status = Status::Run(Path::new("/bin/ls").into(), vec![]);
/// ```
#[non_exhaustive]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
pub enum Status {
    /// Generate a backtrace of the call stack
    Backtrace,

    /// Step over the current function call
    StepOver,

    /// Step into the current function call
    StepInto,

    /// Step out of the current function
    StepOut,

    /// Step a single instruction
    StepSingle,

    /// Look up symbols by name
    GetSymbolsByName(String),

    /// Disassemble memory at the specified address
    ///
    /// The boolean parameter indicates whether to show the literal bytes
    /// (including breakpoint instructions) instead of the original code.
    DisassembleAt(Addr, usize, bool),

    /// Exit the debugger
    DebuggerQuit,

    /// Continue execution
    Continue,

    /// Set a breakpoint at the specified address
    SetBreakpoint(Addr),

    /// Get a breakpoint at the specified address
    GetBreakpoint(Addr),

    /// Remove a breakpoint at the specified address
    DelBreakpoint(Addr),

    /// Get all register values
    DumpRegisters,

    /// Set a register value
    SetRegister(Register, u64),

    /// Write a value to memory
    WriteMem(Addr, Word),

    /// Read a value from memory
    ReadMem(Addr),

    /// Show debugger information
    Infos,

    /// Read a variable's value
    ReadVariable(String),

    /// Write a value to a variable
    WriteVariable(String, usize),

    /// Show the current stack
    GetStack,

    /// Show the process memory map
    ProcMap,

    /// Run a new program
    Run(PathBuf, Vec<CString>),

    /// Set the last signal with the number of the signal
    SetLastSignal(i32),

    /// To be used by plugin hooks if the hook is done
    #[serde(skip)]
    #[cfg(feature = "plugins")]
    PluginContinue,

    #[cfg(feature = "plugins")]
    /// Enable or disable a plugin
    PluginSetEnable(PluginIDOwned, bool),

    #[cfg(feature = "plugins")]
    /// Get if a status is enabled or disabled (or does not exist)
    PluginGetStatus(PluginIDOwned),

    #[cfg(feature = "plugins")]
    /// Get a list of all loaded plugins
    PluginGetList,
}

/// Represents the result of a debugging operation
///
/// [`Feedback`] is used to communicate the results of debugging operations
/// between the debugger core and the user interface. Each variant represents
/// a different type of result that might be returned from a debugging operation.
///
/// # Examples
///
/// ```no_run
/// use coreminer::feedback::Feedback;
/// use coreminer::addr::Addr;
///
/// // Function that might return different types of feedback
/// fn example_operation(operation: &str) -> Feedback {
///     match operation {
///         "read_mem" => Feedback::Word(42),
///         "get_addr" => Feedback::Addr(Addr::from(0x1000usize)),
///         "error" => Feedback::Error(coreminer::errors::DebuggerError::NoDebugee),
///         _ => Feedback::Error(coreminer::errors::DebuggerError::Io(std::io::Error::
///         other("unknown"))),
///     }
/// }
///
/// // Processing feedback in a UI
/// fn display_feedback(feedback: Feedback) {
///     match feedback {
///         Feedback::Word(word) => println!("Word value: {:#x}", word),
///         Feedback::Addr(addr) => println!("Address: {}", addr),
///         Feedback::Error(err) => println!("Error: {}", err),
///         _ => println!("Other feedback type: {}", feedback),
///     }
/// }
/// ```
#[non_exhaustive]
#[derive(Debug, Serialize)]
pub enum Feedback {
    /// Memory word value
    Word(Word),

    /// Memory address
    Addr(Addr),

    /// Register values
    Registers(UserRegs),

    /// Error condition
    Error(DebuggerError),

    /// Success with no specific data
    Ok,

    /// Disassembled code
    Disassembly(Disassembly),

    /// Call stack backtrace
    Backtrace(Backtrace),

    /// Debug symbols
    Symbols(Vec<OwnedSymbol>),

    /// Variable value
    Variable(VariableValue),

    /// Stack contents
    Stack(crate::stack::Stack),

    /// Process memory map
    ProcessMap(ProcessMemoryMap),

    /// Debuggee process exit
    Exit(i32),

    /// Returns a requested [`Breakpoint`]
    Breakpoint(Option<Breakpoint>),

    #[cfg(feature = "plugins")]
    /// Information on if a plugin is enabled
    ///
    /// * `Some(true)` if it is enabled
    /// * `Some(false)` if it is disabled
    /// * `None` if it does not exist
    PluginStatus(Option<bool>),

    #[cfg(feature = "plugins")]
    /// List of loaded plugins
    PluginList(Vec<(PluginIDOwned, bool)>),

    /// Internal feedback for controls
    #[serde(skip)]
    #[allow(private_interfaces)] // this specific part isnt supposed to be used by anyone else
    Internal(InternalFeedback),
}

#[derive(Debug)]
#[non_exhaustive]
pub(crate) enum InternalFeedback {
    /// Internal variant signaling to stop the processing loop
    Quit,
}

impl Display for Feedback {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Feedback::Ok => write!(f, "Ok")?,
            Feedback::Error(e) => write!(f, "Error: {e}")?,
            Feedback::Registers(regs) => write!(f, "Registers: {regs:#x?}")?,
            Feedback::Word(w) => write!(f, "Word: {w:#018x?}")?,
            Feedback::Addr(w) => write!(f, "Address: {w}")?,
            Feedback::Disassembly(t) => write!(f, "{t:#?}")?,
            Feedback::Symbols(t) => write!(f, "Symbols: {t:#?}")?,
            Feedback::Backtrace(t) => write!(f, "Backtrace: {t:#?}")?,
            Feedback::Variable(t) => write!(f, "Variable: {t:#?}")?,
            Feedback::Stack(t) => write!(f, "Stack:\n{t}")?,
            Feedback::ProcessMap(pm) => write!(f, "Process Map:\n{pm:#x?}")?,
            Feedback::Exit(code) => write!(f, "Debugee exited with code {code}")?,
            Feedback::Breakpoint(bp) => write!(f, "Breakpoint: {bp:?}")?,
            Feedback::Internal(_) => write!(f, "Internal Feedback")?,
            #[cfg(feature = "plugins")]
            Feedback::PluginStatus(ps) => write!(f, "Plugin Status: {ps:?}")?,
            #[cfg(feature = "plugins")]
            Feedback::PluginList(list) => {
                write!(f, "Plugin List:")?;
                for (pl, s) in list {
                    write!(f, "\n  {pl:<20}: {s}")?;
                }
            }
        }

        Ok(())
    }
}

impl From<Result<Feedback, DebuggerError>> for Feedback {
    fn from(value: Result<Feedback, DebuggerError>) -> Self {
        match value {
            Ok(f) => f,
            Err(e) => Feedback::Error(e),
        }
    }
}

/// A Datastructure with all the registers
///
/// This is more or less the same as [`nix::libc::user_regs_struct`], but can be serialized with
/// [`serde`].
#[derive(Debug, Clone, Serialize)]
#[allow(missing_docs)] // name of the reg and their values, the fields are self explanatory
pub struct UserRegs {
    pub r15: u64,
    pub r14: u64,
    pub r13: u64,
    pub r12: u64,
    pub rbp: u64,
    pub rbx: u64,
    pub r11: u64,
    pub r10: u64,
    pub r9: u64,
    pub r8: u64,
    pub rax: u64,
    pub rcx: u64,
    pub rdx: u64,
    pub rsi: u64,
    pub rdi: u64,
    pub orig_rax: u64,
    pub rip: u64,
    pub cs: u64,
    pub eflags: u64,
    pub rsp: u64,
    pub ss: u64,
    pub fs_base: u64,
    pub gs_base: u64,
    pub ds: u64,
    pub es: u64,
    pub fs: u64,
    pub gs: u64,
}

impl From<user_regs_struct> for UserRegs {
    fn from(regs: user_regs_struct) -> Self {
        Self {
            r15: regs.r15,
            r14: regs.r14,
            r13: regs.r13,
            r12: regs.r12,
            rbp: regs.rbp,
            rbx: regs.rbx,
            r11: regs.r11,
            r10: regs.r10,
            r9: regs.r9,
            r8: regs.r8,
            rax: regs.rax,
            rcx: regs.rcx,
            rdx: regs.rdx,
            rsi: regs.rsi,
            rdi: regs.rdi,
            orig_rax: regs.orig_rax,
            rip: regs.rip,
            cs: regs.cs,
            eflags: regs.eflags,
            rsp: regs.rsp,
            ss: regs.ss,
            fs_base: regs.fs_base,
            gs_base: regs.gs_base,
            ds: regs.ds,
            es: regs.es,
            fs: regs.fs,
            gs: regs.gs,
        }
    }
}
