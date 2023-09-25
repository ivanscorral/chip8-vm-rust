pub const MEM_SIZE: usize = 0x1000;

pub struct Memory {
    memory: [u8; MEM_SIZE],
    stack: [u16; 0x100],
    pub pc: u16,
    pub sp: u8,
    v: [u8; 16],
    pub i: u16,
    pub dt: u8,
    pub st: u8,
}

impl Memory {
    pub fn new() -> Memory {
        Memory {
            memory: [0; MEM_SIZE],
            stack: [0; 0x100],
            pc: 0x200,
            sp: 0,
            v: [0; 16],
            i: 0,
            dt: 0,
            st: 0,
        }
    }


    pub fn load(&self, addr: u16) -> u8 {
        self.memory[addr as usize]
    }

    pub fn store(&mut self, addr: u16, val: u8) {
        self.memory[addr as usize] = val;
    }

    pub fn read_reg(&self, reg: u8) -> u8 {
        self.v[reg as usize]
    }

    pub fn write_reg(&mut self, reg: u8, val: u8) {
        self.v[reg as usize] = val;
    }


    pub fn update_timers(&mut self) {
        if self.dt > 0 {
            self.dt -= 1;
        }

        if self.st > 0 {
            self.st -= 1;
        }
    }

    pub fn pop_stack(&mut self) -> u16 {
        self.sp -= 1;
        self.stack[self.sp as usize]
    }

    pub fn push_stack(&mut self, val: u16) {
        self.sp += 1;
        self.stack[self.sp as usize] = val;
    }

    pub fn reset(&mut self) {
        self.pc = 0x200;
        self.sp = 0;
        self.v = [0; 16];
        self.i = 0;
        self.dt = 0;
        self.st = 0;

    }

    pub fn print_current_instr(&self) {
        println!("Current instruction: {:04X}", self.read_instr());
    }

    pub fn read_instr(&self) -> u16 {
        (self.load(self.pc) as u16) << 8 | self.load(self.pc + 1) as u16
    }
}
