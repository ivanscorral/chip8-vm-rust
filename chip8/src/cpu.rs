use crate::gpu::{Coordinate, GPU};
use crate::instructions::{parse_opcode, Opcode};
use crate::memory::Memory;

use rand::Rng;

/// Represents a binary-coded decimal (BCD) value.
///
/// A BCD is a class of binary encodings of decimal numbers where each decimal
/// digit is represented by a fixed number of binary digits. In the context
/// of this representation, each decimal digit is represented by an `u8`.
///
/// For example:
/// - The number 123 would be represented as `(1, 2, 3)`.
/// - The number 45 would be represented as `(0, 4, 5)`.
type BCD = (u8, u8, u8);

/// Trait for types that can be represented as a binary-coded decimal (BCD).
///
/// This trait provides a method to convert numbers into their BCD representation.
pub trait BCDRepresentable {
    /// Converts the number into its BCD representation.
    ///
    /// # Returns
    ///
    /// Returns a tuple of three `u8` values, representing the hundreds,
    /// tens, and ones places of the number, respectively.
    fn to_bcd(&self) -> BCD;
}

impl BCDRepresentable for u8 {
    fn to_bcd(&self) -> BCD {
        let hundreds = self / 100;
        let tens = (self % 100) / 10;
        let ones = self % 10;
        (hundreds, tens, ones)
    }
}

/// Represents the CPU of the Chip-8 virtual machine.
pub struct CPU {
    pub memory: Memory,
    gpu: GPU,
    pub key_state: u16,
    pub halt: bool,
    waiting_for_key: Option<u8>,
}

impl CPU {
    pub fn new() -> CPU {
        CPU {
            memory: Memory::new(),
            gpu: GPU::new(),
            key_state: 0x0000,
            halt: false,
            waiting_for_key: None,
        }
    }
    pub fn get_gpu(&mut self) -> &mut GPU {
        &mut self.gpu
    }
    /// Executes the given opcode on the CPU.
    pub fn execute(&mut self, raw_opcode: u16) {
        let (opcode, reg_x, reg_y) = parse_opcode(raw_opcode);

        let val_x = self.memory.read_reg(reg_x);
        let val_y = self.memory.read_reg(reg_y);

        match opcode {
            Opcode::Halt => {
                self.halt = true;
                println!("Halt");
                return;
            }
            Opcode::ClearScreen => self.gpu.reset(),
            Opcode::Return => self.memory.pc = self.memory.pop_stack(),
            Opcode::JumpToAddress(addr) => {
                self.memory.pc = addr;
                return;
            }
            Opcode::CallAddress(addr) => {
                self.memory.push_stack(self.memory.pc.wrapping_add(2));
                self.memory.pc = addr;
                return;
            }
            Opcode::SkipIfRegEqualsByte(k) => {
                if val_x == k {
                    self.increment()
                }
            }
            Opcode::SkipIfRegNotEqualsByte(k) => {
                println!("V{:X} = 0x{:02X}\t k = {:02X}", reg_x, val_x, k);
                if val_x != k {
                    println!("Skipping");
                    self.increment()
                }
            }
            Opcode::SkipIfRegEqualsReg => {
                if val_x == val_y {
                    self.increment()
                }
            }
            Opcode::LoadByteIntoReg(k) => self.memory.write_reg(reg_x, k),
            Opcode::AddByteToReg(k) => self.memory.write_reg(reg_x, val_x.wrapping_add(k)),
            Opcode::LoadRegIntoReg => self.memory.write_reg(reg_x, val_y),
            Opcode::OrRegWithReg => self.memory.write_reg(reg_x, val_x | val_y),
            Opcode::AndRegWithReg => self.memory.write_reg(reg_x, val_x & val_y),
            Opcode::XorRegWithReg => self.memory.write_reg(reg_x, val_x ^ val_y),

            Opcode::AddRegToReg => {
                let sum = self.add_overflow(val_x, val_y);
                self.memory.write_reg(reg_x, sum);
            }
            Opcode::SubtractRegFromReg => {
                let diff = self.sub_overflow(val_x, val_y);
                self.memory.write_reg(reg_x, diff);
            }

            Opcode::ShiftRight => {
                /* SHR Vx {, Vy} instruction */
                let x_ls_bit = val_x & 0x1;
                self.memory.write_reg(0xF, x_ls_bit); /* Set VF to the least significant bit of Vx */
                self.memory.write_reg(reg_x, val_x.wrapping_shr(1)); /* Perform a right shift on Vx */
            }

            Opcode::SubstractRegFromOtherReg => {
                let diff = self.sub_overflow(val_y, val_x);
                self.memory.write_reg(reg_x, diff);
            }
            Opcode::ShiftLeft => {
                /* SHL Vx {, Vy} instruction */
                let x_ms_bit = (val_x & 0x80) >> 7;
                self.memory.write_reg(0xF, x_ms_bit); /* Set VF to the most significant bit of Vx */
                self.memory.write_reg(reg_x, val_x.wrapping_shl(1)); /* DIV the value of Vx by 2 and write the result to Vx */
            }
            Opcode::SkipIfRegNotEqualsReg => {
                if val_x != val_y {
                    self.increment()
                }
            }
            Opcode::LoadIndex(addr) => self.memory.i = addr,
            Opcode::JumpToAddressPlusV0(addr) => {
                let v0 = self.memory.read_reg(0) as u16;
                self.memory.pc = addr.wrapping_add(v0);
            }
            Opcode::RandomByte(k) => {
                let mut rng = rand::thread_rng();
                let rand_u8 = rng.gen::<u8>();
                self.memory.write_reg(reg_x, rand_u8 & k);
            }
            Opcode::DrawSprite(nibble) => {
                /* DRW Vx, Vy, nibble instruction */
                let coords: Coordinate = (val_x.into(), val_y.into());
                let sprite = self.memory.load_sprite(nibble.into());
                self.memory
                    .write_reg(0xF, self.gpu.draw_sprite(coords, sprite));
            }
            Opcode::SkipIfKeyPressed => {
                /* SKP Vx instruction */
                if self.is_key_pressed(val_x.into()) {
                    self.increment();
                }
            }
            Opcode::SkipIfKeyNotPressed => {
                /* SKNP Vx instruction */
                if !self.is_key_pressed(val_x.into()) {
                    self.increment();
                }
            }
            Opcode::LoadDelayTimerIntoReg => self.memory.write_reg(reg_x, self.memory.dt),

            Opcode::LoadKeyIntoReg => {
                if self.key_state == 0 {
                    // No key is being pressed, so we wait for a key press
                    self.waiting_for_key = Some(reg_x);
                    self.memory.pc = self.memory.pc.wrapping_sub(2); // Decrement PC to repeat this instruction until a key is pressed
                } else {
                    for i in 0..16 {
                        // A key is already being pressed, so we store its value in Vx
                        if self.is_key_pressed(i) {
                            self.memory.write_reg(reg_x, i as u8);
                            break;
                        }
                    }
                }
            }
            Opcode::LoadRegIntoDelayTimer => self.memory.dt = val_x,
            Opcode::LoadRegIntoSoundTimer => self.memory.st = val_x,
            Opcode::AddRegToIndex => self.memory.i = self.memory.i.wrapping_add(val_x as u16),
            Opcode::LoadFontIntoReg => self.memory.i = (val_x * 5) as u16,
            Opcode::LoadBCDIntoMem => {
                let i = self.memory.i;
                let bcd_representation = val_x.to_bcd();
                self.memory.store(i, bcd_representation.0);
                self.memory.store(i + 1, bcd_representation.1);
                self.memory.store(i + 2, bcd_representation.2);
            }
            Opcode::StoreRegsIntoMem => {
                for offset in 0..0x10 {
                    self.memory.store(
                        self.memory.i.wrapping_add(offset as u16),
                        self.memory.read_reg(offset as u8),
                    );
                }
            }
            Opcode::LoadRegsFromMem => {
                for offset in 0..0x10 {
                    self.memory.write_reg(
                        offset,
                        self.memory.load(self.memory.i.wrapping_add(offset as u16)),
                    );
                }
            }
            _ => {
                println!("Unknown opcode: {:04X}", raw_opcode);
                self.halt = true;
            }
        }
        self.increment();

        self.print_registers()
    }

