//! # JSON Interface
//!
//! Provides a JSON-based interface for interacting with the Coreminer debugger.
//!
//! This module implements the [`DebuggerUI`] trait to provide a programmatic
//! JSON interface suitable for integration with other tools or remote debugging
//! sessions. It communicates with clients by:
//!
//! - Reading JSON-formatted commands from stdin
//! - Writing JSON-formatted feedback to stdout
//! - Supporting the same debugging operations as the CLI interface
//!
//! This interface enables automation and integration with external tools
//! that can communicate via JSON.

use std::io::{BufRead, BufReader};

use serde::{Deserialize, Serialize};
use serde_json::json;
use tracing::error;

use crate::errors::Result;
use crate::feedback::Feedback;

use super::{DebuggerUI, Status};

/// Input command structure for JSON interface
///
/// This structure defines the format of commands sent to the debugger
/// via the JSON interface.
///
/// # Examples
///
/// ```
/// use coreminer::ui::json::Input;
/// use coreminer::feedback::Status;
/// use coreminer::addr::Addr;
/// use serde_json::json;
///
/// let json = json!({
///     "status": {
///       "SetBreakpoint": 21958295
///     }
/// });
///
/// let input: Input = serde_json::from_value(json).unwrap();
/// // Now input.status contains Status::SetBreakpoint(Addr::from(4194304usize))
/// ```
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Input {
    /// The [`Status`] of the [`DebuggerUI`]
    ///
    /// In other words, that's what is being requested
    pub status: Status,
}

/// JSON-based interface for the debugger
///
/// Implements the [`DebuggerUI`] trait to provide a JSON-based interface
/// for the debugger suitable for programmatic or remote use.
///
/// # Examples
///
/// ```no_run
/// use coreminer::ui::json::JsonUI;
/// use coreminer::ui::DebuggerUI;
/// use coreminer::feedback::Feedback;
///
/// // Create a JSON UI
/// let mut ui = JsonUI::build().unwrap();
///
/// // Process feedback from the debugger with user input
/// let status = ui.process(Feedback::Ok).unwrap();
/// ```
pub struct JsonUI;

impl JsonUI {
    /// Creates a new JSON UI instance
    ///
    /// # Returns
    ///
    /// * `Ok(JsonUI)` - A new JSON UI instance
    /// * `Err(DebuggerError)` - If creation failed
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use coreminer::ui::json::JsonUI;
    ///
    /// let ui = JsonUI::build().unwrap();
    /// ```
    ///
    /// # Errors
    ///
    /// Never fails.
    pub fn build() -> Result<Self> {
        Ok(JsonUI)
    }

    /// Formats feedback as a JSON value
    ///
    /// # Parameters
    ///
    /// * `feedback` - The feedback to format
    ///
    /// # Returns
    ///
    /// * `Ok(Value)` - The JSON formatted feedback
    /// * `Err(DebuggerError)` - If formatting failed
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - JSON serialization fails
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use coreminer::ui::json::JsonUI;
    /// # use coreminer::feedback::Feedback;
    /// let ui = JsonUI::build().unwrap();
    /// let json = JsonUI::format_feedback(&Feedback::Ok).unwrap();
    /// println!("{}", json);
    /// ```
    pub fn format_feedback(feedback: &Feedback) -> Result<serde_json::Value> {
        Ok(json!({ "feedback": feedback }))
    }
}

impl DebuggerUI for JsonUI {
    fn process(&mut self, mut feedback: crate::feedback::Feedback) -> Result<super::Status> {
        let mut reader = BufReader::new(std::io::stdin());
        let mut buf = Vec::new();
        loop {
            println!("{}", Self::format_feedback(&feedback)?);
            buf.clear();
            reader.read_until(b'\n', &mut buf)?;
            let input: Input = match serde_json::from_slice(&buf) {
                Ok(a) => a,
                Err(e) => {
                    error!("{e}");
                    feedback = Feedback::Error(e.into());
                    continue;
                }
            };
            return Ok(input.status);
        }
    }
}
