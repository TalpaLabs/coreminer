use std::fs;
use std::path::Path;
use std::rc::Rc;

use gimli::write::EndianVec;
use gimli::{EndianRcSlice, NativeEndian};
use object::{File, Object, ObjectSection};
use ouroboros::self_referencing;

use crate::errors::Result;

// the gimli::Reader we use
type GimliRd = EndianRcSlice<NativeEndian>;

pub struct CMDebugInfo<'executable> {
    object_info: object::File<'executable>,
    linedata: addr2line::Context<GimliRd>,
}

impl<'executable> CMDebugInfo<'executable> {
    pub fn build(object_info: object::File<'executable>) -> Result<Self> {
        let dwarf = gimli::Dwarf::load(|section| -> std::result::Result<_, ()> {
            let data = object_info
                .section_by_name(section.name())
                .map(|s| s.uncompressed_data().unwrap());

            Ok(GimliRd::new(
                Rc::from(data.unwrap_or_default().as_ref()),
                gimli::NativeEndian,
            ))
        })
        .unwrap();

        let linedata = addr2line::Context::from_dwarf(dwarf)?;

        Ok(CMDebugInfo {
            object_info,
            linedata,
        })
    }
}
