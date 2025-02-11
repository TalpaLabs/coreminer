use std::fmt::Display;
use std::io::{Read, Seek, Write};
use std::ops::{Add, Sub};
use std::str::FromStr;

use nix::sys::ptrace;
use nix::unistd::Pid;

use crate::errors::Result;

use self::dbginfo::GimliLocation;
use self::errors::DebuggerError;

pub mod breakpoint;
pub mod consts;
pub mod dbginfo;
pub mod debugger;
pub mod disassemble;
pub mod dwarf_parse;
pub mod errors;
pub mod feedback;
pub mod ui;
pub mod unwind;
pub mod variable;

pub type Word = i64;
pub type RawPointer = *mut std::ffi::c_void;

#[derive(Hash, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Addr(pub RawPointer);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum Register {
    r15,
    r14,
    r13,
    r12,
    rbp,
    rbx,
    r11,
    r10,
    r9,
    r8,
    rax,
    rcx,
    rdx,
    rsi,
    rdi,
    orig_rax,
    rip,
    cs,
    eflags,
    rsp,
    ss,
    fs_base,
    gs_base,
    ds,
    es,
    fs,
    gs,
}

impl FromStr for Register {
    type Err = DebuggerError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let s = s.to_lowercase();
        Ok(match s.as_str() {
            "r15" => Self::r15,
            "r14" => Self::r14,
            "r13" => Self::r13,
            "r12" => Self::r12,
            "rbp" => Self::rbp,
            "rbx" => Self::rbx,
            "r11" => Self::r11,
            "r10" => Self::r10,
            "r9" => Self::r9,
            "r8" => Self::r8,
            "rax" => Self::rax,
            "rcx" => Self::rcx,
            "rdx" => Self::rdx,
            "rsi" => Self::rsi,
            "rdi" => Self::rdi,
            "orig_rax" => Self::orig_rax,
            "rip" => Self::rip,
            "cs" => Self::cs,
            "eflags" => Self::eflags,
            "rsp" => Self::rsp,
            "ss" => Self::ss,
            "fs_base" => Self::fs_base,
            "gs_base" => Self::gs_base,
            "ds" => Self::ds,
            "es" => Self::es,
            "fs" => Self::fs,
            "gs" => Self::gs,
            _ => return Err(DebuggerError::ParseStr(s)),
        })
    }
}

impl TryFrom<gimli::Register> for Register {
    type Error = DebuggerError;
    /// Gimli has the indexes of the registers, but we have it as an enumerator.
    /// The DWARF Register Number Mapping is defined in the amd64 ABI here:
    /// <https://refspecs.linuxbase.org/elf/x86_64-abi-0.99.pdf#figure.3.36>
    fn try_from(value: gimli::Register) -> Result<Self> {
        match value.0 {
            0 => Ok(Register::rax),
            1 => Ok(Register::rdx),
            2 => Ok(Register::rcx),
            3 => Ok(Register::rbx),
            4 => Ok(Register::rsi),
            5 => Ok(Register::rdi),
            6 => Ok(Register::rbp),
            7 => Ok(Register::rsp),
            8 => Ok(Register::r8),
            9 => Ok(Register::r9),
            10 => Ok(Register::r10),
            11 => Ok(Register::r11),
            12 => Ok(Register::r12),
            13 => Ok(Register::r13),
            14 => Ok(Register::r14),
            15 => Ok(Register::r15),
            16 => Ok(Register::rip),

            49 => Ok(Register::eflags),

            50 => Ok(Register::es),
            51 => Ok(Register::cs),
            52 => Ok(Register::ss),
            53 => Ok(Register::ds),
            54 => Ok(Register::fs),
            55 => Ok(Register::gs),

            56 => Ok(Register::fs_base),
            57 => Ok(Register::gs_base),

            // We skip 58..=62 because they correspond to tr, ldtr, mxcsr, fcw, fsw, etc.
            // which aren't in our enum.

            // No standard mapping for 63 or `orig_rax`
            // So we return None for those or anything else unrecognized.
            x => Err(DebuggerError::UnimplementedRegister(x)),
        }
    }
}

impl Addr {
    pub fn from_relative(base: Addr, raw: usize) -> Addr {
        Self::from(base.usize() + raw)
    }

    pub fn relative(&self, base: Addr) -> Addr {
        *self - base
    }

    pub fn usize(&self) -> usize {
        self.0 as usize
    }
    pub fn u64(&self) -> u64 {
        self.0 as u64
    }
    pub fn raw_pointer(&self) -> RawPointer {
        self.0
    }
}

impl Display for Addr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#018x}", self.0 as usize)
    }
}

impl Add for Addr {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self((self.0 as usize + rhs.0 as usize) as RawPointer)
    }
}

impl Add<usize> for Addr {
    type Output = Self;
    fn add(self, rhs: usize) -> Self::Output {
        Self((self.0 as usize + rhs) as RawPointer)
    }
}

impl Sub for Addr {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self((self.0 as usize - rhs.0 as usize) as RawPointer)
    }
}

impl Sub<usize> for Addr {
    type Output = Self;
    fn sub(self, rhs: usize) -> Self::Output {
        Self((self.0 as usize - rhs) as RawPointer)
    }
}

