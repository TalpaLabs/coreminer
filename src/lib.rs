use std::ffi::CString;
use std::path::Path;
use std::str::FromStr;

use nix::sys::ptrace;
use nix::unistd::{execv, Pid};
use tracing::{error, info};

use self::errors::DebuggerError;

pub mod errors;

pub fn launch_debuggee(
    executable: impl AsRef<Path>,
    args: &[CString],
) -> Result<Pid, DebuggerError> {
    let path: &Path = executable.as_ref();
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
            nix::unistd::ForkResult::Parent { child } => Ok(child),
            nix::unistd::ForkResult::Child => {
                ptrace::traceme()?;
                info!("starting the debuggee process");
                let cpath = CString::from_str(path.to_string_lossy().to_string().as_str())?;
                execv(&cpath, args)?;
                unreachable!()
            }
        },
    }
}

pub fn is_loaded() -> bool {
    true
}
