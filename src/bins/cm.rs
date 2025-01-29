use std::path::PathBuf;

use coreminer::debugger::Debugger;
use coreminer::errors::DebuggerError;
use coreminer::ui::cli::CliUi;

use clap::Parser;
use tracing::debug;

/// Launch the core debugger
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// The Program to launch as debuggee
    #[clap(short, long)]
    run: PathBuf,
}

fn main() -> Result<(), DebuggerError> {
    setup_logger();
    debug!("set up the logger");

    let args = Args::parse();

    let debuggee_args = Vec::new();
    let ui = CliUi::build()?;
    let mut debug: Debugger<CliUi> = Debugger::build(&args.run, ui)?;
    let obj_data_raw = std::fs::read(&args.run)?;
    let obj_data = debug.parse_exec_data(&obj_data_raw)?;
    debug.launch_debuggee(&debuggee_args, obj_data)?;
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
