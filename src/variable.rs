use tracing::info;

use crate::dbginfo::{search_through_symbols, OwnedSymbol, SymbolKind};
use crate::debuggee::Debuggee;
use crate::dwarf_parse::FrameInfo;
use crate::errors::{DebuggerError, Result};
use crate::{get_reg, mem_read, Addr, Word};

pub type VariableExpression = String;

#[derive(Debug, Clone)]
pub enum VariableValue {
    Bytes(Vec<u8>),
    Other(Word),
    Numeric(gimli::Value),
}

impl Debuggee<'_> {
    pub fn filter_expressions(
        &self,
        haystack: &[OwnedSymbol],
        expression: &VariableExpression,
    ) -> Result<Vec<OwnedSymbol>> {
        Ok(search_through_symbols(haystack, |s| {
            s.name() == Some(expression)
        }))
    }

    pub fn var_read(&self, sym: &OwnedSymbol, frame_info: &FrameInfo) -> Result<VariableValue> {
        match sym.kind() {
            SymbolKind::Variable | SymbolKind::Parameter => (),
            _ => return Err(DebuggerError::WrongSymbolKind(sym.kind())),
        }
        if sym.datatype().is_none() {
            return Err(DebuggerError::VariableSymbolNoType);
        }
        if sym.location().is_none() {
            return Err(DebuggerError::SymbolHasNoLocation);
        }
        let loc_attr = sym.location().unwrap();
        let datatype = match self.get_type_for_symbol(sym)? {
            Some(d) => d,
            None => return Err(DebuggerError::NoDatatypeFound),
        };
        let location = self.parse_location(loc_attr, frame_info, sym.encoding())?;

        let value = match location {
            gimli::Location::Value { value } => value.into(),
            gimli::Location::Bytes { value } => VariableValue::Bytes(value.to_vec()),
            gimli::Location::Address { address } => {
                let addr: Addr = address.into();
                info!("reading var from {addr}");
                let size = datatype.byte_size().expect("datatype had no byte_size");
                let mut buf = vec![0; size];
                let len = mem_read(&mut buf, self.pid, addr)?;
                assert_eq!(len, size);

                VariableValue::Bytes(buf)
            }
            gimli::Location::Register { register } => {
                VariableValue::Other(get_reg(self.pid, register.try_into()?)? as i64)
            }
            gimli::Location::Empty => todo!(),
            other => unimplemented!("gimli location of type {other:?} is not implemented"),
        };

        Ok(value)
    }
}

impl From<gimli::Value> for VariableValue {
    fn from(value: gimli::Value) -> Self {
        Self::Numeric(value)
    }
}
