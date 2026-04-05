use alloc::string::String;
use alloc::vec::Vec;

pub enum ConsoleEvent {
    Submit(String),
    Cancel,
    Interrupt,
}

pub struct LineConsole {
    line: Vec<u8>,
}

impl LineConsole {
    pub fn new() -> Self {
        Self { line: Vec::new() }
    }

    pub fn clear_line(&mut self) {
        self.line.clear();
    }

    pub fn handle_byte(&mut self, byte: u8) -> Option<ConsoleEvent> {
        match byte {
            b'\r' | b'\n' => {
                let line = core::mem::take(&mut self.line);
                let line = String::from_utf8(line).unwrap_or_else(|_| String::new());
                Some(ConsoleEvent::Submit(line))
            }
            0x03 => {
                self.line.clear();
                Some(ConsoleEvent::Interrupt)
            }
            0x08 | 0x7f => {
                if !self.line.is_empty() {
                    self.line.pop();
                }
                None
            }
            0x20..=0x7e => {
                self.line.push(byte);
                None
            }
            _ => Some(ConsoleEvent::Cancel),
        }
    }
}
