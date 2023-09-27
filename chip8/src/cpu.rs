extern crate rand;

use crate::gpu::GPU;
use crate::memory::Memory;

use rand::Rng;

trait Keyboard {
    fn poll(&mut self);
}

impl Keyboard for CPU {
    fn poll(&mut self) {
        unimplemented!()
    }
}

pub struct CPU {
    memory: Memory,
    gpu: GPU,
    key_state: [u8; 16],
    pub halt: bool,
}

impl CPU {
    pub fn new() -> CPU {
        CPU {
            memory: Memory::new(),
            gpu: GPU::new(),
            key_state: [0; 16],
            halt: false,
        }
    }

    pub fn execute(&mut self, opcode: u16) {
        println!("Executing opcode: {:04X}", opcode);
        let addr = opcode & 0x0FFF;
        let reg_x = ((opcode & 0x0F00) >> 8) as u8;
        let reg_y = ((opcode & 0x00F0) >> 4) as u8;
        let k = (opcode & 0x00FF) as u8;
        println!("x: V{:01X}\ty: V{:01X}\nkk: 0x{:02X}\tnnn: 0x{:03X}", reg_x, reg_y, k, addr);
        match opcode & 0xF000 {
            0x0000 => {
                match opcode & 0x00FF {
                    0x0000 => {
                        /* HLT instruction */
                        self.halt = true;
                    }
                    0x00E0 => {
                        /* CLS instruction */
                        self.gpu.reset();
                    }
                    0x00EE => {
                        /* RET instruction */
                        self.memory.pc = self.memory.pop_stack();
                    }
                    _ => {}
                }
            }
            0x1000 => {
                /* JUMP addr instruction */
                // TODO: Test JUMP instruction
                self.memory.pc = addr;
            }
            0x2000 => {
                /* CALL addr instruction */
                // TODO: Test CALL instruction
                self.memory.push_stack(self.memory.pc);
                self.memory.pc = addr
            }
            0x3000 => {
                /* SE Vx, byte instruction */
                if self.memory.read_reg(reg_x) == k {
                    self.increment()
                }
            }
            0x4000 => {
                /* SNE Vx, byte instruction */
                if self.memory.read_reg(reg_x) != k {
                    self.increment()
                }
            }
            0x5000 => {
                /* SE Vx, Vy instruction */
                if self.memory.read_reg(reg_x) == self.memory.read_reg(reg_y) {
                    self.increment()
                }
            }
            0x6000 => {
                /* LD Vx, k instruction */

                self.memory.write_reg(reg_x, k); /* Set Vx to kk */
            }
            0x7000 => {
                /* ADD Vx, byte instruction */
                self.memory
                    .write_reg(reg_x, self.memory.read_reg(reg_x).wrapping_add(k));
                /* Vx = Vx + kk */
            }
            0x8000 => {
                let val_y = self.memory.read_reg(reg_y);
                let val_x = self.memory.read_reg(reg_x);
                match opcode & 0x000F {
                    0x0000 => {
                        /* LD Vx, Vy instruction */
                        self.memory.write_reg(reg_x, val_y); /* Read the value stored in Vy and write it to Vx */
                    }
                    0x0001 => {
                        /* OR Vx, Vy instruction */
                        self.memory.write_reg(reg_x, val_x | val_y); /* OR the value of Vx and Vy and write the result to Vx */
                    }
                    0x0002 => {
                        /* AND Vx, Vy instruction */
                        self.memory.write_reg(reg_x, val_x & val_y); /* AND the value of Vx and Vy and write the result to Vx */
                    }
                    0x0003 => {
                        /* XOR Vx, Vy instruction */
                        self.memory.write_reg(reg_x, val_x ^ val_y); /* XOR the value of Vx and Vy and write the result to Vx */
                    }
                    0x0004 => {
                        /* ADD Vx, Vy instruction */
                        if (val_x as u16) + (val_y as u16) > 0xFF {
                            self.memory.write_reg(0xF, 1);
                        } else {
                            self.memory.write_reg(0xF, 0);
                        }
                        let sum = val_x.wrapping_add(val_y);
                        self.memory.write_reg(reg_x, sum)
                        /* ADD the value of Vx and Vy and write the result to Vx */
                    }
                    0x0005 => {
                        /* SUB Vx, Vy instruction */
                        self.memory.write_reg(0xF, (val_x > val_y) as u8);
                        self.memory.write_reg(reg_x, val_x.wrapping_sub(val_y));
                    }
                    0x0006 => {
                        /* SHR Vx {, Vy} instruction */
                        let x_ls_bit = val_x & 0x1;
                        self.memory.write_reg(0xF, x_ls_bit); /* Set VF to the least significant bit of Vx */
                        self.memory.write_reg(reg_x, val_x.wrapping_shr(1)); /* DIV the value of Vx by 2 and write the result to Vx */
                    }
                    0x0007 => {
                        /* SUBN Vx, Vy instruction */
                        self.memory.write_reg(0xF, (val_y > val_x) as u8); /* Set VF to the most significant bit of Vy */
                        self.memory.write_reg(reg_x, val_y.wrapping_sub(val_x));
                        /* SUB the value of Vx and Vy and write the result to Vx */
                    }
                    0x000E => {
                        /* SHL Vx {, Vy} instruction */
                        let x_ms_bit = (val_x & 0x80) >> 7;
                        self.memory.write_reg(0xF, x_ms_bit); /* Set VF to the most significant bit of Vx */
                        self.memory.write_reg(reg_x, val_x.wrapping_shl(1)); /* DIV the value of Vx by 2 and write the result to Vx */
                    }
                    _ => {}
                }
            }
            0x9000 => {
                /* SNE Vx, Vy instruction */
                if self.memory.read_reg(reg_x) != self.memory.read_reg(reg_y) {
                    self.increment()
                }
            }
            0xA000 => {
                /* LD I, addr instruction */
                self.memory.i = opcode & 0x0FFF; /* Set I to addr */
            }
            0xB000 => {
                /* JP V0, addr instruction */
                self.memory.pc = addr.wrapping_add(self.memory.read_reg(0) as u16);
                /* Jump to addr + V0 */
            }
            0xC000 => {
                /* RND Vx, byte instruction */
                let mut rng = rand::thread_rng();
                let rand_u8 = rng.gen::<u8>();
                self.memory.write_reg(reg_x, rand_u8 & k); /* Set Vx to a random byte, which is the result of a bitwise AND with kk */
            }
            0xD000 => {
                /* DRW Vx, Vy, nibble instruction */
                // TODO: Unimplemented
                unimplemented!();
            }
            0xE000 => {
                /* SKP Vx instruction */
                // TODO: Unimplemented
                unimplemented!();
            }
            0xF000 => {
                let val_x = self.memory.read_reg(reg_x);
                match opcode & 0x0FF {
                    0x0007 => {
                        /* LD Vx, DT instruction */
                        self.memory.write_reg(reg_x, self.memory.dt); /* Set Vx to DT */
                    }
                    0x000A => {
                        /* LD Vx, K instruction */
                        // TODO: Implement memory.read_key()
                        unimplemented!()
                    }
                    0x0015 => {
                        /* LD DT, Vx instruction */
                        self.memory.dt = val_x; /* Set delay timer to Vx */
                    }
                    0x0018 => {
                        /* LD ST, Vx instruction */
                        self.memory.st = val_x; /* Set sound timer to Vx */
                    }
                    0x001E => {
                        /* ADD I, Vx instruction */
                        self.memory.i = val_x as u16 + self.memory.i; /* Set I to I + Vx */
                    }
                    0x0029 => {
                        /* LD F, Vx instruction */
                        unimplemented!();
                    }
                    0x0033 => {
                        /* LD B, Vx instruction */
                        unimplemented!();
                    }
                    0x0055 => {
                        /* LD [I], Vx instruction */
                        for offset in 0..0x10 {
                            self.memory.store(
                                self.memory.i + offset as u16,
                                self.memory.read_reg(offset as u8),
                            );
                        }
                    }
                    0x0065 => {
                        /* LD Vx, [I] instruction */
                        for offset in 0..0x10 {
                            self.memory
                                .write_reg(offset, self.memory.load(self.memory.i + offset as u16));
                        }
                    }
                    _ => {}
                }
            }

            _ => {
                println!("Unknown opcode: {:04X}", opcode);
                panic!();
            }
        }
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
        self.increment();
    }

