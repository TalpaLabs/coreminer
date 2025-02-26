use std::collections::HashMap;
use std::fmt::Display;

use gimli::{
    Attribute, DW_AT_frame_base, DW_AT_high_pc, DW_AT_location, DW_AT_low_pc, DW_AT_name,
    DW_AT_type, Unit,
};
use nix::sys::ptrace;
use nix::unistd::Pid;
use proc_maps::MapRange;
use tracing::{debug, warn};

use crate::breakpoint::{Breakpoint, INT3_BYTE};
use crate::dbginfo::{search_through_symbols, CMDebugInfo, OwnedSymbol, SymbolKind};
use crate::disassemble::Disassembly;
use crate::dwarf_parse::GimliReaderThing;
use crate::stack::Stack;
use crate::{get_reg, mem_read_word, Result};
use crate::{mem_read, Addr};

pub struct Debuggee {
    pub(crate) pid: Pid,
    pub(crate) breakpoints: HashMap<Addr, Breakpoint>,
    pub(crate) symbols: Vec<OwnedSymbol>,
}

impl Debuggee {
    pub(crate) fn build(
        pid: Pid,
        dbginfo: CMDebugInfo<'_>,
        breakpoints: HashMap<Addr, Breakpoint>,
    ) -> Result<Self> {
        let mut symbols = Vec::new();
        let dwarf = &dbginfo.dwarf;
        let mut iter = dwarf.units();

        while let Some(header) = iter.next()? {
            let unit = dwarf.unit(header)?;
            let mut tree = unit.entries_tree(None)?;
            symbols.push(Self::process_tree(pid, dwarf, &unit, tree.root()?)?);
        }

        Ok(Self {
            pid,
            breakpoints,
            symbols,
        })
    }

    pub fn kill(&self) -> Result<()> {
        ptrace::kill(self.pid)?;
        Ok(())
    }

    fn get_process_map_by_pid(pid: Pid) -> Result<Vec<MapRange>> {
        Ok(proc_maps::get_process_maps(pid.into())?)
    }

    pub fn get_base_addr_by_pid(pid: Pid) -> Result<Addr> {
        Ok(Self::get_process_map_by_pid(pid)?[0].start().into())
    }

    #[inline]
    pub fn get_process_map(&self) -> Result<Vec<MapRange>> {
        Self::get_process_map_by_pid(self.pid)
    }

    pub fn get_base_addr(&self) -> Result<Addr> {
        Self::get_base_addr_by_pid(self.pid)
    }

    pub fn disassemble(&self, addr: Addr, len: usize, literal: bool) -> Result<Disassembly> {
        let mut data_raw: Vec<u8> = vec![0; len];
        mem_read(&mut data_raw, self.pid, addr)?;

        let mut bp_indexes = Vec::new();

        for (idx, byte) in data_raw.iter_mut().enumerate() {
            if *byte == INT3_BYTE {
                let bp = match self.breakpoints.get(&(addr + idx)) {
                    None => {
                        warn!(
                            "found an int3 without breakpoint at {}, ignoring",
                            addr + idx
                        );
                        continue;
                    }
                    Some(b) => b,
                };
                bp_indexes.push(idx);

                if !literal {
                    *byte = bp.saved_data().expect(
                        "breakpoint exists for a part of code that is an in3, but is disabled",
                    );
                }
            }
        }

        let out: Disassembly = Disassembly::disassemble(&data_raw, addr, &bp_indexes)?;

        for idx in bp_indexes {
            if !self.breakpoints.contains_key(&(addr + idx)) {
                panic!("a stored index that we thought had a breakpoint did not actually have a breakpoint")
            }
            if !literal {
                data_raw[idx] = INT3_BYTE;
            }
        }

        Ok(out)
    }

    fn entry_from_gimli(
        pid: Pid,
        dwarf: &gimli::Dwarf<GimliReaderThing>,
        unit: &Unit<GimliReaderThing>,
        entry: &gimli::DebuggingInformationEntry<'_, '_, GimliReaderThing>,
    ) -> Result<OwnedSymbol> {
        let base_addr = Self::get_base_addr_by_pid(pid)?;

        let name = Self::parse_string(dwarf, unit, entry.attr(DW_AT_name)?)?;
        let kind = SymbolKind::try_from(entry.tag())?;
        let low = Self::parse_addr_low(dwarf, unit, entry.attr(DW_AT_low_pc)?, base_addr)?;
        let high = Self::parse_addr_high(entry.attr(DW_AT_high_pc)?, low)?;
        let datatype: Option<usize> = Self::parse_datatype(entry.attr(DW_AT_type)?)?;
        let location: Option<Attribute<GimliReaderThing>> = entry.attr(DW_AT_location)?;
        let frame_base: Option<Attribute<GimliReaderThing>> = entry.attr(DW_AT_frame_base)?;

        let mut sym = OwnedSymbol::new(entry.offset().0, kind, &[], unit.encoding());
        sym.set_name(name);
        sym.set_location(location);
        sym.set_datatype(datatype);
        sym.set_low_addr(low);
        sym.set_high_addr(high);
        sym.set_frame_base(frame_base);
        Ok(sym)
    }

