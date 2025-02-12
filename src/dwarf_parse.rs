use core::panic;

use gimli::write::LocationListOffsets;
use gimli::{Expression, Reader, Unit};
use nix::unistd::Pid;
use tracing::{trace, warn};

use crate::dbginfo::{GimliLocation, OwnedSymbol};
use crate::debugger::Debuggee;
use crate::errors::{DebuggerError, Result};
use crate::{get_reg, mem_read, Addr};

pub(crate) type GimliReaderThing = gimli::EndianReader<gimli::LittleEndian, std::rc::Rc<[u8]>>;

pub struct FrameInfos {
    pub frame_base: Option<Addr>,
    pub canonical_frame_address: Option<Addr>,
}
impl FrameInfos {
    pub(crate) fn empty() -> FrameInfos {
        FrameInfos {
            frame_base: None,
            canonical_frame_address: None,
        }
    }
}

impl Debuggee<'_> {
    pub(crate) fn parse_addr_low(
        dwarf: &gimli::Dwarf<GimliReaderThing>,
        unit: &Unit<GimliReaderThing>,
        attribute: Option<gimli::Attribute<GimliReaderThing>>,
        base_addr: Addr,
    ) -> Result<Option<Addr>> {
        Ok(if let Some(a) = attribute {
            let a: u64 = match dwarf.attr_address(unit, a.value())? {
                None => {
                    warn!("could not parse addr: {a:?}");
                    return Ok(None);
                }
                Some(a) => a,
            };
            Some(Addr::from_relative(base_addr, a as usize))
        } else {
            None
        })
    }

    pub(crate) fn parse_addr_high(
        attribute: Option<gimli::Attribute<GimliReaderThing>>,
        low: Option<Addr>,
    ) -> Result<Option<Addr>> {
        Ok(if let Some(a) = attribute {
            let addr: Addr = match a.value().udata_value() {
                None => {
                    warn!("could not parse addr: {a:?}");
                    return Ok(None);
                }
                Some(a) => {
                    if let Some(l) = low {
                        l + a as usize
                    } else {
                        return Err(DebuggerError::HighAddrExistsButNotLowAddr);
                    }
                }
            };
            Some(addr)
        } else {
            None
        })
    }

    pub(crate) fn parse_string(
        dwarf: &gimli::Dwarf<GimliReaderThing>,
        unit: &Unit<GimliReaderThing>,
        attribute: Option<gimli::Attribute<GimliReaderThing>>,
    ) -> Result<Option<String>> {
        Ok(if let Some(a) = attribute {
            Some(
                dwarf
                    .attr_string(unit, a.value())?
                    .to_string_lossy()?
                    .to_string(),
            )
        } else {
            None
        })
    }

    pub(crate) fn parse_datatype(
        attribute: Option<gimli::Attribute<GimliReaderThing>>,
    ) -> Result<Option<usize>> {
        Ok(if let Some(a) = attribute {
            if let gimli::AttributeValue::UnitRef(thing) = a.value() {
                Some(thing.0)
            } else {
                warn!("idk");
                None
            }
        } else {
            None
        })
    }

    pub(crate) fn parse_location(
        pid: Pid,
        unit: &Unit<GimliReaderThing>,
        attribute: Option<gimli::Attribute<GimliReaderThing>>,
        frame_infos: &mut FrameInfos,
    ) -> Result<Option<GimliLocation>> {
        let attribute = match attribute {
            None => return Ok(None),
            Some(a) => a,
        };

        match attribute.value() {
            gimli::AttributeValue::Exprloc(expr) => {
                Self::eval_expression(pid, unit, expr, frame_infos)
            }
            // gimli::AttributeValue::LocationListsRef(loclist_offs) => {
            //     Self::parse_loclist(loclist_offs)
            // }
            _ => panic!("we did not know a location could be this"),
        }
    }

    pub(crate) fn parse_loclist(
        loclist_offset: LocationListOffsets,
    ) -> Result<Option<GimliLocation>> {
        todo!()
    }

    pub(crate) fn eval_expression(
        pid: Pid,
        unit: &Unit<GimliReaderThing>,
        expression: Expression<GimliReaderThing>,
        frame_infos: &mut FrameInfos,
    ) -> Result<Option<GimliLocation>> {
        let mut evaluation = expression.evaluation(unit.encoding());
        let mut res = evaluation.evaluate()?;
        loop {
            match res {
                gimli::EvaluationResult::Complete => {
                    break;
                }
                gimli::EvaluationResult::RequiresMemory {
                    address,
                    size,
                    .. // there is more but that is getting to complicated, just give gimli 
                    // unsized values of the right size
                } => {
                    let mut buff = vec![0; size as usize];
                    let addr: Addr = address.into(); // NOTE: may be relative?
                    let read_this_many_bytes = mem_read(&mut buff, pid, addr)?;
                    assert_eq!(size as usize, read_this_many_bytes);
                    let value = to_value(size, &buff);
                    res = evaluation.resume_with_memory(value)?;
                }
                gimli::EvaluationResult::RequiresRegister { register, .. /* ignore the actual type and give as word */ } => {
                    let reg= crate::Register::try_from(register)?;
                    let reg_value = crate::get_reg(pid, reg)?;
                    res = evaluation.resume_with_register(gimli::Value::from_u64(gimli::ValueType::Generic, reg_value)?)?;
                }
                gimli::EvaluationResult::RequiresFrameBase =>{
                    let frame_base: Addr = frame_infos.frame_base.expect("frame base was None");

                    res = evaluation.resume_with_frame_base(
                        frame_base.u64()
                    )?;
                }
                gimli::EvaluationResult::RequiresCallFrameCfa => {
                    let cfa = get_reg(pid, crate::Register::rbp)?;
                    res = evaluation.resume_with_call_frame_cfa(cfa)?;
                }
                other => {
                    unimplemented!("Gimli expression parsing for {other:?} is not implemented")
                }
            }
        }
        let pieces = evaluation.result();

        if pieces.is_empty() {
            warn!("really? we did all that parsing and got NOTHING");
            Ok(None)
        } else {
            let loc = pieces[0].location.clone();
            trace!("location for the expression: {loc:?}");
            Ok(Some(loc))
        }
    }
}

fn to_value(size: u8, buff: &[u8]) -> gimli::Value {
    match size {
        1 => gimli::Value::U8(buff[0]),
        2 => gimli::Value::U16(u16::from_be_bytes([buff[0], buff[1]])),
        4 => gimli::Value::U32(u32::from_be_bytes([buff[0], buff[1], buff[2], buff[3]])),
        x => unimplemented!("Requested memory with size {x}, which is not supported yet."),
    }
}
