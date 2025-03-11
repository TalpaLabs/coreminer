use steckrs::simple_plugin;
use tracing::info;

use crate::errors::Result;
use crate::feedback::Feedback;
use crate::feedback::Status;
use crate::plugins::extension_points::EPreSignalHandler;

use super::extension_points::EPreSignalHandlerF;

simple_plugin!(
    HelloWorldPlugin,
    "hello_world",
    "A plugin that says hello world when the debuggee gets a signal",
    hooks: [(EPreSignalHandler, SignalHello)]
);

struct SignalHello;
impl EPreSignalHandlerF for SignalHello {
    fn pre_handle_signal(
        &self,
        feedback: &Feedback,
        siginfo: &nix::libc::siginfo_t,
        sig: &nix::sys::signal::Signal,
    ) -> Result<Status> {
        info!("HELLO WORLD: {sig}");
        Ok(Status::PluginContinue)
    }
}
