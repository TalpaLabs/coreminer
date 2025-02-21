use std::collections::HashMap;
use std::ffi::CString;
use std::fmt::Display;
use std::path::Path;

use iced_x86::FormatterTextKind;
use nix::sys::ptrace;
use nix::sys::signal::Signal;
use nix::sys::wait::{waitpid, WaitPidFlag, WaitStatus};
use nix::unistd::execv;
use tracing::{debug, error, info, trace, warn};

use crate::breakpoint::Breakpoint;
use crate::consts::{SI_KERNEL, TRAP_BRKPT, TRAP_TRACE};
use crate::dbginfo::{CMDebugInfo, OwnedSymbol};
use crate::debuggee::Debuggee;
use crate::disassemble::Disassembly;
use crate::dwarf_parse::FrameInfo;
use crate::errors::{DebuggerError, Result};
use crate::feedback::Feedback;
use crate::ui::{DebuggerUI, Status};
use crate::variable::{VariableExpression, VariableValue};
use crate::{mem_read_word, mem_write_word, unwind, Addr, Register, Word};

pub struct Debugger<'executable, UI: DebuggerUI> {
    pub(crate) debuggee: Option<Debuggee>,
    pub(crate) ui: UI,
    stored_obj_data: Option<object::File<'executable>>,
    stored_obj_data_raw: Vec<u8>,
}

impl<'executable, UI: DebuggerUI> Debugger<'executable, UI> {
    pub fn build(ui: UI) -> Result<Self> {
        Ok(Debugger {
            debuggee: None,
            ui,
            stored_obj_data: None,
            stored_obj_data_raw: Vec::new(),
        })
    }

    fn launch_debuggee(&mut self, path: impl AsRef<Path>, args: &[CString]) -> Result<()> {
        let path: &Path = path.as_ref();
        if !path.exists() {
            let err = DebuggerError::ExecutableDoesNotExist(path.to_string_lossy().to_string());
            error!("{err}");
            return Err(err);
        }
        if !path.is_file() {
            let err = DebuggerError::ExecutableIsNotAFile(path.to_string_lossy().to_string());
            error!("{err}");
            return Err(err);
        }

        let executable_obj_data: object::File<'_> = self.stored_obj_data.take().unwrap();

        let dbginfo: CMDebugInfo = CMDebugInfo::build(executable_obj_data)?;

        let fork_res = unsafe { nix::unistd::fork() };
        match fork_res {
            Err(e) => {
                error!("could not start executable: {e}");
                Err(e.into())
            }
            Ok(fr) => match fr {
                nix::unistd::ForkResult::Parent { child: pid } => {
                    let dbge = Debuggee::build(pid, dbginfo, HashMap::new())?;
                    self.debuggee = Some(dbge);
                    Ok(())
                }
                nix::unistd::ForkResult::Child => {
                    let cpath = CString::new(path.to_string_lossy().to_string().as_str())?;
                    ptrace::traceme()
                        .inspect_err(|e| eprintln!("error while doing traceme: {e}"))?;
                    execv(&cpath, args)?; // NOTE: unsure if args[0] is set to the executable
                    unreachable!()
                }
            },
        }
    }

    pub fn wait_signal(&self) -> Result<Feedback> {
        let dbge = self.debuggee.as_ref().ok_or(DebuggerError::NoDebugee)?;

        loop {
            match self.wait(&[])? {
                WaitStatus::Exited(_, exit_code) => {
                    return Ok(Feedback::Exit(exit_code));
                }
                WaitStatus::Signaled(_, signal, _) => {
                    debug!("Debuggee terminated by signal: {}", signal);
                    return Ok(Feedback::Exit(-1));
                }
                _ => {
                    // Get and handle other signals as before
                    let siginfo = ptrace::getsiginfo(dbge.pid)?;
                    let sig = Signal::try_from(siginfo.si_signo)?;
                    match sig {
                        Signal::SIGTRAP => {
                            self.handle_sigtrap(sig, siginfo)?;
                            return Ok(Feedback::Ok);
                        }
                        Signal::SIGSEGV
                        | Signal::SIGINT
                        | Signal::SIGPIPE
                        | Signal::SIGSTOP
                        | Signal::SIGWINCH
                        | Signal::SIGILL => {
                            self.handle_important_signal(sig, siginfo)?;
                            return Ok(Feedback::Ok);
                        }
                        _ => {
                            self.handle_other_signal(sig, siginfo)?;
                            continue;
                        }
                    }
                }
            }
        }
    }

