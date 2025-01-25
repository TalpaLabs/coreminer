use std::ffi::CString;
use std::path::{Path, PathBuf};
use std::str::FromStr;

use nix::sys::ptrace;
use nix::sys::wait::{waitpid, WaitPidFlag, WaitStatus};
use nix::unistd::{execv, Pid};
use tracing::{error, info};

use crate::errors::{DebuggerError, Result};
use crate::ui::{DebuggerUI, Status};

#[derive(Debug, Clone, Hash)]
pub struct Debugger<UI: DebuggerUI> {
    debuggee: Option<Debuggee>,
    ui: UI,
    executable: PathBuf,
}

#[derive(Debug, Clone, Hash, Copy)]
pub struct Debuggee {
    pid: Pid,
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
                    self.debuggee = Some(Debuggee { pid });
                    Ok(())
                }
                nix::unistd::ForkResult::Child => {
                    ptrace::traceme()?;
                    info!("starting the debuggee process");
                    let cpath = CString::from_str(path.to_string_lossy().to_string().as_str())?;
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
            self.debuggee.unwrap().pid,
            if flags.is_empty() { None } else { Some(flags) },
        )?)
    }

    pub fn run_debugger(&self) -> Result<()> {
        self.wait(&[])?; // wait until the debuggee is stopped

        loop {
            match self.ui.process_command() {
                Err(e) => {
                    error!("{e}");
                    return Err(e);
                }
                Ok(s) => match s {
                    Status::Stop => break,
                    Status::Continue => (),
                },
            }
        }

        Ok(())
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
        if let Some(dbge) = self.debuggee {
            dbge.kill()?;
        }
        Ok(())
    }
}
