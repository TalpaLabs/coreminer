#![allow(missing_docs)] // TODO: add proper docs
#![allow(clippy::missing_errors_doc)]

use steckrs::extension_point;

use nix::libc::siginfo_t;
use nix::sys::signal::Signal;

use crate::errors::Result;
use crate::feedback::Feedback;
use crate::ui::Status;

/// # Examples
/// ```ignore
/// for_hooks!(for hook[EPreSignalHandler] in self {
///         self.hook_feedback_loop(hook, |f| {
///             Ok(Status::Continue)
///         })?;
///     }
/// );
/// ```
#[macro_export]
macro_rules! for_hooks {
    (for $hook_var:ident[$extension_point:ident] in $debugger:ident $body:block) => {
        let plugins = $debugger.plugins();
        let plugins_lock = plugins.lock().unwrap();
        let hooks: Vec<&Hook<$extension_point>> = plugins_lock
            .hook_registry()
            .get_by_extension_point::<EPreSignalHandler>();

        for $hook_var in hooks {
            $body
        }
    };
}

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