    pub fn wait(&self, options: &[WaitPidFlag]) -> Result<WaitStatus> {
        let mut flags = WaitPidFlag::empty();
        for f in options {
            flags |= *f;
        }
        Ok(waitpid(
            self.debuggee.as_ref().unwrap().pid,
            if flags.is_empty() { None } else { Some(flags) },
        )?)
    }

    pub fn run_debugger(&mut self) -> Result<()> {
        if let Some(dbge) = self.debuggee.as_ref() {
            self.wait(&[])?; // wait until the debuggee is stopped
            let fun =
                dbge.get_function_by_addr(Addr::from_relative(dbge.get_base_addr()?, 0x1140))?;
            debug!("function at 0x1140: {fun:#?}");
            let root_syms = dbge.symbols();
            debug!("root symbols:\n{root_syms:#?}");

            info!("PID: {}", dbge.pid);
            info!("base addr: {}", dbge.get_base_addr()?);
        } else {
            info!("debuggee not yet launched")
        }

        let mut feedback: Feedback = Feedback::Ok;
        loop {
            let ui_res = self.ui.process(feedback);
            feedback = {
                match ui_res {
                    Err(e) => {
                        error!("{e}");
                        return Err(e);
                    }
                    Ok(s) => match s {
                        Status::Infos => self.infos(),
                        Status::DebuggerQuit => break,
                        Status::Continue => self.cont(None),
                        Status::SetBreakpoint(addr) => self.set_bp(addr),
                        Status::DelBreakpoint(addr) => self.del_bp(addr),
                        Status::DumpRegisters => self.dump_regs(),
                        Status::SetRegister(r, v) => self.set_reg(r, v),
                        Status::WriteMem(a, v) => self.write_mem(a, v),
                        Status::ReadMem(a) => self.read_mem(a),
                        Status::DisassembleAt(a, l) => self.disassemble_at(a, l),
                        Status::GetSymbolsByName(s) => self.get_symbol_by_name(s),
                        Status::StepSingle => self.single_step(),
                        Status::StepOut => self.step_out(),
                        Status::StepInto => self.step_into(),
                        Status::StepOver => self.step_over(),
                        Status::Backtrace => self.backtrace(),
                        Status::ReadVariable(va) => self.read_variable(va),
                        Status::WriteVariable(va, val) => self.write_variable(va, val),
                        Status::GetStack => self.get_stack(),
                        Status::ProcMap => self.get_process_map(),
                        Status::Run(exe, args) => self.run(&exe, &args),
                    },
                }
            }
            .into();

            // Clean up if process exited
            if let Feedback::Exit(_) = feedback {
                self.debuggee = None;
            }
        }

        Ok(())
    }

    pub fn cont(&mut self, sig: Option<Signal>) -> Result<Feedback> {
        let dbge = self.debuggee.as_ref().ok_or(DebuggerError::NoDebugee)?;
        ptrace::cont(dbge.pid, sig)?;

        self.wait_signal() // wait until the debuggee is stopped again!!!
    }

    pub fn dump_regs(&self) -> Result<Feedback> {
        let dbge = self.debuggee.as_ref().ok_or(DebuggerError::NoDebugee)?;
        let regs = ptrace::getregs(dbge.pid)?;
        Ok(Feedback::Registers(regs))
    }

    pub fn cleanup(&self) -> Result<()> {
        if let Some(dbge) = &self.debuggee {
            dbge.kill()?;
        }
        Ok(())
    }

    pub fn set_bp(&mut self, addr: Addr) -> Result<Feedback> {
        let dbge = self.debuggee.as_mut().unwrap();
        let mut bp = Breakpoint::new(dbge.pid, addr);
        bp.enable()?;
        dbge.breakpoints.insert(addr, bp);

        Ok(Feedback::Ok)
    }

    pub fn del_bp(&mut self, addr: Addr) -> Result<Feedback> {
        let dbge = self.debuggee.as_mut().unwrap();

        if let Some(_bp) = dbge.breakpoints.get_mut(&addr) {
            dbge.breakpoints.remove(&addr); // gets disabled on dropping
        } else {
            warn!("removed a breakpoint at {addr:x?} that did not exist");
        }

        Ok(Feedback::Ok)
    }

