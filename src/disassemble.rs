use std::fmt::Write;

use crate::errors::Result;

use iced_x86::{Decoder, DecoderOptions, Formatter, Instruction, NasmFormatter};

pub fn disassemble(data: &[u8]) -> Result<String> {
    let mut decoder = Decoder::with_ip(
        EXAMPLE_CODE_BITNESS,
        data,
        EXAMPLE_CODE_RIP,
        DecoderOptions::NONE,
    );

    let mut formatter = NasmFormatter::new();
    formatter.options_mut().set_digit_separator("`");
    formatter.options_mut().set_first_operand_char_index(10);

    let mut output = String::new();
    let mut instruction = Instruction::default();

    while decoder.can_decode() {
        // There's also a decode() method that returns an instruction but that also
        // means it copies an instruction (40 bytes):
        //     instruction = decoder.decode();
        decoder.decode_out(&mut instruction);

        // Format the instruction ("disassemble" it)
        formatter.format(&instruction, &mut output);

        // Eg. "00007FFAC46ACDB2 488DAC2400FFFFFF     lea       rbp,[rsp-100h]"
        writeln!(output, "{:016X} ", instruction.ip())
            .expect("could not write to internal format buf");
        let start_index = (instruction.ip() - EXAMPLE_CODE_RIP) as usize;
        let instr_bytes = &data[start_index..start_index + instruction.len()];
        for b in instr_bytes.iter() {
            write!(output, "{:02X}", b).expect("could not write to internal format buf");
        }
        if instr_bytes.len() < HEXBYTES_COLUMN_BYTE_LENGTH {
            for _ in 0..HEXBYTES_COLUMN_BYTE_LENGTH - instr_bytes.len() {
                write!(output, "  ").expect("could not write to internal format buf");
            }
        }
    }

    Ok(output)
}
const HEXBYTES_COLUMN_BYTE_LENGTH: usize = 10;
const EXAMPLE_CODE_BITNESS: u32 = 64;
const EXAMPLE_CODE_RIP: u64 = 0x0000_7FFA_C46A_CDA4;
