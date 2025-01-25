use nix::sys::ptrace;
use nix::unistd::Pid;

use crate::errors::{DebuggerError, Result};

pub type Addr = *mut std::ffi::c_void;

pub const INT3: i64 = 0xcc;
pub const WORD_MASK: i64 = 0xff;
pub const WORD_MASK_INV: i64 = i64::MAX ^ WORD_MASK;

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

        let data_word: i64 = ptrace::read(self.pid, self.addr)?;
        self.saved_data = Some((data_word & WORD_MASK) as u8);
        let data_word_modified: i64 = (data_word & WORD_MASK_INV) | INT3;
        ptrace::write(self.pid, self.addr, data_word_modified)?;

        Ok(())
    }

    pub fn disable(&mut self) -> Result<()> {
        if self.saved_data.is_none() {
            return Err(DebuggerError::BreakpointIsAlreadyDisabled);
        }

        let data_word: i64 = ptrace::read(self.pid, self.addr)?;
        let data_word_restored: i64 = (data_word & WORD_MASK_INV) | self.saved_data.unwrap() as i64;
        ptrace::write(self.pid, self.addr, data_word_restored)?;
        self.saved_data = None;

        Ok(())
    }
}
