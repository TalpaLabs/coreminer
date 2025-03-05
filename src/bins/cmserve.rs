use std::path::Path;
use std::process::exit;

use coreminer::addr::Addr;
use coreminer::debugger::Debugger;
use coreminer::errors::DebuggerError;
use coreminer::feedback::Feedback;
use coreminer::ui::json::{Input, JsonUI};

use clap::Parser;
use coreminer::ui::Status;
use tracing::trace;

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
/// Coreminer Server - JSON interface for programmatic debugging
///
/// Provides a JSON-based interface to the Coreminer debugger, enabling integration
/// with UIs such as hardhat or scripting. Communicates via standard input/output
/// using JSON for both commands and responses.
///
/// cmserve does not implement the Debugger Adapter Protocol as of yet and instead uses
/// a simplified protocol based on internal datastructures.
struct Args {
    #[arg(long)]
    /// Print example JSON commands and exit
    ///
    /// Displays sample JSON structures for sending commands to the debugger
    example_statuses: bool,

    #[arg(long)]
    /// Print example JSON responses and exit
    ///
    /// Displays sample JSON structures for the responses sent by the debugger
    example_feedbacks: bool,
}

fn main() -> Result<(), DebuggerError> {
    setup_logger();

    let args = Args::parse();

    if args.example_statuses {
        example_statuses();
        exit(0);
    }
    if args.example_feedbacks {
        example_feedbacks();
        exit(0);
    }

    let ui = JsonUI::build()?;
    let mut debug: Debugger<_> = Debugger::build(ui)?;
    debug.run_debugger()?;
    debug.cleanup()?;

    Ok(())
}

fn example_statuses() {
    let statuses: &[Status] = &[
        Status::StepOut,
        Status::DebuggerQuit,
        Status::Continue,
        Status::ProcMap,
        Status::SetBreakpoint(Addr::from(21958295usize)),
        Status::SetRegister(coreminer::Register::r9, 133719),
        Status::DumpRegisters,
        Status::Backtrace,
        Status::Run(
            Path::new("/bin/ls").into(),
            vec![c"/etc".into(), c"-la".into()],
        ),
        Status::GetSymbolsByName("main".to_string()),
        Status::DisassembleAt(Addr::from(1337139usize), 50, false),
    ];

    for s in statuses {
        println!(
            "{}",
            serde_json::to_string(&Input { status: s.clone() }).unwrap()
        )
    }
}

fn example_feedbacks() {
    let feedbacks: &[Feedback] = &[
        Feedback::Ok,
        Feedback::Word(921589215i64),
        Feedback::Variable(coreminer::variable::VariableValue::Bytes(vec![
            19, 13, 13, 13, 17,
        ])),
    ];

    for f in feedbacks {
        println!(
            "{}",
            serde_json::to_string(&JsonUI::format_feedback(f)).unwrap()
        )
    }
}

fn setup_logger() {
    // construct a subscriber that prints formatted traces to stdout
    let subscriber = tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .without_time()
        .with_file(false)
        .with_target(false)
        .finish();
    // use that subscriber to process traces emitted after this point
    tracing::subscriber::set_global_default(subscriber).expect("could not setup logger");
    trace!("set up the logger");
}
