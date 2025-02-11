use tracing::debug;

use crate::dbginfo::OwnedSymbol;
use crate::debugger::Debuggee;
use crate::errors::Result;
use crate::Word;

pub type VariableExpression = String;

#[derive(Debug, Clone)]
pub enum VariableValue {
    Other(Word),
}

impl Debuggee<'_> {
    pub fn filter_expressions(
        &self,
        haystack: &[OwnedSymbol],
        expression: VariableExpression,
    ) -> Result<Vec<OwnedSymbol>> {
        Ok(haystack
            .iter()
            .filter(|a| a.name() == Some(&expression))
            .cloned()
            .collect())
    }

    pub fn var_read(&self, sym: &OwnedSymbol) -> Result<VariableValue> {
        if sym.location.is_none() {
            panic!("boom")
        }
        let loc = sym.location.as_ref().unwrap();
        debug!("loc of that thing: {loc:?}");

        match loc {
            gimli::Location::Address { address } => Ok(VariableValue::Other(crate::mem_read_word(
                self.pid,
                (*address).into(),
            )?)),
            other => unimplemented!("reading a location with {other:?} is not implemented"),
        }
    }
}
