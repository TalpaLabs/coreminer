use tracing::{info, trace};

use crate::dbginfo::{search_through_symbols, OwnedSymbol, SymbolKind};
use crate::debuggee::Debuggee;
use crate::dwarf_parse::FrameInfo;
use crate::errors::{DebuggerError, Result};
use crate::{get_reg, mem_read, mem_write, set_reg, Addr, Word, WORD_BYTES};

pub type VariableExpression = String;

#[derive(Debug, Clone)]
pub enum VariableValue {
    Bytes(Vec<u8>),
    Other(Word),
    Numeric(gimli::Value),
}

impl VariableValue {
    fn byte_size(&self) -> usize {
        match self {
            Self::Bytes(b) => b.len(),
            Self::Other(_w) => WORD_BYTES,
            Self::Numeric(v) => match v.value_type() {
                gimli::ValueType::U8 | gimli::ValueType::I8 => 1,
                gimli::ValueType::U16 | gimli::ValueType::I16 => 2,
                gimli::ValueType::U32 | gimli::ValueType::I32 | gimli::ValueType::F32 => 4,
                gimli::ValueType::U64
                | gimli::ValueType::I64
                | gimli::ValueType::F64
                | gimli::ValueType::Generic => 8,
            },
        }
    }

    fn to_u64(&self) -> u64 {
        match self {
            Self::Bytes(b) => {
                if b.len() > WORD_BYTES {
                    panic!("too many bytes to put into a word")
                }
                // NOTE: this is safe because `b` should never have more bytes than a u64
                crate::bytes_to_u64(b).unwrap()
            }
            Self::Other(w) => crate::bytes_to_u64(&w.to_ne_bytes()).unwrap(),
            Self::Numeric(v) => match v {
                gimli::Value::U8(v) => (*v).into(),
                gimli::Value::I8(v) => crate::bytes_to_u64(&v.to_ne_bytes()).unwrap(),
                gimli::Value::U16(v) => (*v).into(),
                gimli::Value::I16(v) => crate::bytes_to_u64(&v.to_ne_bytes()).unwrap(),
                gimli::Value::U32(v) => (*v).into(),
                gimli::Value::I32(v) => crate::bytes_to_u64(&v.to_ne_bytes()).unwrap(),
                gimli::Value::F32(v) => crate::bytes_to_u64(&v.to_ne_bytes()).unwrap(),
                gimli::Value::U64(v) => *v,
                gimli::Value::I64(v) => crate::bytes_to_u64(&v.to_ne_bytes()).unwrap(),
                gimli::Value::F64(v) => crate::bytes_to_u64(&v.to_ne_bytes()).unwrap(),
                gimli::Value::Generic(v) => *v,
            },
        }
    }

    fn resize_to_bytes(&self, byte_size: usize) -> Vec<u8> {
        if byte_size > WORD_BYTES {
            panic!("requested byte size was larger than a word")
        }

        let mut data = self.to_u64().to_ne_bytes().to_vec();
        while data.len() < byte_size {
            data.push(0);
        }
        data
    }
}

impl From<usize> for VariableValue {
    fn from(value: usize) -> Self {
        VariableValue::Numeric(gimli::Value::Generic(value as u64))
    }
}

impl Debuggee {
    pub fn filter_expressions(
        &self,
        haystack: &[OwnedSymbol],
        expression: &VariableExpression,
    ) -> Result<Vec<OwnedSymbol>> {
        Ok(search_through_symbols(haystack, |s| {
            s.name() == Some(expression)
        }))
    }

    pub fn var_write(
        &self,
        sym: &OwnedSymbol,
        frame_info: &FrameInfo,
        value: VariableValue,
    ) -> Result<()> {
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

        let datatype = match self.get_type_for_symbol(sym)? {
            Some(d) => d,
            None => return Err(DebuggerError::NoDatatypeFound),
        };

        if datatype.byte_size().is_none() {
            panic!("datatype found but it had no byte_size?")
        }

        let loc_attr = sym.location().unwrap();
        let location = self.parse_location(loc_attr, frame_info, sym.encoding())?;
        trace!("doing location match for writing variable");

        match location {
            gimli::Location::Address { address } => {
                let value_raw = value.resize_to_bytes(datatype.byte_size().unwrap());
                let addr: Addr = address.into();
                trace!("writing to {addr}");
                let written = mem_write(&value_raw, self.pid, addr)?;
                assert_eq!(written, value.byte_size());
            }
            gimli::Location::Register { register } => {
                trace!("setting register");
                set_reg(self.pid, register.try_into()?, value.to_u64())?
            }
            other => unimplemented!(
                "writing to variable with gimli location of type {other:?} is not implemented"
            ),
        }
        trace!("done writing the variable");

        Ok(())
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
