use std::path::{Path, PathBuf};
use std::process::exit;

use coreminer::addr::Addr;
use coreminer::debugger::Debugger;
use coreminer::errors::DebuggerError;
use coreminer::feedback::Feedback;
use coreminer::ui::json::{Input, JsonUI};

use clap::Parser;
use coreminer::feedback::Status;
use coreminer::Word;
use serde::de::Error;
use steckrs::PluginIDOwned;
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

    if args.example_statuses {
        example_statuses();
    }
    if args.example_feedbacks {
        example_feedbacks();
    }
    if args.example_feedbacks || args.example_statuses {
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
        #[cfg(feature = "plugins")]
        Status::PluginSetEnable(PluginIDOwned::from("foobar"), true),
        #[cfg(feature = "plugins")]
        Status::PluginGetStatus(PluginIDOwned::from("foobar")),
        Status::SetBreakpoint(Addr::from(21958295usize)),
        Status::SetRegister(coreminer::Register::r9, 133719),
        Status::DumpRegisters,
        Status::Backtrace,
        Status::WriteMem(Addr::from(9218098521usize), 0xff),
        Status::ReadMem(Addr::from(9218098521usize)),
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
        Feedback::Word(921589215 as Word),
        Feedback::Word(Word::MAX),
        Feedback::Word(Word::MIN),
        Feedback::Variable(coreminer::variable::VariableValue::Bytes(vec![
            19, 13, 13, 13, 17,
        ])),
        Feedback::Error(DebuggerError::BreakpointIsAlreadyEnabled),
        Feedback::Error(DebuggerError::UnimplementedRegister(1337)),
        Feedback::Error(DebuggerError::Json(serde_json::Error::custom("test err"))),
        #[cfg(feature = "plugins")]
        Feedback::PluginStatus(Some(false)),
    ];

    for f in feedbacks {
        println!(
            "{}",
            serde_json::to_string(&JsonUI::format_feedback(f).unwrap()).unwrap()
        )
    }
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
