use std::fmt::{Display, Write};

use crate::errors::Result;

const CODE_BITNESS: u32 = 64;

use iced_x86::{
    Decoder, DecoderOptions, Formatter, FormatterOutput, FormatterTextKind, IntelFormatter,
    NasmFormatter,
};

// Custom formatter output that stores the output in a vector.
struct MyFormatterOutput {
    vec: Vec<(String, FormatterTextKind)>,
}

impl MyFormatterOutput {
    pub fn new() -> Self {
        Self { vec: Vec::new() }
    }
}

impl FormatterOutput for MyFormatterOutput {
    fn write(&mut self, text: &str, kind: FormatterTextKind) {
        // This allocates a string. If that's a problem, just call print!() here
        // instead of storing the result in a vector.
        self.vec.push((String::from(text), kind));
    }
}

pub fn disassemble(data: &[u8], rip: u64) -> Result<String> {
    let mut buf = String::new();
    let bytes = data;
    let mut decoder = Decoder::with_ip(CODE_BITNESS, bytes, rip, DecoderOptions::NONE);

    let mut formatter = NasmFormatter::new();
    // padding
    formatter.options_mut().set_first_operand_char_index(16);

    // numbers stuff
    formatter.options_mut().set_hex_suffix("");
    formatter.options_mut().set_hex_prefix("");
    formatter.options_mut().set_decimal_suffix("");
    formatter.options_mut().set_decimal_prefix("0d");
    formatter.options_mut().set_octal_suffix("");
    formatter.options_mut().set_octal_prefix("0o");
    formatter.options_mut().set_binary_suffix("");
    formatter.options_mut().set_binary_prefix("0b");

    // memory stuff
    formatter.options_mut().set_show_symbol_address(true);
    formatter.options_mut().set_rip_relative_addresses(false);
    formatter
        .options_mut()
        .set_memory_size_options(iced_x86::MemorySizeOptions::Always);
    let mut output = MyFormatterOutput::new();
    for instruction in &mut decoder {
        apbuf(&mut buf, format!("{:016x}\t", instruction.ip()));
        output.vec.clear();
        formatter.format(&instruction, &mut output);
        for (text, _kind) in output.vec.iter() {
            apbuf(&mut buf, text);
        }
        apbuf(&mut buf, "\n");
    }

    Ok(buf)
}

fn apbuf(buf: &mut String, text: impl Display) {
    write!(buf, "{text}").expect("could not write to internal buffer at disassembly")
}
