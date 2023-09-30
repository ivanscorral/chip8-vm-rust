/// The size of the memory in bytes.
pub const MEM_SIZE: usize = 0x1000;

/// The memory struct, representing the 4KB of RAM available to the Chip-8.
pub struct Memory {
   /// The memory, represented as an array of bytes.
   pub memory: [u8; MEM_SIZE],

   /// The stack, represented as an array of 16-bit words.
   pub stack: [u16; 0x100],

   /// The program counter, which points to the next instruction to be executed.
   pub pc: u16,

   /// The stack pointer, which points to the top of the stack.
   pub sp: u8,

   /// The general purpose registers, V0-VF.
   pub v: [u8; 16],

   /// The index register, I.
   pub i: u16,

   /// The delay timer register, DT.
   pub dt: u8,

   /// The sound timer register, ST.
   pub st: u8,
}

impl Memory {
   /// Creates a new `Memory` instance.
   pub(crate) fn new() -> Memory {
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

   /// Loads a byte from memory at the specified address.
   ///
   /// # Arguments
   ///
   /// * `addr` - The address to load from.
   ///
   /// # Returns
   ///
   /// The byte at the specified address.
   pub(crate) fn load(&self, addr: u16) -> u8 {
       self.memory[addr as usize]
   }

   /// Stores a byte to memory at the specified address.
   ///
   /// # Arguments
   ///
   /// * `addr` - The address to store to.
   /// * `val` - The byte to store.
   pub(crate) fn store(&mut self, addr: u16, val: u8) {
       self.memory[addr as usize] = val;
   }

   /// Reads a register value.
   ///
   /// # Arguments
   ///
   /// * `reg` - The register to read from.
   ///
   /// # Returns
   ///
   /// The value of the specified register.
   pub(crate) fn read_reg(&self, reg: u8) -> u8 {
       self.v[reg as usize]
   }

   /// Writes a value to a register.
   ///
   /// # Arguments
   ///
   /// * `reg` - The register to write to.
   /// * `val` - The value to write.
   pub(crate) fn write_reg(&mut self, reg: u8, val: u8) {
       self.v[reg as usize] = val;
   }

   /// Updates the delay timer and sound timer.
   pub(crate) fn update_timers(&mut self) {
       if self.dt > 0 {
           self.dt -= 1;
       }

       if self.st > 0 {
           self.st -= 1;
       }

       // TODO: Play sound when sound timer is 0
   }

   /// Pops a value from the stack.
   ///
   /// # Returns
   ///
   /// The value that was popped from the stack.
   pub(crate) fn pop_stack(&mut self) -> u16 {
       self.sp -= 1;
       self.stack[self.sp as usize]
   }

   /// Pushes a value onto the stack.
   ///
   /// # Arguments
   ///
   /// * `val` - The value to push onto the stack.
   pub(crate) fn push_stack(&mut self, val: u16) {
       self.stack[self.sp as usize] = val;
         self.sp += 1;
   }

   /// Resets the memory to its initial state.
    pub(crate) fn reset(&mut self) {
        self.pc = 0x200;
        self.sp = 0;
        self.v = [0; 16];
        self.i = 0;
        self.dt = 0;
        self.st = 0;
    }


    /// Reads the next instruction from memory.
    ///
    /// # Returns
    ///
    /// The next instruction as a 16-bit value.
    pub(crate) fn read_instr(&self) -> u16 {
        (self.load(self.pc) as u16) << 8 | self.load(self.pc + 1) as u16
    }

    /// Loads the sprites into memory.
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

    /// Loads a sprite from memory.
    ///
    /// # Arguments
    ///
    /// * `nibble` - The number of nibbles (4-bit values) to load.
    ///
    /// # Returns
    ///
    /// The loaded sprite as a vector of bytes.
    pub(crate) fn load_sprite(&self, nibble: usize) -> Vec<u8> {
        let mut sprite = Vec::new();
        let start_addr = self.i as usize;

        if start_addr + nibble > MEM_SIZE {
            return sprite;
        }

        for offset in 0..nibble {
            let byte = self.load((start_addr + offset) as u16);
            sprite.push(byte);
        }

        sprite
    }

}
