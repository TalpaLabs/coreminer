use crate::errors::Result;

pub mod cli;

pub enum Status {
    Continue,
    Stop,
}

pub trait DebuggerUI {
    fn process_command(&self) -> Result<Status>;
}
