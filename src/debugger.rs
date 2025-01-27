use std::collections::HashMap;
use std::ffi::CString;
use std::path::{Path, PathBuf};

use nix::sys::personality::Persona;
use nix::sys::signal::Signal;
use nix::sys::wait::{waitpid, WaitPidFlag, WaitStatus};
use nix::sys::{personality, ptrace};
use nix::unistd::{execv, Pid};
use tracing::{debug, error, info, warn};

use crate::breakpoint::Breakpoint;
use crate::errors::{DebuggerError, Result};
use crate::feedback::Feedback;
use crate::ui::{DebuggerUI, Register, Status};
use crate::{rmem, wmem, Addr, Word};

#[derive(Debug, Clone)]
pub struct Debugger<UI: DebuggerUI> {
    debuggee: Option<Debuggee>,
    ui: UI,
    executable: PathBuf,
}

#[derive(Debug, Clone)]
pub struct Debuggee {
    pid: Pid,
    breakpoints: HashMap<Addr, Breakpoint>,
}

impl Debuggee {
    pub fn kill(&self) -> Result<()> {
        ptrace::kill(self.pid)?;
        Ok(())
    }
}

impl<UI: DebuggerUI> Debugger<UI> {
    pub fn build(executable: impl AsRef<Path>, ui: UI) -> Self {
        Debugger {
            executable: executable.as_ref().to_owned(),
            debuggee: None,
            ui,
        }
    }

    pub fn launch_debuggee(&mut self, args: &[CString]) -> Result<()> {
        let path: &Path = self.executable.as_ref();
        if !path.exists() {
            let err = DebuggerError::ExecutableDoesNotExist(path.to_string_lossy().to_string());
            error!("{err}");
            return Err(err);
        }
        if !path.is_file() {
            let err = DebuggerError::ExecutableIsNotAFile(path.to_string_lossy().to_string());
            error!("{err}");
            return Err(err);
        }

        let fork_res = unsafe { nix::unistd::fork() };
        match fork_res {
            Err(e) => {
                error!("could not start executable: {e}");
                Err(e.into())
            }
            Ok(fr) => match fr {
                nix::unistd::ForkResult::Parent { child: pid } => {
                    self.debuggee = Some(Debuggee {
                        pid,
                        breakpoints: HashMap::new(),
                    });
                    Ok(())
                }
                nix::unistd::ForkResult::Child => {
                    personality::set(Persona::ADDR_NO_RANDOMIZE)?; // FIXME: maybe remove this
                    ptrace::traceme()?;
                    info!("starting the debuggee process");
                    let cpath = CString::new(path.to_string_lossy().to_string().as_str())?;
                    execv(&cpath, args)?; // NOTE: unsure if args[0] is set to the executable
                    unreachable!()
                }
            },
        }
    }

    pub fn wait(&self, options: &[WaitPidFlag]) -> Result<WaitStatus> {
        self.err_if_no_debuggee()?;
        let mut flags = WaitPidFlag::empty();
        for f in options {
            flags |= *f;
        }
        Ok(waitpid(
            self.debuggee.as_ref().unwrap().pid,
            if flags.is_empty() { None } else { Some(flags) },
        )?)
    }

    pub fn run_debugger(&mut self) -> Result<()> {
        self.wait(&[])?; // wait until the debuggee is stopped

        let mut feedback: Feedback = Feedback::Ok;
        loop {
            let ui_res = self.ui.process(&feedback);
            feedback = {
                match ui_res {
                    Err(e) => {
                        error!("{e}");
                        return Err(e);
                    }
                    Ok(s) => match s {
                        Status::DebuggerQuit => break,
                        Status::Continue => self.cont(None),
                        Status::SetBreakpoint(addr) => self.set_bp(addr),
                        Status::DelBreakpoint(addr) => self.del_bp(addr),
                        Status::DumpRegisters => self.dump_regs(),
                        Status::SetRegister(r, v) => self.set_regs(r, v),
                        Status::WriteMem(a, v) => self.write_mem(a, v),
                        Status::ReadMem(a) => self.read_mem(a),
                    },
                }
            }
            .into();
        }

        Ok(())
    }

