use crate::memory::Memory;

pub struct CPU {
    memory: Memory,
}

impl CPU {
    pub fn new() -> CPU {
        CPU {
            memory: Memory::new(),
        }
    }
    pub fn execute(&mut self, opcode: u16) {
        match opcode & 0xF000 {
            0x1000 => {
                println!("Opcode 0x1000");
            },
            _ => {

            }
        }
    }
}
