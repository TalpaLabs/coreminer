//! # User Interface Module
//!
//! Provides interfaces and implementations for interacting with the debugger.
//!
//! This module defines the core user interface abstractions used by the debugger,
//! allowing for different interface implementations (such as CLI, JSON-RPC, etc.)
//! while maintaining a consistent API for the debugger core to interact with.
//!
//! The [`DebuggerUI`] trait defines the interface for UI implementations.
//!
//! This module also includes submodules for specific UI implementations:
//! - [`cli`]: A command-line interface implementation

use crate::errors::Result;
use crate::feedback::{Feedback, Status};
use crate::Register;

#[cfg(feature = "cli")]
pub mod cli;
#[cfg(feature = "cmserve")]
pub mod json;

/// Interface for debugger user interfaces
///
/// [`DebuggerUI`] defines the interface that must be implemented by any user
/// interface that wants to interact with the debugger. It provides a way for
/// the debugger to send feedback to the UI and receive commands in return.
///
/// # Examples
///
/// ```no_run
/// use coreminer::ui::DebuggerUI;
/// use coreminer::feedback::{Feedback,Status};
/// use coreminer::errors::Result;
///
/// // A simple UI implementation that always returns Continue
/// struct SimpleUI;
///
/// impl DebuggerUI for SimpleUI {
///     fn process(&mut self, feedback: Feedback) -> Result<Status> {
///         println!("Received feedback: {}", feedback);
///         Ok(Status::Continue)
///     }
/// }
///
/// // Using the UI with a debugger
/// # fn run_example() -> Result<()> {
/// # use coreminer::debugger::Debugger;
/// let ui = SimpleUI;
/// let mut debugger = Debugger::build(ui)?;
/// debugger.run_debugger()?;
/// debugger.cleanup()?;
/// # Ok(())
/// # }
/// ```
pub trait DebuggerUI {
    /// Processes feedback from the debugger and returns a status command
    ///
    /// This method is called by the debugger to send feedback to the UI
    /// and receive a command in response. The UI implementation should
    /// present the feedback to the user in an appropriate way and then
    /// return a command based on user input or internal logic.
    ///
    /// # Parameters
    ///
    /// * `feedback` - The feedback from the debugger
    ///
    /// # Returns
    ///
    /// * `Ok(Status)` - The command to send to the debugger
    /// * `Err(DebuggerError)` - If an error occurred during processing
    ///
    /// # Errors
    ///
    /// This method can fail if there are issues with user input or other
    /// UI-specific errors.
    fn process(&mut self, feedback: Feedback) -> Result<Status>;
}
