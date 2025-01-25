use nix::sys::ptrace;
use nix::unistd::Pid;

use crate::errors::{DebuggerError, Result};

pub type RawPointer = *mut std::ffi::c_void;

#[derive(Hash, Clone, Copy, Debug)]
pub struct Addr(RawPointer);

pub const INT3: i64 = 0xcc;
pub const WORD_MASK: i64 = 0xff;
pub const WORD_MASK_INV: i64 = i64::MAX ^ WORD_MASK;

#[derive(Debug, Clone, Copy, Hash)]
pub struct Breakpoint {
    addr: Addr,
    pid: Pid,
    saved_data: Option<u8>,
}

impl Breakpoint {
    pub fn new(pid: Pid, addr: Addr) -> Self {
        Self {
            pid,
            addr,
            saved_data: None,
        }
    }

    pub fn enable(&mut self) -> Result<()> {
        if self.saved_data.is_some() {
            return Err(DebuggerError::BreakpointIsAlreadyEnabled);
        }

        let data_word: i64 = ptrace::read(self.pid, self.addr.into())?;
        self.saved_data = Some((data_word & WORD_MASK) as u8);
        let data_word_modified: i64 = (data_word & WORD_MASK_INV) | INT3;
        ptrace::write(self.pid, self.addr.into(), data_word_modified)?;

        Ok(())
    }

    pub fn disable(&mut self) -> Result<()> {
        if self.saved_data.is_none() {
            return Err(DebuggerError::BreakpointIsAlreadyDisabled);
        }

        let data_word: i64 = ptrace::read(self.pid, self.addr.into())?;
        let data_word_restored: i64 = (data_word & WORD_MASK_INV) | self.saved_data.unwrap() as i64;
        ptrace::write(self.pid, self.addr.into(), data_word_restored)?;
        self.saved_data = None;

        Ok(())
    }
}

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