    // RETURNS ALL SYMBOLS!
    //
    // those symbols have references to their children
    fn process_tree(
        pid: Pid,
        dwarf: &gimli::Dwarf<GimliReaderThing>,
        unit: &Unit<GimliReaderThing>,
        node: gimli::EntriesTreeNode<GimliReaderThing>,
    ) -> Result<OwnedSymbol> {
        let mut children: Vec<OwnedSymbol> = Vec::new();
        let mut parent = Self::entry_from_gimli(pid, dwarf, unit, node.entry())?;

        // then process it's children
        let mut children_tree = node.children();
        while let Some(child) = children_tree.next()? {
            // Recursively process a child.
            children.push(match Self::process_tree(pid, dwarf, unit, child) {
                Err(e) => {
                    debug!("could not parse a leaf of the debug symbol tree: {e}");
                    continue;
                }
                Ok(s) => s,
            });
        }

        parent.set_children(children);
        Ok(parent)
    }

    pub fn get_symbol_by_name(&self, name: impl Display) -> Result<Vec<OwnedSymbol>> {
        let all: Vec<OwnedSymbol> = self
            .symbols_query(|a| a.name() == Some(&name.to_string()))
            .to_vec();

        Ok(all)
    }

    pub fn get_function_by_addr(&self, addr: Addr) -> Result<Option<OwnedSymbol>> {
        debug!("get function for addr {addr}");
        for sym in self
            .symbols_query(|a| a.kind() == SymbolKind::Function)
            .iter()
            .cloned()
        {
            if sym.low_addr().is_some_and(|a| a <= addr)
                && sym.high_addr().is_some_and(|a| addr < a)
            {
                return Ok(Some(sym));
            }
        }

        Ok(None)
    }

    pub fn get_local_variables(&self, addr: Addr) -> Result<Vec<OwnedSymbol>> {
        debug!("get locals of function {addr}");
        for sym in self.symbols_query(|a| a.kind() == SymbolKind::Function) {
            if sym.low_addr().is_some_and(|a| a <= addr)
                && sym.high_addr().is_some_and(|a| addr < a)
            {
                return Ok(sym.children().to_vec());
            }
        }

        Ok(Vec::new())
    }

    pub fn get_symbol_by_offset(&self, offset: usize) -> Result<Option<OwnedSymbol>> {
        // BUG: this might return multiple items if we're dealing with multiple
        // compilation units

        let v: Vec<OwnedSymbol> = self
            .symbols_query(|s| s.offset() == offset)
            .into_iter()
            .collect();
        if v.len() > 1 {
            panic!("multiple or no items for that offset")
        }
        if v.is_empty() {
            Ok(None)
        } else {
            Ok(Some(v[0].clone()))
        }
    }

    #[inline]
    pub fn get_type_for_symbol(&self, sym: &OwnedSymbol) -> Result<Option<OwnedSymbol>> {
        if let Some(dt) = sym.datatype() {
            self.get_symbol_by_offset(dt)
        } else {
            Ok(None)
        }
    }

    pub fn symbols(&self) -> &[OwnedSymbol] {
        &self.symbols
    }

    /// like [Self::symbols] but includes all children somehow
    pub fn symbols_query<F>(&self, fil: F) -> Vec<OwnedSymbol>
    where
        F: Fn(&OwnedSymbol) -> bool,
    {
        search_through_symbols(self.symbols(), fil)
    }

    pub fn get_stack(&self) -> Result<Stack> {
        let rbp: Addr = get_reg(self.pid, crate::Register::rbp)?.into();
        let rsp: Addr = get_reg(self.pid, crate::Register::rsp)?.into();

        let mut next: Addr = rbp;
        let mut stack = Stack::new(rbp);
        while next >= rsp {
            stack.push(mem_read_word(self.pid, next)?);
            next -= 8usize;
        }

        Ok(stack)
    }
}
