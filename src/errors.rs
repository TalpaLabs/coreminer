use gimli::DwTag;
use thiserror::Error;

use crate::dbginfo::SymbolKind;

pub type Result<T> = std::result::Result<T, DebuggerError>;

#[derive(Error, Debug)]
pub enum DebuggerError {
    #[error("Os error: {0}")]
    Os(#[from] nix::Error),
    #[error("Io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Executable does not exist: {0}")]
    ExecutableDoesNotExist(String),
    #[error("Executable is not a file: {0}")]
    ExecutableIsNotAFile(String),
    #[error("Could not convert to CString: {0}")]
    CStringConv(#[from] std::ffi::NulError),
    #[error("No debuggee configured")]
    NoDebugee,
    #[error("Tried to enable breakpoint again")]
    BreakpointIsAlreadyEnabled,
    #[error("Tried to disable breakpoint again")]
    BreakpointIsAlreadyDisabled,
    #[error("Could not parse integer: {0}")]
    ParseInt(#[from] std::num::ParseIntError),
    #[error("Could not parse string: {0}")]
    ParseStr(String),
    #[error("Error while getting cli input: {0}")]
    CliUiDialogueError(#[from] dialoguer::Error),
    #[error("Error while reading information from the executable file: {0}")]
    Object(#[from] object::Error),
    #[error("Error while working with the DWARF debug information: {0}")]
    Dwarf(#[from] gimli::Error),
    #[error("Error while loading the DWARF debug information into gimli")]
    GimliLoad,
    #[error("Could not format: {0}")]
    Format(#[from] std::fmt::Error),
    #[error("DWARF Tag not implemented for this debugger: {0}")]
    DwTagNotImplemented(DwTag),
    #[error("Tried stepping out of main function, this makes no sense")]
    StepOutMain,
    #[error("Unwind Error: {0}")]
    Unwind(#[from] unwind::Error),
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
    #[error("Variable expression led to multiple variables being found: {0}")]
    AmbiguousVarExpr(String),
    #[error("Variable expression led to no variables being found: {0}")]
    VarExprReturnedNothing(String),
    #[error("No datatype found for symbol which needed one")]
    NoDatatypeFound,
    #[error("The debuggee is currently not in a known function")]
    NotInFunction,
    #[error("A required attribute did not exist: {0:?}")]
    AttributeDoesNotExist(gimli::DwAt),
    #[error("While parsing a DWARF location: no frame information was provided")]
    NoFrameInfo,
}
