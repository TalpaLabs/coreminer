use std::io::{stdout, Write};

use super::{DebuggerUI, Status};
use crate::errors::Result;

pub struct CliUi;

impl CliUi {
    pub fn get_line(&self) -> Result<String> {
        let mut buf = String::new();
        let mut stdout = std::io::stdout();
        let stdin = std::io::stdin();
        print!("cm> ");
        stdout.flush()?;
        stdin.read_line(&mut buf)?;

        Ok(buf)
    }
}

impl DebuggerUI for CliUi {
    fn process_command(&self) -> crate::errors::Result<Status> {
        let line = self.get_line()?;
        dbg!(line);
        Ok(Status::Stop)
    }
}
