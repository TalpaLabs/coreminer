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

/// Launch the core debugger
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(long)]
    /// Print example status inputs and exit
    example_statuses: bool,
    #[arg(long)]
    /// Print example status inputs and exit
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
        .finish();
    // use that subscriber to process traces emitted after this point
    tracing::subscriber::set_global_default(subscriber).expect("could not setup logger");
    trace!("set up the logger");
}