    fn atomic_single_step(&self) -> Result<()> {
        let dbge = self.debuggee.as_ref().ok_or(DebuggerError::NoDebugee)?;

        // FIXME: this is probably noticeable
        if let Err(e) = ptrace::step(dbge.pid, None) {
            error!("could not do atomic step: {e}");
            return Err(e.into());
        }

        Ok(())
    }

    pub fn single_step(&mut self) -> Result<Feedback> {
        if self.go_back_step_over_bp()? {
            info!("breakpoint before, caught up and continueing with single step")
        }
        let dbge = self.debuggee.as_ref().ok_or(DebuggerError::NoDebugee)?;

        let maybe_bp_addr: Addr = self.get_current_addr()?;
        if dbge.breakpoints.contains_key(&maybe_bp_addr) {
            trace!("step over instruction with breakpoint");
            self.dse(maybe_bp_addr)?;
        } else {
            trace!("step regular instruction");
            self.atomic_single_step()?;
            self.wait_signal()?;
        }
        trace!("now at {:018x}", self.get_reg(Register::rip)?);

        Ok(Feedback::Ok)
    }

    pub fn step_out(&mut self) -> Result<Feedback> {
        {
            let a = self
                .debuggee
                .as_ref()
                .unwrap()
                .get_function_by_addr(self.get_reg(Register::rip)?.into())?;
            if let Some(s) = a {
                debug!("step out in following function: {s:#?}");
                if s.name() == Some("main") {
                    error!("you're about to do something stupid: no stepping out of the earliest stack frame allowed");
                    return Err(DebuggerError::StepOutMain);
                }
            } else {
                warn!("did not find debug symbol for current address");
            }
        }

        let stack_frame_pointer: Addr = self.get_reg(Register::rbp)?.into();
        let return_addr: Addr =
            mem_read_word(self.debuggee.as_ref().unwrap().pid, stack_frame_pointer + 8)?.into();
        trace!("rsb: {stack_frame_pointer}");
        trace!("ret_addr: {return_addr}");

        let should_remove_breakpoint = if !self
            .debuggee
            .as_ref()
            .unwrap()
            .breakpoints
            .contains_key(&return_addr)
        {
            self.set_bp(return_addr)?;
            true
        } else {
            false
        };

        self.cont(None)?;

        if should_remove_breakpoint {
            self.del_bp(return_addr)?;
            self.set_reg(Register::rip, self.get_reg(Register::rip)? - 1)?; // we need to go back
                                                                            // else we skip an instruction
        }
        Ok(Feedback::Ok)
    }

    fn dse(&mut self, here: Addr) -> Result<()> {
        trace!("disabling the breakpoint");
        self.debuggee
            .as_mut()
            .unwrap()
            .breakpoints
            .get_mut(&here)
            .unwrap()
            .disable()?;

        trace!("atomic step");
        self.atomic_single_step()?;
        trace!("waiting");
        self.wait_signal()
            .inspect_err(|e| warn!("weird wait_signal error: {e}"))?;
        trace!("enable stepped over bp again");
        self.debuggee
            .as_mut()
            .unwrap()
            .breakpoints
            .get_mut(&here)
            .unwrap()
            .enable()?;
        trace!("dse done");

        Ok(())
    }

    pub fn go_back_step_over_bp(&mut self) -> Result<bool> {
        // This function is hell with the borrow checker.
        // You can only have a single mutable refence OR n immutable references
        // Thus, you cannot simply `let bp = ...` at the start and later use things like
        // `self.atomic_single_step`
        let maybe_bp_addr: Addr = self.get_current_addr()? - 1;
        trace!("Checkinf if {maybe_bp_addr} had a breakpoint");

        if self
            .debuggee
            .as_mut()
            .unwrap()
            .breakpoints
            .get_mut(&maybe_bp_addr)
            .is_some_and(|a| a.is_enabled())
        {
            let here = maybe_bp_addr;
            trace!("set register to {here}");
            self.set_reg(Register::rip, here.into())?;

            self.dse(here)?;
            Ok(true)
        } else {
            trace!("breakpoint is disabled or does not exist, doing nothing");
            Ok(false)
        }
    }

