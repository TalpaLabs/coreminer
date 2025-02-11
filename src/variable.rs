use tracing::debug;

use crate::dbginfo::OwnedSymbol;
use crate::errors::Result;
use crate::Word;

pub type VariableExpression = String;

#[derive(Debug, Clone)]
pub enum VariableValue {
    Other(Word),
}

pub fn filter_expressions(
    haystack: &[OwnedSymbol],
    expression: VariableExpression,
) -> Result<Vec<OwnedSymbol>> {
    Ok(haystack
        .iter()
        .filter(|a| a.name() == Some(&expression))
        .cloned()
        .collect())
}

pub fn var_read(sym: &OwnedSymbol) -> Result<VariableValue> {
    if sym.location.is_none() {
        panic!("boom")
    }
    let loc = sym.location.as_ref().unwrap();
    debug!("loc of that thing: {loc:?}");
    todo!()
}
