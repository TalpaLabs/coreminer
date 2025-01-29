use std::rc::Rc;

use addr2line::fallible_iterator::FallibleIterator;
use clap::value_parser;
use gimli::{EndianRcSlice, NativeEndian};
use object::{Object, ObjectSection, ObjectSymbol};

use crate::errors::{DebuggerError, Result};
use crate::Addr;

// the gimli::Reader we use
type GimliRd = EndianRcSlice<NativeEndian>;

pub struct CMDebugInfo<'executable> {
    pub object_info: object::File<'executable>,
    pub linedata: addr2line::Context<GimliRd>,
}

#[derive(Debug)]
pub struct OwnedSymbol {
    pub name: String,
    pub addr: Addr,
}

impl<'executable> CMDebugInfo<'executable> {
    pub fn build(object_info: object::File<'executable>) -> Result<Self> {
        let loader = |section: gimli::SectionId| -> std::result::Result<_, ()> {
            // does never fail surely
            let data = object_info
                .section_by_name(section.name())
                .map(|s| s.uncompressed_data().unwrap_or_default());

            Ok(GimliRd::new(
                Rc::from(data.unwrap_or_default().as_ref()),
                gimli::NativeEndian,
            ))
        };
        let dwarf = gimli::Dwarf::load(loader).unwrap();
        // TODO: somehow get the variables from gimli

        let linedata = addr2line::Context::from_dwarf(dwarf)?;

        Ok(CMDebugInfo {
            object_info,
            linedata,
        })
    }
}

impl TryFrom<object::Symbol<'_, '_>> for OwnedSymbol {
    fn try_from(value: object::Symbol<'_, '_>) -> std::result::Result<Self, Self::Error> {
        Ok(OwnedSymbol {
            name: value.name()?.to_string(),
            addr: value.address().into(),
        })
    }

    type Error = DebuggerError;
}
