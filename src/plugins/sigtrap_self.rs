use nix::sys::signal::Signal::SIGTRAP;
use steckrs::simple_plugin;
use tracing::{debug, info, trace, warn};

use crate::addr::Addr;
use crate::breakpoint::Breakpoint;
use crate::feedback::{Feedback, Status};

use super::extension_points::{EPreSigtrap, EPreSigtrapF};

simple_plugin!(
    SigtrapGuardPlugin,
    "sigtrap_guard",
    "Handles programs that use int3 on their own and register their own signal handler for SIGTRAP",
    hooks: [(EPreSigtrap, SigtrapInjectionGuard::default())]
);

#[derive(Default)]
struct SigtrapInjectionGuard {
    rip: Option<Addr>,
    // it looks ridiculous but it is exactly what we need here
    #[allow(clippy::option_option)]
    bp: Option<Option<Breakpoint>>,
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
            (_, Some(addr)) => {
                trace!("using stored rip");
                addr
            }
            (crate::feedback::Feedback::Registers(regs), None) => {
                self.rip = Some(regs.rip.into());
                self.rip.unwrap()
            }
            (_, _) => return Ok((Status::DumpRegisters, false)),
        };

        let maybe_bp: Option<&Breakpoint> = match (feedback, &self.bp) {
            (_, Some(bp)) => {
                trace!("using stored bp");
                bp.as_ref()
            }
            (crate::feedback::Feedback::Breakpoint(bp), None) => {
                self.bp = Some(bp.clone());
                bp.as_ref()
            }
            _ => return Ok((Status::GetBreakpoint(rip), false)),
        };

        if let Some(bp) = maybe_bp {
            info!("It's just a regular breakpoint: {bp:?}");
            Ok((Status::PluginContinue, false))
        } else if matches!(feedback, Feedback::Ok) {
            // we're done
            Ok((Status::PluginContinue, true))
        } else {
            warn!("The debugger stopped with SIGTRAP, but there is no breakpoint there!");
            warn!("This is likely a self inserted interrupt by the debuggee program!");
            warn!("Forwarding the SIGTRAP to the debuggee");
            Ok((Status::SetLastSignal(siginfo.si_signo), false))
        }
    }
}
