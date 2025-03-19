use nix::sys::wait::WaitStatus;
use steckrs::simple_plugin;
use tracing::info;

use coreminer::addr::Addr;
use coreminer::errors::Result;
use coreminer::feedback::Feedback;
use coreminer::feedback::Status;
use coreminer::plugins::extension_points::EPreSignalHandler;
use coreminer::plugins::extension_points::EPreSignalHandlerF;

simple_plugin!(
    HelloWorldPlugin,
    "hello_world",
    "A plugin that says hello world when the debuggee gets a signal",
    hooks: [(EPreSignalHandler, SignalHello)]
);

struct SignalHello;
impl EPreSignalHandlerF for SignalHello {
    fn pre_handle_signal(
        &mut self,
        feedback: &Feedback,
        siginfo: &nix::libc::siginfo_t,
        sig: &nix::sys::signal::Signal,
        wait_status: &WaitStatus,
    ) -> Result<Status> {
        match feedback {
            Feedback::Stack(s) => {
                info!("HELLO: got stack: {s:?}");
                Ok(Status::ProcMap)
            }
            Feedback::ProcessMap(pm) => {
                let base = pm.regions.first().map_or(Addr::NULL, |r| r.start_address);
                info!("HELLO: base_addr: {base}");
                Ok(Status::PluginContinue)
            }
            _ => {
                info!("HELLO: received {sig}");
                info!("HELLO: status {wait_status:?}; info: {siginfo:?}");
                Ok(Status::GetStack)
            }
        }
    }
}

fn main() {
    unimplemented!("This is an example implementation of a plugin. It would have to be linked against the coreminer. This is no application of it's own")
}
