use crate::breakpoint::Addr;
use crate::errors::Result;
use crate::feedback::Feedback;

pub mod cli;

pub enum Status {
    DebuggerQuit,
    Continue,
    SetBreakpoint(Addr),
    DelBreakpoint(Addr),
    DumpRegisters,
}

pub trait DebuggerUI {
    fn process(&mut self, feedback: &Feedback) -> Result<Status>;
}
