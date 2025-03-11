use steckrs::{Plugin, PluginManager};
use tracing::{error, warn};

use self::hello_world::HelloWorldPlugin;

pub mod extension_points;
pub mod hello_world;

pub(crate) fn default_plugin_manager() -> PluginManager {
    let mut manager = PluginManager::new();

    #[allow(clippy::single_element_loop)] // there will be more later
    for plugin in [HelloWorldPlugin::new()] {
        load_default_plugin(&mut manager, plugin);
    }

    manager
}

fn load_default_plugin<P: Plugin>(manager: &mut PluginManager, plugin: P) {
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
