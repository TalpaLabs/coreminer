use nix::unistd::Pid;
use tracing::trace;

use crate::errors::{DebuggerError, Result};
use crate::{mem_read_word, mem_write_word, Addr};

pub const MASK_ALL: i64 = -1; // yup for real, two's complement
pub const INT3: i64 = 0x00000000000000cc;
pub const WORD_MASK: i64 = 0x00000000000000ff;
pub const WORD_MASK_INV: i64 = MASK_ALL ^ WORD_MASK;

#[derive(Debug, Clone, Hash)]
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

    #[inline]
    pub fn is_enabled(&self) -> bool {
        self.saved_data.is_some()
    }

    pub fn enable(&mut self) -> Result<()> {
        if self.is_enabled() {
            return Err(DebuggerError::BreakpointIsAlreadyEnabled);
        }

        // FIXME: this sets the wrong 'original'
        let data_word: i64 = mem_read_word(self.pid, self.addr)?;
        trace!("original word: {data_word:016x}");
        self.saved_data = Some((data_word & WORD_MASK) as u8);
        trace!("saved_byte: {:02x}", self.saved_data.as_ref().unwrap());
        let data_word_modified: i64 = (data_word & WORD_MASK_INV) | INT3;
        trace!("modified word: {data_word_modified:016x}");
        mem_write_word(self.pid, self.addr, data_word_modified)?;

        Ok(())
    }

    pub fn disable(&mut self) -> Result<()> {
        if !self.is_enabled() {
            return Err(DebuggerError::BreakpointIsAlreadyDisabled);
        }

        // FIXME: this sets the wrong 'original'
        let data_word: i64 = mem_read_word(self.pid, self.addr)?;
        trace!("breakpo: {data_word:016x}");
        let data_word_restored: i64 = (data_word & WORD_MASK_INV) | self.saved_data.unwrap() as i64;
        trace!("restore: {data_word_restored:016x}");
        mem_write_word(self.pid, self.addr, data_word_restored)?;
        self.saved_data = None;

        Ok(())
    }
}

impl Drop for Breakpoint {
    fn drop(&mut self) {
        if self.is_enabled() {
            self.disable()
                .expect("could not disable breakpoint while dropping")
        }
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_minus_one_has_this_representaiton() {
        assert_eq!(
            &(-1i64).to_le_bytes(),
            &[0xffu8, 0xffu8, 0xffu8, 0xffu8, 0xffu8, 0xffu8, 0xffu8, 0xffu8,]
        )
    }
}