    pub fn cont(&self, sig: Option<Signal>) -> Result<Feedback> {
        self.err_if_no_debuggee()?;
        ptrace::cont(self.debuggee.as_ref().unwrap().pid, sig)?;

        self.wait(&[])?; // wait until the debuggee is stopped again!!!
        Ok(Feedback::Ok)
    }

    pub fn dump_regs(&self) -> Result<Feedback> {
        self.err_if_no_debuggee()?;
        let dbge = self.debuggee.as_ref().unwrap();
        let regs = ptrace::getregs(dbge.pid)?;
        Ok(Feedback::Registers(regs))
    }

    fn err_if_no_debuggee(&self) -> Result<()> {
        if self.debuggee.is_none() {
            let err = DebuggerError::NoDebugee;
            error!("{err}");
            Err(err)
        } else {
            Ok(())
        }
    }

    pub fn cleanup(&self) -> Result<()> {
        if let Some(dbge) = &self.debuggee {
            dbge.kill()?;
        }
        Ok(())
    }

    pub fn set_bp(&mut self, addr: Addr) -> Result<Feedback> {
        self.err_if_no_debuggee()?;
        let dbge = self.debuggee.as_mut().unwrap();

        let mut bp = Breakpoint::new(dbge.pid, addr);
        bp.enable()?;
        dbge.breakpoints.insert(addr, bp);

        Ok(Feedback::Ok)
    }

    pub fn del_bp(&mut self, addr: Addr) -> Result<Feedback> {
        self.err_if_no_debuggee()?;
        let dbge = self.debuggee.as_mut().unwrap();
        debug!("{:#x?}", dbge.breakpoints);

        if let Some(_bp) = dbge.breakpoints.get_mut(&addr) {
            dbge.breakpoints.remove(&addr); // gets disabled on dropping
        } else {
            warn!("removed a breakpoint at {addr:x?} that did not exist");
        }

        Ok(Feedback::Ok)
    }

    pub fn set_regs(&self, r: Register, v: u64) -> Result<Feedback> {
        self.err_if_no_debuggee()?;
        let dbge = self.debuggee.as_ref().unwrap();
        let mut regs = ptrace::getregs(dbge.pid)?;

        match r {
            Register::r9 => regs.r9 = v,
            Register::r8 => regs.r8 = v,
            Register::r10 => regs.r10 = v,
            Register::r11 => regs.r11 = v,
            Register::r12 => regs.r12 = v,
            Register::r13 => regs.r13 = v,
            Register::r14 => regs.r14 = v,
            Register::r15 => regs.r15 = v,
            Register::rip => regs.rip = v,
            Register::rbp => regs.rbp = v,
            Register::rax => regs.rax = v,
            Register::rcx => regs.rcx = v,
            Register::rbx => regs.rbx = v,
            Register::rdx => regs.rdx = v,
            Register::rsi => regs.rsi = v,
            Register::rsp => regs.rsp = v,
            Register::rdi => regs.rdi = v,
            Register::orig_rax => regs.orig_rax = v,
            Register::eflags => regs.eflags = v,
            Register::es => regs.es = v,
            Register::cs => regs.cs = v,
            Register::ss => regs.ss = v,
            Register::fs_base => regs.fs_base = v,
            Register::fs => regs.fs = v,
            Register::gs_base => regs.gs_base = v,
            Register::gs => regs.gs = v,
            Register::ds => regs.ds = v,
        }

        ptrace::setregs(dbge.pid, regs)?;

        Ok(Feedback::Ok)
    }

    pub fn read_mem(&self, addr: Addr) -> Result<Feedback> {
        self.err_if_no_debuggee()?;
        let dbge = self.debuggee.as_ref().unwrap();

        let w = rmem(dbge.pid, addr)?;

        Ok(Feedback::Word(w))
    }
    pub fn write_mem(&self, addr: Addr, value: Word) -> Result<Feedback> {
        self.err_if_no_debuggee()?;
        let dbge = self.debuggee.as_ref().unwrap();

        wmem(dbge.pid, addr, value)?;

        Ok(Feedback::Ok)
    }
}
