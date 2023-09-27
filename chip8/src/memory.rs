pub const MEM_SIZE: usize = 0x1000;

pub struct Memory {
    pub memory: [u8; MEM_SIZE],
    pub stack: [u16; 0x100],
    pub pc: u16,
    pub sp: u8,
    pub v: [u8; 16],
    pub i: u16,
    pub dt: u8,
    pub st: u8,
}

impl Memory {
    pub fn new() -> Memory {
        let mut mem = Memory {
            memory: [0; MEM_SIZE],
            stack: [0; 0x100],
            pc: 0x200,
            sp: 0,
            v: [0; 16],
            i: 0,
            dt: 0,
            st: 0,
        };
        mem.load_sprites();
        mem
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

        // TODO: Play sound when sound timer is 0

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

    pub fn read_instr(&self) -> u16 {
        (self.load(self.pc) as u16) << 8 | self.load(self.pc + 1) as u16
    }

    fn load_sprites(&mut self) {
        let sprites: [u8; 0x50] = [
            0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
            0x20, 0x60, 0x20, 0x20, 0x70, // 1
            0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
            0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
            0x90, 0x90, 0xF0, 0x10, 0x10, // 4
            0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
            0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
            0xF0, 0x10, 0x20, 0x40, 0x40, // 7
            0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
            0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
            0xF0, 0x90, 0xF0, 0x90, 0x90, // A
            0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
            0xF0, 0x80, 0x80, 0x80, 0xF0, // C
            0xE0, 0x90, 0x90, 0x90, 0xE0, // D
            0xF0, 0x98, 0xF0, 0x80, 0xF0, // E
            0xF0, 0x80, 0xF0, 0x80, 0x80, // F
        ];

        self.memory[0..0x50].copy_from_slice(&sprites);
    }
}
