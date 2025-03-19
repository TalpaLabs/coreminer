//! # Error Types
//!
//! Defines error types and a result alias used throughout the [crate].
//!
//! This module provides a comprehensive error handling system for the debugger,
//! using the [thiserror] crate to define error types with detailed messages.
//! It centralizes all potential error conditions that might occur during debugging
//! operations, from system-level errors to debug information parsing issues.

use gimli::DwTag;
use serde::{Serialize, Serializer};
use thiserror::Error;

use crate::addr::Addr;
use crate::dbginfo::SymbolKind;

/// Type alias for Results returned by coreminer functions
///
/// This alias makes error handling more convenient by defaulting to the
/// [`DebuggerError`] type, allowing functions to simply return `Result<T>`.
pub type Result<T> = std::result::Result<T, DebuggerError>;

/// Comprehensive error type for the coreminer debugger
///
/// [`DebuggerError`] encapsulates all potential errors that can occur during
/// debugging operations, including system errors, parsing errors, and
/// debugger-specific errors.
///
/// # Examples
///
/// ```
/// use coreminer::errors::{DebuggerError, Result};
///
/// fn example_function() -> Result<()> {
///     // System error example
///     let file = std::fs::File::open("nonexistent_file")?;
///
///     // Debugger-specific error example
///     if true {
///         return Err(DebuggerError::NoDebugee);
///     }
///
///     Ok(())
/// }
/// ```
#[derive(Error, Debug, Serialize)]
#[allow(missing_docs)] // its just error types
pub enum DebuggerError {
    #[error("Os error: {0}")]
    Os(
        #[serde(serialize_with = "ser_err")]
        #[from]
        nix::Error,
    ),
    #[error("Io error: {0}")]
    Io(
        #[serde(serialize_with = "ser_err")]
        #[from]
        std::io::Error,
    ),
    #[error("Given Executable does not exist")]
    ExecutableDoesNotExist,
    #[error("Given Executable is not a file")]
    ExecutableIsNotAFile,
    #[error("Given Executable is not executable (try chmod +x)")]
    ExecutableIsNotExecutable,
    #[error("Could not convert to CString: {0}")]
    CStringConv(
        #[serde(serialize_with = "ser_err")]
        #[from]
        std::ffi::NulError,
    ),
    #[error("No debuggee configured")]
    NoDebugee,
    #[error("Tried to enable breakpoint again")]
    BreakpointIsAlreadyEnabled,
    #[error("Tried to disable breakpoint again")]
    BreakpointIsAlreadyDisabled,
    #[error("Could not parse integer: {0}")]
    ParseInt(
        #[serde(serialize_with = "ser_err")]
        #[from]
        std::num::ParseIntError,
    ),
    #[error("Could not parse string: {0}")]
    ParseStr(String),
    #[error("Error while getting cli input: {0}")]
    #[cfg(feature = "cli")]
    CliUiDialogueError(
        #[serde(serialize_with = "ser_err")]
        #[from]
        dialoguer::Error,
    ),
    #[error("Error while reading information from the executable file: {0}")]
    Object(
        #[serde(serialize_with = "ser_err")]
        #[from]
        object::Error,
    ),
    #[error("Error while working with the DWARF debug information: {0}")]
    Dwarf(
        #[serde(serialize_with = "ser_err")]
        #[from]
        gimli::Error,
    ),
    #[error("Error while loading the DWARF debug information into gimli")]
    GimliLoad,
    #[error("Could not format: {0}")]
    Format(
        #[serde(serialize_with = "ser_err")]
        #[from]
        std::fmt::Error,
    ),
    #[error("DWARF Tag not implemented for this debugger: {0}")]
    DwTagNotImplemented(#[serde(serialize_with = "ser_dwtag")] DwTag),
    #[error("Tried stepping out of main function, this makes no sense")]
    StepOutMain,
    #[error("Unwind Error: {0}")]
    Unwind(
        #[serde(serialize_with = "ser_err")]
        #[from]
        unwind::Error,
    ),
    #[error("While calculating the higher address with DWARF debug symbols, the lower address was none but the higher (offset) was some")]
    HighAddrExistsButNotLowAddr,
    #[error("Register with index {0} is not supported by this debugger")]
    UnimplementedRegister(u16),
    #[error("Wrong Symbol kind for this operation: {0:?}")]
    WrongSymbolKind(SymbolKind),
    #[error("Symbol has no datatype (but needed it)")]
    VariableSymbolNoType,
    #[error("Symbol has no location (but needed it)")]
    SymbolHasNoLocation,
    #[error("Symbol has byte size (but needed it)")]
    SymbolHasNoByteSize,
    #[error("Variable expression led to multiple variables being found: {0}")]
    AmbiguousVarExpr(String),
    #[error("Variable expression led to no variables being found: {0}")]
    VarExprReturnedNothing(String),
    #[error("No datatype found for symbol which needed one")]
    NoDatatypeFound,
    #[error("The debuggee is currently not in a known function")]
    NotInFunction,
    #[error("A required attribute did not exist: {0:?}")]
    AttributeDoesNotExist(#[serde(serialize_with = "ser_dwat")] gimli::DwAt),
    #[error("While parsing a DWARF location: no frame information was provided")]
    NoFrameInfo,
    #[error("Tried to run a program while one was already running")]
    AlreadyRunning,
    #[error("Found multiple DWARF entries for an operation that was supposed to only find one")]
    MultipleDwarfEntries,
    #[error("Working with JSON failed: {0}")]
    Json(
        #[serde(serialize_with = "ser_err")]
        #[from]
        serde_json::Error,
    ),
    #[error(
        "Tried to disassemble a line that we had already disassembled for this iteration: {0}"
    )]
    AlreadyDisassembled(Addr),
    #[error("The UI used {:?}", crate::feedback::Status::PluginContinue)]
    UiUsedPluginContinue,
    #[error("A plugin had too many iterations (this is an error in the plugin)")]
    TooManyPluginIterations,
    #[error("Error while controlling a pluign: {0}")]
    PluginError(#[from] steckrs::error::PluginError),
}

#[allow(clippy::trivially_copy_pass_by_ref)] // serde passes by ref
fn ser_dwat<S>(attr: &gimli::DwAt, s: S) -> std::result::Result<S::Ok, S::Error>
where
    S: Serializer,
{
    s.serialize_str(&attr.to_string())
}

#[allow(clippy::trivially_copy_pass_by_ref)] // serde passes by ref
fn ser_dwtag<S>(tag: &gimli::DwTag, s: S) -> std::result::Result<S::Ok, S::Error>
where
    S: Serializer,
{
    s.serialize_str(&tag.to_string())
}

fn ser_err<S>(err: impl std::error::Error, s: S) -> std::result::Result<S::Ok, S::Error>
where
    S: Serializer,
{
    s.serialize_str(&err.to_string())
}