    pub fn disassemble_at(&self, addr: Addr, len: usize) -> Result<Feedback> {
        let dbge = self.debuggee.as_ref().ok_or(DebuggerError::NoDebugee)?;

        let t = dbge.disassemble(addr, len)?;

        Ok(Feedback::Disassembly(t))
    }

    pub fn get_symbol_by_name(&self, name: impl Display) -> Result<Feedback> {
        let dbge = self.debuggee.as_ref().ok_or(DebuggerError::NoDebugee)?;

        let symbols: Vec<OwnedSymbol> = dbge.get_symbol_by_name(name)?;
        Ok(Feedback::Symbols(symbols))
    }

    pub fn handle_sigtrap(
        &self,
        sig: nix::sys::signal::Signal,
        siginfo: nix::libc::siginfo_t,
    ) -> Result<()> {
        info!("debugee received {}: {}", sig.as_str(), siginfo.si_code);

        match siginfo.si_code {
            SI_KERNEL => trace!("SI_KERNEL"), // we don't know what do do?
            TRAP_BRKPT => {
                trace!("TRAP_BRKPT")
            }
            TRAP_TRACE => trace!("TRAP_TRACE"), // single stepping
            _ => warn!("Strange SIGTRAP code: {}", siginfo.si_code),
        }
        Ok(())
    }

    pub fn handle_important_signal(
        &self,
        sig: Signal,
        siginfo: nix::libc::siginfo_t,
    ) -> Result<()> {
        info!("debugee received {}: {}", sig.as_str(), siginfo.si_code);
        Ok(())
    }

    pub fn handle_other_signal(&self, sig: Signal, siginfo: nix::libc::siginfo_t) -> Result<()> {
        info!("debugee received {}: {}", sig.as_str(), siginfo.si_code);
        Ok(())
    }

    fn infos(&self) -> std::result::Result<Feedback, DebuggerError> {
        let dbge = self.debuggee.as_ref().ok_or(DebuggerError::NoDebugee)?;
        info!("Breakpoints:\n{:#?}", dbge.breakpoints);
        Ok(Feedback::Ok)
    }

    pub fn step_into(&mut self) -> Result<Feedback> {
        self.go_back_step_over_bp()?;

        loop {
            let rip: Addr = (self.get_reg(Register::rip)?).into();
            let disassembly: Disassembly = self.debuggee.as_ref().unwrap().disassemble(rip, 8)?;
            let next_instruction = &disassembly.inner()[0];
            let operator = next_instruction.2[0].clone();

            if operator.1 != FormatterTextKind::Mnemonic {
                error!("could not read operator from disassembly");
            }
            if operator.0.trim() == "call" {
                self.single_step()?;
                break;
            } else {
                self.single_step()?; // PERF: this is very inefficient :/ maybe remove the autostepper
            }
        }

        Ok(Feedback::Ok)
    }

    pub fn step_over(&mut self) -> Result<Feedback> {
        self.go_back_step_over_bp()?;

        self.step_into()?;
        self.step_out()
    }

    pub fn backtrace(&self) -> Result<Feedback> {
        let dbge = self.debuggee.as_ref().ok_or(DebuggerError::NoDebugee)?;

        let backtrace = unwind::unwind(dbge.pid)?;

        Ok(Feedback::Backtrace(backtrace))
    }

    pub fn get_current_addr(&self) -> Result<Addr> {
        Ok((self.get_reg(Register::rip)?).into())
    }

    fn prepare_variable_access(
        &self,
        expression: &VariableExpression,
    ) -> Result<(OwnedSymbol, OwnedSymbol, FrameInfo)> {
        let dbge = self.debuggee.as_ref().ok_or(DebuggerError::NoDebugee)?;
        let rip: Addr = self.get_current_addr()?;

        // Get current function
        let current_function = match dbge.get_function_by_addr(rip)? {
            Some(f) if f.frame_base().is_some() => f,
            Some(_) => {
                return Err(DebuggerError::AttributeDoesNotExist(
                    gimli::DW_AT_frame_base,
                ))
            }
            None => return Err(DebuggerError::NotInFunction),
        };

        // Find variable
        let locals = dbge.get_local_variables(rip)?;
        let vars = dbge.filter_expressions(&locals, expression)?;
        let var = match vars.len() {
            0 => {
                return Err(DebuggerError::VarExprReturnedNothing(
                    expression.to_string(),
                ))
            }
            1 => vars[0].clone(),
            _ => return Err(DebuggerError::AmbiguousVarExpr(expression.to_string())),
        };

        // Build frame info
        let mut frame_info = FrameInfo::new(
            None,
            Some(Into::<Addr>::into(self.get_reg(Register::rbp)?) + 16usize),
        );

        let frame_base = dbge.parse_location(
            current_function.frame_base().unwrap(),
            &frame_info,
            current_function.encoding(),
        )?;

        let frame_base: Addr = match frame_base {
            gimli::Location::Address { address } => address.into(),
            other => unimplemented!(
                "frame base DWARF location was not an address as expected: is {other:?}"
            ),
        };

        frame_info.frame_base = Some(frame_base);

        Ok((current_function, var, frame_info))
    }

