use steckrs::extension_point;

use nix::libc::siginfo_t;
use nix::sys::signal::Signal;

use crate::errors::Result;

extension_point!(
    EPreSignalHandler: EPreSignalHandlerF,
    // true if the signal should be ignored
    fn pre_handle_signal(&self, siginfo: &siginfo_t, sig: &Signal) -> bool,
);

extension_point!(
    EOnSigTrap: EOnSigTrapF,
    // true if the signal should be ignored
    fn handle_sigtrap(&self, siginfo: &siginfo_t, sig: &Signal) -> Result<()>,
);
