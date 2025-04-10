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

    #[arg(short, long)]
    /// Do not log anything
    quiet: bool,

    #[arg(long)]
    /// Log into a logfile instead of stderr
    logfile: Option<PathBuf>,
}

fn main() -> Result<(), DebuggerError> {
    let args = Args::parse();

    if !args.quiet {
        setup(args.logfile);
    }

    let ui = CliUi::build(args.default_executable.as_deref())?;
    let mut debug: Debugger<CliUi> = Debugger::build(ui)?;
    debug.run_debugger()?;
    debug.cleanup()?;

    Ok(())
}

fn setup(logfile: Option<PathBuf>) {
    human_panic::setup_panic!();

    if let Some(lf) = logfile {
        let file = match std::fs::File::options().create(true).append(true).open(lf) {
            Ok(f) => f,
            Err(e) => {
                eprintln!("could not setup logfile: {e}");
                std::process::exit(1);
            }
        };

        // construct a subscriber that prints formatted traces to file
        let subscriber = tracing_subscriber::fmt()
            .with_max_level(
                #[cfg(debug_assertions)]
                tracing::Level::TRACE,
                #[cfg(not(debug_assertions))]
                tracing::Level::INFO,
            )
            .without_time()
            .with_file(false)
            .with_target(false)
            .with_writer(file)
            .finish();
        tracing::subscriber::set_global_default(subscriber).expect("could not setup logger");
    } else {
        let subscriber = tracing_subscriber::fmt()
            .with_max_level(
                #[cfg(debug_assertions)]
                tracing::Level::TRACE,
                #[cfg(not(debug_assertions))]
                tracing::Level::INFO,
            )
            .without_time()
            .with_file(false)
            .with_target(false)
            .with_writer(std::io::stderr)
            .finish();
        tracing::subscriber::set_global_default(subscriber).expect("could not setup logger");
    }

    trace!("set up the logger");
}
