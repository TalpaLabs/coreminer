use crate::errors::{DebuggerError, Result};

#[derive(Debug)]
pub enum Feedback {
    Nothing,
    Error(DebuggerError),
    Continue,
}
