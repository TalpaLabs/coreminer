use std::fs;
use std::path::Path;
use std::rc::Rc;

use gimli::write::EndianVec;
use gimli::{EndianRcSlice, NativeEndian};
use object::{File, Object, ObjectSection};
use ouroboros::self_referencing;

use crate::errors::{DebuggerError, Result};

// the gimli::Reader we use
type GimliRd = EndianRcSlice<NativeEndian>;

pub struct CMDebugInfo<'executable> {
    object_info: object::File<'executable>,
    linedata: addr2line::Context<GimliRd>,
}

impl<'executable> CMDebugInfo<'executable> {
    pub fn build(object_info: object::File<'executable>) -> Result<Self> {
        // FIXME: this is about the ugliest function ever
        //
        //
        // But it works...
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
        let linedata = addr2line::Context::from_dwarf(dwarf)?;

        Ok(CMDebugInfo {
            object_info,
            linedata,
        })
    }
}