impl From<RawPointer> for Addr {
    fn from(value: RawPointer) -> Self {
        Addr(value)
    }
}

impl From<Addr> for RawPointer {
    fn from(value: Addr) -> Self {
        value.0
    }
}

impl From<usize> for Addr {
    fn from(value: usize) -> Self {
        Addr(value as RawPointer)
    }
}

impl From<Word> for Addr {
    fn from(value: Word) -> Self {
        Addr(value as RawPointer)
    }
}

impl From<u64> for Addr {
    fn from(value: u64) -> Self {
        Addr(value as RawPointer)
    }
}

impl From<Addr> for Word {
    fn from(value: Addr) -> Self {
        value.0 as Word
    }
}

impl From<Addr> for u64 {
    fn from(value: Addr) -> Self {
        value.0 as u64
    }
}

pub(crate) fn mem_write_word(pid: Pid, addr: Addr, value: Word) -> Result<()> {
    Ok(ptrace::write(pid, addr.into(), value)?)
}

pub(crate) fn mem_read_word(pid: Pid, addr: Addr) -> Result<Word> {
    Ok(ptrace::read(pid, addr.into())?)
}

pub(crate) fn mem_read(data_raw: &mut [u8], pid: Pid, addr: Addr) -> Result<usize> {
    let mut file = std::fs::File::options()
        .read(true)
        .write(false)
        .open(format!("/proc/{pid}/mem"))?;
    file.seek(std::io::SeekFrom::Start(addr.into()))?;
    let len = file.read(data_raw)?;

    Ok(len)
}

pub(crate) fn mem_write(data_raw: &[u8], pid: Pid, addr: Addr) -> Result<usize> {
    let mut file = std::fs::File::options()
        .read(false)
        .write(true)
        .open(format!("/proc/{pid}/mem"))?;
    file.seek(std::io::SeekFrom::Start(addr.into()))?;
    let len = file.write(data_raw)?;

    Ok(len)
}

pub fn get_reg(pid: Pid, r: Register) -> Result<u64> {
    let regs = ptrace::getregs(pid)?;

    let v = match r {
        Register::r9 => regs.r9,
        Register::r8 => regs.r8,
        Register::r10 => regs.r10,
        Register::r11 => regs.r11,
        Register::r12 => regs.r12,
        Register::r13 => regs.r13,
        Register::r14 => regs.r14,
        Register::r15 => regs.r15,
        Register::rip => regs.rip,
        Register::rbp => regs.rbp,
        Register::rax => regs.rax,
        Register::rcx => regs.rcx,
        Register::rbx => regs.rbx,
        Register::rdx => regs.rdx,
        Register::rsi => regs.rsi,
        Register::rsp => regs.rsp,
        Register::rdi => regs.rdi,
        Register::orig_rax => regs.orig_rax,
        Register::eflags => regs.eflags,
        Register::es => regs.es,
        Register::cs => regs.cs,
        Register::ss => regs.ss,
        Register::fs_base => regs.fs_base,
        Register::fs => regs.fs,
        Register::gs_base => regs.gs_base,
        Register::gs => regs.gs,
        Register::ds => regs.ds,
    };

    Ok(v)
}

pub fn set_reg(pid: Pid, r: Register, v: u64) -> Result<()> {
    let mut regs = ptrace::getregs(pid)?;

    match r {
        Register::r9 => regs.r9 = v,
        Register::r8 => regs.r8 = v,
        Register::r10 => regs.r10 = v,
        Register::r11 => regs.r11 = v,
        Register::r12 => regs.r12 = v,
        Register::r13 => regs.r13 = v,
        Register::r14 => regs.r14 = v,
        Register::r15 => regs.r15 = v,
        Register::rip => regs.rip = v,
        Register::rbp => regs.rbp = v,
        Register::rax => regs.rax = v,
        Register::rcx => regs.rcx = v,
        Register::rbx => regs.rbx = v,
        Register::rdx => regs.rdx = v,
        Register::rsi => regs.rsi = v,
        Register::rsp => regs.rsp = v,
        Register::rdi => regs.rdi = v,
        Register::orig_rax => regs.orig_rax = v,
        Register::eflags => regs.eflags = v,
        Register::es => regs.es = v,
        Register::cs => regs.cs = v,
        Register::ss => regs.ss = v,
        Register::fs_base => regs.fs_base = v,
        Register::fs => regs.fs = v,
        Register::gs_base => regs.gs_base = v,
        Register::gs => regs.gs = v,
        Register::ds => regs.ds = v,
    }

    ptrace::setregs(pid, regs)?;

    Ok(())
}

pub fn gimli_location_to_addr(_pid: Pid, loc: &GimliLocation) -> Result<Addr> {
    match loc {
        gimli::Location::Address { address } => Ok((*address).into()),
        other => unimplemented!("reading a location with {other:?} is not implemented"),
    }
}

#[cfg(test)]
mod test {
    use super::Register;
    #[test]
    fn test_dwarf_number_to_register() {
        assert_eq!(
            Register::try_from(gimli::Register(6)).expect("could not make register from valid num"),
            Register::rbp
        );
        assert_eq!(
            Register::try_from(gimli::Register(15))
                .expect("could not make register from valid num"),
            Register::r15
        );
        Register::try_from(gimli::Register(666)).expect_err("could make register from invalid num");
    }
}
