pub const MEM_SIZE: usize = 0x1000;

pub struct Memory {
    memory: [u8; MEM_SIZE],
    stack: [u16; 0x100],
    pc: u16,
    sp: u8,
    v: [u8; 16],
    i: u16,
    dt: u8,
    st: u8,
}

impl Memory {
    pub fn new() -> Memory {
        Memory {
            memory: [0; MEM_SIZE],
            stack: [0; 0x100],
            pc: 0,
            sp: 0,
            v: [0; 16],
            i: 0,
            dt: 0,
            st: 0,
        }
    }

    fn read(&self, addr: u16) -> u8 {
        self.memory[addr as usize]
    }

    fn write(&mut self, addr: u16, val: u8) {
        self.memory[addr as usize] = val;
    }

    fn read_reg(&self, reg: u8) -> u8 {
        self.v[reg as usize]
    }

    fn write_reg(&mut self, reg: u8, val: u8) {
        self.v[reg as usize] = val;
    }

    fn print_current_instr(&self) {
        println!("Current instruction: {:04X}", self.read_instr());
    }

    fn read_instr(&self) -> u16 {
        (self.read(self.pc) as u16) << 8 | self.read(self.pc + 1) as u16
    }

    fn consume_instr(&mut self) -> u16 {
        self.pc += 2;
        self.read_instr()
    }

}
