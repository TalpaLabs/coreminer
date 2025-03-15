//! # Plugin System
//!
//! Provides plugin functionality for extending the debugger with custom behaviors.
//!
//! This module implements a plugin system using the [`steckrs`] crate, allowing:
//! - Loading and managing plugins
//! - Defining extension points where plugins can hook into the debugger
//! - Executing plugin hooks at specific points in the debugging lifecycle
//!
//! The plugin system is structured around extension points where plugins can register
//! hooks to be called when certain events occur during debugging, such as receiving signals
//! or setting breakpoints.
//!
//! ## Architecture
//!
//! - [`ExtensionPoint`](steckrs::hook::ExtensionPoint)s define interfaces for plugins to implement
//! - [`Plugin`]s register hooks that implement these extension points
//! - The debugger invokes hooks at appropriate times during execution
//!
//! ## Default Plugins
//!
//! The module includes several built-in plugins:
//! - [`SigtrapGuardPlugin`]: A plugin that prevents the detection of the coreminer debugger with
//!                           a signal handler for SIGTRAP
//!
//! ## Usage
//!
//! The plugin system is conditionally compiled when the `plugins` feature is enabled.
//! The [`for_hooks!`](crate::for_hooks) macro provides a convenient way to iterate over
//! all enabled hooks for a specific extension point.
//!
//! # Examples
//!
//! ```no_run
//! # use steckrs::hook::ExtensionPoint;
//! # use steckrs::extension_point;
//! # use coreminer::feedback::{Feedback, Status};
//! # use coreminer::plugins::extension_points::EPreSignalHandler;
//! # use coreminer::for_hooks;
//! # use coreminer::ui::DebuggerUI;
//! # use coreminer::debugger::Debugger;
//! # use coreminer::addr::Addr;
//!
//! // use some extension point
//!
//! extension_point!(
//!     EEasyPoint: EEasyPointF;
//!     fn hello(&self) -> String;
//! );
//!
//! # fn helper<UI: DebuggerUI>(debugger: &mut Debugger<UI>) {
//!
//! // Inside a method that has access to hooks
//! // Iterate over all activated hooks for that extension point
//! for_hooks!(
//!     for hook[EEasyPoint] in debugger {
//!         println!("{}", hook.inner().hello());
//!     }
//! );
//! # }
//! ```

use steckrs::{Plugin, PluginManager};
use tracing::{error, warn};

use self::sigtrap_guard::SigtrapGuardPlugin;

pub mod extension_points;

pub mod sigtrap_guard;

/// Creates the default plugin manager with built-in plugins already loaded and activated
///
/// This function initializes a new [`PluginManager`] and loads all default plugins,
/// enabling them automatically. It's used when initializing the debugger.
///
/// # Returns
///
/// A [`PluginManager`] with all default plugins loaded and enabled
///
/// # Examples
///
/// ```no_run
/// use coreminer::plugins::default_plugin_manager;
/// use std::sync::{Arc, Mutex};
///
/// let plugin_manager = Arc::new(Mutex::new(default_plugin_manager()));
/// ```
///
/// # Panics
///
/// Panics if loading or configuring the default plugins fails.
#[must_use]
pub fn default_plugin_manager() -> PluginManager {
    let mut manager = PluginManager::new();

    let st_plugin = SigtrapGuardPlugin::new();
    load_plugin(&mut manager, st_plugin);

    manager
}

/// Loads and enables a [`Plugin`] in the [`PluginManager`]
///
/// This helper function attempts to load and enable a plugin, handling any errors
/// that occur during the process. If the plugin cannot be enabled, it attempts
/// to unload it to prevent issues.
///
/// # Parameters
///
/// * `manager` - The plugin manager to load the plugin into
/// * `plugin` - The plugin to load and enable
///
/// # Type Parameters
///
/// * `P` - A type that implements the [`Plugin`] trait
pub fn load_plugin<P: Plugin>(manager: &mut PluginManager, plugin: P) {
    let id = plugin.id();
    if let Err(e) = manager.load_plugin(Box::new(plugin)) {
        error!("Could not load plugin {id}: {e}");
    }
    if let Err(e) = manager.enable_plugin(id) {
        error!("Could not enable plugin {id}: {e}");
        warn!("Trying to unload plugin {id} because of previous error");
        match manager.unload_plugin(id) {
            Ok(()) => warn!("unloading {id} was successfull"),
            Err(e) => error!("unloading {id} was failed: {e}"),
        }
    }
}

/// Executes code for each enabled hook implementing a specific extension point
///
/// This macro simplifies the process of iterating over all enabled hooks for a specific
/// extension point and running code for each hook. It handles locking the plugin manager
/// and retrieving the appropriate hooks.
///
/// # Parameters
///
/// * `$hook_var` - The variable name to bind each hook to within the body
/// * `$extension_point` - The extension point type to find hooks for
/// * `$debugger` - The debugger instance that contains the plugin manager
/// * `$body` - The code block to execute for each hook
///
/// # Examples
///
/// ```no_run
/// # use steckrs::hook::ExtensionPoint;
/// # use coreminer::feedback::{Feedback, Status};
/// # use coreminer::plugins::extension_points::EPreSignalHandler;
/// # use coreminer::for_hooks;
/// # use coreminer::ui::DebuggerUI;
/// # use coreminer::debugger::Debugger;
/// # use coreminer::addr::Addr;
/// # fn helper<UI: DebuggerUI>(debugger: &mut Debugger<UI>) {
///
/// // Inside a method that has access to hooks
/// for_hooks!(
///     for hook[EPreSignalHandler] in debugger {
///         debugger.hook_feedback_loop(hook, |feedback| {
///             // Process the feedback and return a new status
///             println!("Received feedback: {}", feedback);
///
///             if let Feedback::Word(w) = feedback {
///                 println!("got word {w}");
///                 Ok(Status::PluginContinue)
///             } else {
///                 Ok(Status::ReadMem(Addr::from(0xdeadbeef_usize)))
///             }
///         }).unwrap();
///     }
/// );
/// # }
/// ```
///
/// # Panics
///
/// This macro will panic if it cannot acquire the lock on the plugin manager.
#[macro_export]
macro_rules! for_hooks {
    (for $hook_var:ident[$extension_point:ident] in $debugger:ident $body:block) => {
        let plugins = $debugger.plugins();
        trace!("locking plugins");
        let mut plugins_lock = plugins
            .lock()
            .expect("failed to lock the plugin manager of the coreminer debugger");
        let hooks: Vec<(_, &mut steckrs::hook::Hook<$extension_point>)> =
            plugins_lock.get_enabled_hooks_by_ep_mut::<$extension_point>();
        for (_, $hook_var) in hooks {
            {
                $body
            }
        }
        drop(plugins_lock);
        trace!("unlocking plugins");
    };
}
