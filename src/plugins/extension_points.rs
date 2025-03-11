#![allow(missing_docs)] // TODO: add proper docs
#![allow(clippy::missing_errors_doc)]

use steckrs::extension_point;

use nix::libc::siginfo_t;
use nix::sys::signal::Signal;

use crate::errors::Result;
use crate::feedback::Feedback;
use crate::ui::Status;

extension_point!(
    // [ExtensionPoint](steckrs::hook::ExtensionPoint) called before signals to the debuggee are
    // handled
    EPreSignalHandler:
    // Functions implemented by [EPreSignalHandler]
    EPreSignalHandlerF;
    // Will be called in a feedback loop where you can issue regular debugger commands.
    //
    // # Errors
    //
    // Will error when the implementing plugin somehow fails
    fn pre_handle_signal(&self, feedback: &Feedback, siginfo: &siginfo_t, sig: &Signal) -> Result<Status>;
);
