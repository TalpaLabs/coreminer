use std::fmt::Display;

use nix::libc::user_regs_struct;

use crate::errors::DebuggerError;

#[derive(Debug)]
pub enum Feedback {
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
        }

        Ok(())
    }
}
