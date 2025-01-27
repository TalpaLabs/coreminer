use nix::sys::ptrace;
use nix::unistd::Pid;

use crate::errors::Result;

pub mod breakpoint;
pub mod debugger;
pub mod errors;
pub mod feedback;
pub mod ui;

pub type RawPointer = *mut std::ffi::c_void;

#[derive(Hash, Clone, Copy, Debug, PartialEq, Eq)]
pub struct Addr(pub RawPointer);

impl From<RawPointer> for Addr {
    fn from(value: RawPointer) -> Self {
        Addr(value)
    }
}

impl From<Addr> for RawPointer {
    fn from(value: Addr) -> Self {
        value.0
    }
}

impl From<usize> for Addr {
    fn from(value: usize) -> Self {
        Addr(value as RawPointer)
    }
}

pub(crate) fn wmem(pid: Pid, addr: Addr, value: i64) -> Result<()> {
    Ok(ptrace::write(pid, addr.into(), value)?)
}

pub(crate) fn rmem(pid: Pid, addr: Addr) -> Result<i64> {
    Ok(ptrace::read(pid, addr.into())?)
}
