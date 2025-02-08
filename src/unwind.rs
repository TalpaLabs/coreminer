//! Thanks to the BugStalker debugger project, which's source code for unwinding was useful in
//! getting this to work.
//!
//! <https://github.com/godzie44/BugStalker> (MIT Licensed)

use crate::errors::Result;
use crate::Addr;

use nix::unistd::Pid;
use unwind::{Accessors, AddressSpace, Byteorder, Cursor, PTraceState, RegNum};

#[derive(Debug, Clone)]
pub struct Backtrace {
    pub frames: Vec<BacktraceFrame>,
}
#[derive(Debug, Clone)]
pub struct BacktraceFrame {
    pub addr: Addr,
    pub start_addr: Option<Addr>,
    pub name: Option<String>,
}

impl Backtrace {
    fn new(frames: &[BacktraceFrame]) -> Self {
        Self {
            frames: frames.to_vec(),
        }
    }
}

pub fn unwind(pid: Pid) -> Result<Backtrace> {
    let state = PTraceState::new(pid.as_raw() as u32)?;
    let address_space = AddressSpace::new(Accessors::ptrace(), Byteorder::DEFAULT)?;
    let mut cursor = Cursor::remote(&address_space, &state)?;
    let mut frames = vec![];

    loop {
        let ip = cursor.register(RegNum::IP)?;
        match (cursor.procedure_info(), cursor.procedure_name()) {
            (Ok(ref info), Ok(ref name)) if ip == info.start_ip() + name.offset() => {
                let fn_name = format!("{:#}", rustc_demangle::demangle(name.name()));

                frames.push(BacktraceFrame {
                    name: Some(fn_name),
                    start_addr: Some(info.start_ip().into()),
                    addr: ip.into(),
                });
            }
            _ => {
                frames.push(BacktraceFrame {
                    name: None,
                    start_addr: None,
                    addr: ip.into(),
                });
            }
        }

        if !cursor.step()? {
            break;
        }
    }

    Ok(Backtrace::new(&frames))
}
