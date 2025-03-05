use std::path::PathBuf;

use coreminer::debugger::Debugger;
use coreminer::errors::DebuggerError;
use coreminer::ui::cli::CliUi;

use clap::Parser;
use tracing::trace;

/// Coreminer - A powerful low-level debugger for Linux
///
/// Coreminer is designed to debug ELF binaries that may be resistant to standard
/// debugging approaches. It provides a feature-rich command-line interface with
/// capabilities including memory inspection, breakpoint management, register manipulation,
/// and DWARF debug symbol resolution.
#[derive(Parser, Debug)]
#[command(
    author,
    version,
    about,
    long_about,
    help_template = r#"{about-section}
{usage-heading} {usage}
{all-args}{tab}

{name}: v{version}
Authors: {author-with-newline}
"#
)]
struct Args {
    /// Optional path to the executable to debug with the run command
    ///
    /// If provided, this executable will be loaded automatically
    /// and can be run with the 'run' command without arguments.
    default_executable: Option<PathBuf>,
}

fn main() -> Result<(), DebuggerError> {
    setup_logger();

    let args = Args::parse();

    let ui = CliUi::build(args.default_executable.as_deref())?;
    let mut debug: Debugger<CliUi> = Debugger::build(ui)?;
    debug.run_debugger()?;
    debug.cleanup()?;

    Ok(())
}

fn setup_logger() {
    // construct a subscriber that prints formatted traces to stdout
    let subscriber = tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .without_time()
        .finish();
    // use that subscriber to process traces emitted after this point
    tracing::subscriber::set_global_default(subscriber).expect("could not setup logger");
    trace!("set up the logger");
}
