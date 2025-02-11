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
    todo!()
}

pub fn var_read(sym: &OwnedSymbol) -> Result<VariableValue> {
    todo!()
}
