use coreminer::debugger::Debugger;
use coreminer::errors::DebuggerError;
use coreminer::ui::cli::CliUi;

use clap::Parser;
use tracing::debug;

/// Launch the core debugger
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /*
     * this example does not actually need extra arguments, all configured in the run_debugger
     * function interactively
     */
}

fn main() -> Result<(), DebuggerError> {
    setup_logger();
    debug!("set up the logger");

    let _args = Args::parse();

    let ui = CliUi::build()?;
    let mut debug: Debugger<CliUi> = Debugger::build(ui)?;
    debug.run_debugger()?;
    debug.cleanup()?;

    Ok(())
}

fn setup_logger() {
    // construct a subscriber that prints formatted traces to stdout
    let subscriber = tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE)
        .without_time()
        .finish();
    // use that subscriber to process traces emitted after this point
    tracing::subscriber::set_global_default(subscriber).expect("could not setup logger");
}
