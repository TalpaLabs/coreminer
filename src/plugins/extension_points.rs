//! # Extension Points
//!
//! Defines the [`ExtensionPoint`](steckrs::hook::ExtensionPoint) where [plugins](steckrs::Plugin)
//! can hook into the debugger.
//!
//! Extension points represent specific events or phases in the debugging process
//! where plugins can register hooks to execute custom behavior. Each extension point
//! defines a trait that plugin hooks must implement.
//!
//! This module uses the [`extension_point!`](steckrs::extension_point) macro from the [`steckrs`]
//! crate to define extension points in a concise and type-safe manner.
//!
//! ## Available Extension Points
//!
//! - [`EPreSignalHandler`]: Called before the debugger processes signals from the debuggee
//!
//! ## Usage
//!
//! Extension points are used by the debugger to invoke plugin hooks at appropriate times.
//! Plugins register hooks for specific extension points, and the debugger calls those hooks
//! when the corresponding event occurs.
//!
//! ```rust
//! use steckrs::simple_plugin;
//! use coreminer::plugins::extension_points::{EPreSignalHandler, EPreSignalHandlerF};
//! use coreminer::errors::Result;
//! use coreminer::feedback::{Feedback, Status};
//! use nix::sys::wait::WaitStatus;
//! use nix::libc::siginfo_t;
//! use nix::sys::signal::Signal;
//!
//! // Define a hook implementation
//! struct MySignalHandler;
//! impl EPreSignalHandlerF for MySignalHandler {
//!     fn pre_handle_signal(
//!         &self,
//!         feedback: &Feedback,
//!         siginfo: &siginfo_t,
//!         sig: &Signal,
//!         wait_status: &WaitStatus,
//!     ) -> Result<Status> {
//!         // Custom signal handling logic
//!         Ok(Status::PluginContinue)
//!     }
//! }
//!
//! // Register the hook in a plugin
//! simple_plugin!(
//!     MyPlugin,
//!     "my_plugin",
//!     "A plugin that handles signals",
//!     hooks: [(EPreSignalHandler, MySignalHandler)]
//! );
//! ```

use nix::sys::wait::WaitStatus;
use steckrs::extension_point;

use nix::libc::siginfo_t;
use nix::sys::signal::Signal;

use crate::errors::Result;
use crate::feedback::Feedback;
use crate::feedback::Status;

extension_point!(
    /// Extension point for handling signals before the debugger processes them
    ///
    /// This extension point is called whenever the debugger receives a signal from
    /// the debuggee process. It allows plugins to intercept and handle signals before
    /// the debugger's default signal handling logic, potentially changing the behavior
    /// of the debugger in response to specific signals.
    EPreSignalHandler:
    /// Functions that must be implemented by hooks for the [`EPreSignalHandler`] extension point
    EPreSignalHandlerF;
    /// Processes a signal from the debuggee before the debugger handles it
    ///
    /// This function runs in a feedback loop, allowing the hook to execute debugger
    /// commands by returning Status values and receiving Feedback from those commands.
    /// The loop continues until the hook returns Status::PluginContinue.
    ///
    /// # Parameters
    ///
    /// * `self` - The hook instance
    /// * `feedback` - The current feedback from the debugger
    /// * `siginfo` - Signal information structure from the operating system
    /// * `sig` - The signal type that was received
    /// * `wait_status` - The waitpid status information
    ///
    /// # Returns
    ///
    /// * `Ok(Status)` - The next command for the debugger to execute
    /// * `Err(DebuggerError)` - If an error occurs during signal handling
    ///
    /// # Errors
    ///
    /// Returns an error if the hook implementation fails.
    fn pre_handle_signal(&mut self, feedback: &Feedback, siginfo: &siginfo_t,
        sig: &Signal, wait_status: &WaitStatus) -> Result<Status>;
);

extension_point!(
    /// Extension point for handling SIGTRAP before the debugger processes it
    ///
    /// This extension point is called whenever the debugger receives a SIGTRAP from
    /// the debuggee process. It allows plugins to intercept and handle the SIGTRAP.
    ///
    /// SIGTRAP is normally used to stop the debugee process with breakpoints, but the debugee can also
    /// be compiled with explicit in3 instructions to make debugging harder.
    EPreSigtrap:
    /// Functions that must be implemented by hooks for the [`EPreSignalHandler`] extension point
    EPreSigtrapF;
    /// Processes a `SIGTRAP` signal from the debuggee before the debugger handles it
    ///
    /// This function runs in a feedback loop, allowing the hook to execute debugger
    /// commands by returning Status values and receiving Feedback from those commands.
    /// The loop continues until the hook returns Status::PluginContinue.
    ///
    /// # Parameters
    ///
    /// * `self` - The hook instance
    /// * `feedback` - The current feedback from the debugger
    /// * `siginfo` - Signal information structure from the operating system
    /// * `sig` - The signal type that was received
    ///
    /// # Returns
    ///
    /// * `Ok((Status, RETURN_EARLY))` - The next command for the debugger to execute, and if the
    ///                                  function should return early
    /// * `Err(DebuggerError)` - If an error occurs during signal handling
    ///
    /// # Errors
    ///
    /// Returns an error if the hook implementation fails.
    fn pre_handle_sigtrap(&mut self, feedback: &Feedback, siginfo: &siginfo_t,
        sig: &Signal) -> Result<(Status, bool)>;
);
