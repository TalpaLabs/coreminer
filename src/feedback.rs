use std::fmt::Display;

use nix::libc::user_regs_struct;

use crate::errors::DebuggerError;
use crate::Word;

#[derive(Debug)]
pub enum Feedback {
    Word(Word),
    Registers(user_regs_struct),
    Error(DebuggerError),
    Ok,
}

impl Display for Feedback {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Feedback::Ok => write!(f, "Ok")?,
            Feedback::Error(e) => write!(f, "Error: {e}")?,
            Feedback::Registers(regs) => write!(f, "Registers: {regs:#x?}")?,
            Feedback::Word(w) => write!(f, "Word: {w:#x?}")?,
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
