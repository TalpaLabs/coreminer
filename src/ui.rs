use crate::errors::Result;
use crate::feedback::Feedback;

pub mod cli;

pub enum Status {
    DebuggerQuit,
    Nothing,
    Continue,
}

pub trait DebuggerUI {
    fn process(&self, feedback: &Feedback) -> Result<Status>;
}