    pub fn print_registers(&self) {
        println!("Registers\nPC: 0x{:04X}\t SP: 0x{:04X}, I: 0x{:04X}", self.memory.pc, self.memory.sp, self.memory.i);

        let mut tabs = 0;
        for i in 0..16 {
            if tabs < 3 {
                print!("V{:X}: 0x{:02X}\t", i, self.memory.read_reg(i));
                tabs += 1;
            } else {
                println!("V{:X}: 0x{:02X}", i, self.memory.read_reg(i));
                tabs = 0;
            }
        }
        println!();
    }

    pub fn print_memory_region(&self, start: u16, end: u16, tabs_count: usize) {
        let mut tabs = 0;
        for i in start..=end {
            if tabs == 0 {
                print!("0x{:04X}: ", i);
            }
            if tabs < tabs_count - 1 {
                print!("{:02X} ", self.memory.load(i));
                tabs += 1;
            }else {
                println!("{:02X}", self.memory.load(i));
                tabs = 0;
            }
        }
        println!();
    }

    pub fn key_pressed(&mut self, key: u8) {
        self.key_state[key as usize] = 1;
    }

    pub fn key_released(&mut self, key: u8) {
        self.key_state[key as usize] = 0;
    }

    pub fn reset(&mut self) {
        self.memory.reset();
        self.gpu.reset();
        self.halt = false;
    }

    pub fn load_program(&mut self, program: &[u8]) {
        println!("Loading program");
        for i in 0..program.len() {
            self.memory.store((i + 0x200) as u16, program[i]);
        }
        self.print_memory_region(0x200, 0xFFF, 32);
    }
}