    fn increment(&mut self) {
        self.memory.pc = self.memory.pc.wrapping_add(2);
    }

    fn update_timers(&mut self) {
        self.memory.update_timers();
    }

    pub fn cycle(&mut self) {
        self.execute(self.memory.read_instr());
        self.update_timers();
        //self.poll();
    }

    /// Prints the values of all the CPU registers.
    pub fn print_registers(&self) {
        println!(
            "PC: 0x{:04X}\tSP: 0x{:04X}\tI: 0x{:04X}",
            self.memory.pc, self.memory.sp, self.memory.i
        );

        // Number of columns in the output
        let num_cols = 4;

        for row in 0..4 {
            for col in 0..num_cols {
                // Calculate the current register index based on row and column
                let reg_index = row + 4 * col;
                if col < num_cols - 1 {
                    print!(
                        "V{:X}: 0x{:02X}\t",
                        reg_index,
                        self.memory.read_reg(reg_index)
                    );
                } else {
                    println!(
                        "V{:X}: 0x{:02X}",
                        reg_index,
                        self.memory.read_reg(reg_index)
                    );
                }
            }
        }
        println!();
    }

    /// Prints the memory region from `start` to `end` (inclusive) with the specified number of tabs.
    pub fn print_memory_region(&self, start: u16, end: u16, tabs_count: usize) {
        let mut tabs = 0;
        for i in start..end {
            if tabs == 0 {
                print!("0x{:04X}: ", i);
            }
            if tabs < tabs_count - 1 {
                print!("{:02X} ", self.memory.load(i));
                tabs += 1;
            } else {
                println!("{:02X}", self.memory.load(i));
                tabs = 0;
            }
        }
        println!();
    }

    pub fn key_pressed(&mut self, index: usize) {
        self.key_state |= 1 << index;
        if let Some(reg) = self.waiting_for_key {
            self.memory.write_reg(reg, index as u8);
            self.waiting_for_key = None;
        }
        println!("key_state: {:04X}", self.key_state);
    }

    pub fn key_released(&mut self, index: usize) {
        self.key_state &= !(1 << index);
    }

    pub fn is_key_pressed(&self, index: usize) -> bool {
        self.key_state & (1 << index) != 0
    }

    pub fn reset(&mut self) {
        println!("Resetting CPU");
        self.memory.reset();
        self.gpu.reset();
        self.halt = false;
    }

    fn handle_overflow(&mut self, result: u8, overflow: bool) -> u8 {
        self.memory.write_reg(0xF, if overflow { 1 } else { 0 });
        result
    }

    fn add_overflow(&mut self, x: u8, y: u8) -> u8 {
        let (sum, overflow) = x.overflowing_add(y);
        self.handle_overflow(sum, overflow)
    }

    fn sub_overflow(&mut self, x: u8, y: u8) -> u8 {
        let (diff, overflow) = x.overflowing_sub(y);
        self.handle_overflow(diff, overflow)
    }

    pub fn load_program(&mut self, program: &[u8]) {
        println!("Loading program");
        for i in 0..program.len() {
            self.memory.store((i + 0x200) as u16, program[i]);
        }
        self.print_memory_region(0x200, 0x400, 16);
    }
}