    pub fn read_variable(&self, expression: VariableExpression) -> Result<Feedback> {
        let dbge = self.debuggee.as_ref().ok_or(DebuggerError::NoDebugee)?;

        let (_, var, frame_info) = self.prepare_variable_access(&expression)?;

        let val = dbge.var_read(&var, &frame_info)?;

        Ok(Feedback::Variable(val))
    }

    pub fn write_variable(
        &self,
        expression: VariableExpression,
        value: impl Into<VariableValue>,
    ) -> Result<Feedback> {
        let dbge = self.debuggee.as_ref().ok_or(DebuggerError::NoDebugee)?;

        let (_, var, frame_info) = self.prepare_variable_access(&expression)?;

        dbge.var_write(&var, &frame_info, value.into())?;

        Ok(Feedback::Ok)
    }

    pub fn read_mem(&self, addr: Addr) -> Result<Feedback> {
        let dbge = self.debuggee.as_ref().ok_or(DebuggerError::NoDebugee)?;

        let w = mem_read_word(dbge.pid, addr)?;

        Ok(Feedback::Word(w))
    }

    pub fn write_mem(&self, addr: Addr, value: Word) -> Result<Feedback> {
        let dbge = self.debuggee.as_ref().ok_or(DebuggerError::NoDebugee)?;

        mem_write_word(dbge.pid, addr, value)?;

        Ok(Feedback::Ok)
    }

    pub fn get_reg(&self, r: Register) -> Result<u64> {
        let dbge = self.debuggee.as_ref().ok_or(DebuggerError::NoDebugee)?;

        crate::get_reg(dbge.pid, r)
    }

    pub fn set_reg(&self, r: Register, v: u64) -> Result<Feedback> {
        let dbge = self.debuggee.as_ref().ok_or(DebuggerError::NoDebugee)?;
        crate::set_reg(dbge.pid, r, v)?;
        Ok(Feedback::Ok)
    }

    pub fn get_stack(&self) -> Result<Feedback> {
        let dbge = self.debuggee.as_ref().ok_or(DebuggerError::NoDebugee)?;

        let stack = dbge.get_stack()?;
        Ok(Feedback::Stack(stack))
    }

    pub fn get_process_map(&self) -> Result<Feedback> {
        let dbge = self.debuggee.as_ref().ok_or(DebuggerError::NoDebugee)?;

        let pm = dbge.get_process_map()?;

        Ok(Feedback::ProcessMap(pm))
    }

    pub fn run(
        &mut self,
        executable_path: impl AsRef<Path>,
        arguments: &[CString],
    ) -> Result<Feedback> {
        if self.debuggee.is_some() {
            return Err(DebuggerError::AlreadyRunning);
        }

        // NOTE: the lifetimes of the raw object data have given us many problems. It would be
        // possible to read the object data out in the main function and passing it to the
        // constructor of Debugger, but that would mean that we cannot debug a different program in
        // the same session.
        let exe: &Path = executable_path.as_ref();

        // First, read the file data
        self.stored_obj_data_raw = std::fs::read(exe)?;

        // Create a new scope to handle the borrow checker
        {
            // Create a reference to the raw data that matches the 'executable lifetime
            let raw_data: &'executable [u8] = unsafe {
                std::mem::transmute::<&[u8], &'executable [u8]>(&self.stored_obj_data_raw)
            };

            // Parse the object file
            let obj_data = object::File::parse(raw_data)?;
            self.stored_obj_data = Some(obj_data);
        }

        // Now launch the debuggee
        self.launch_debuggee(exe, arguments)?;

        Ok(Feedback::Ok)
    }
}
