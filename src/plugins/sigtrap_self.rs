use nix::sys::signal::Signal::SIGTRAP;
use steckrs::simple_plugin;
use tracing::{debug, info, warn};

use crate::addr::Addr;
use crate::breakpoint::Breakpoint;
use crate::errors::DebuggerError;
use crate::feedback::{Feedback, Status};
use crate::ui::DebuggerUI;

use super::extension_points::{EPreSigtrap, EPreSigtrapF};

simple_plugin!(
    SigtrapInjectorPlugin,
    "sigtrap_injector",
    "Handles programs that use int3 on their own and register their own signal handler for SIGTRAP",
    hooks: [(EPreSigtrap, SigtrapInjectionGuard::default())]
);

#[derive(Default)]
struct SigtrapInjectionGuard {
    rip: Option<Addr>,
    bp: Option<Breakpoint>,
}

impl EPreSigtrapF for SigtrapInjectionGuard {
    fn pre_handle_sigtrap(
        &mut self,
        feedback: &crate::feedback::Feedback,
        siginfo: &nix::libc::siginfo_t,
        sig: &nix::sys::signal::Signal,
    ) -> crate::errors::Result<(crate::feedback::Status, bool)> {
        if *sig != SIGTRAP {
            return Ok((Status::PluginContinue, false));
        }

        let rip = match (feedback, self.rip) {
            (_, Some(addr)) => addr,
            (crate::feedback::Feedback::Registers(regs), None) => {
                self.rip = Some(regs.rip.into());
                self.rip.unwrap()
            }
            (_, _) => return Ok((Status::DumpRegisters, false)),
        };

        debug!("rip: {rip}");

        let maybe_bp: Option<&Breakpoint> = match (feedback, self.bp.as_ref()) {
            (_, Some(bp)) => Some(bp),
            (crate::feedback::Feedback::Breakpoint(bp), None) => bp.as_ref(),
            _ => return Ok((Status::GetBreakpoint(rip), false)),
        };

        if let Some(bp) = maybe_bp {
            info!("It's just a regular breakpoint: {bp:?}");
            Ok((Status::PluginContinue, false))
        } else {
            warn!("The debugger stopped with SIGTRAP, but there is no breakpoint there!");

            if matches!(feedback, Feedback::Ok) {
                // we're done
                Ok((Status::PluginContinue, false))
            } else {
                Ok((Status::SetLastSignal(siginfo.si_signo), false))
            }
        }
    }
}
