use steckrs::extension_point;

use nix::libc::siginfo_t;
use nix::sys::signal::Signal;

extension_point!(
    PreSignalHandler: PreSignalHandlerF,
    // true if the signal should be ignored
    fn pre_handle_signal(&self, siginfo: &siginfo_t, sig: &Signal) -> bool,
);
