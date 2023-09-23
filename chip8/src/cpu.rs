extern crate rand;

use crate::memory::Memory;
use rand::Rng;

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
            0x0000 => {
                match opcode & 0x000F {
                    0x0000 => {
                        // TODO Implement CLS instruction
                    }
                    0x000E => {
                        /* RET instruction */
                        let addr = self.memory.pop_stack();
                        self.memory.set_pc(addr);
                    }
                    _ => {}
                }
            }
            0x1000 => {
                /* JUMP addr instruction */
                let addr = opcode & 0x0FFF;
                self.memory.set_pc(addr);
            }
            0x2000 => {
                /* CALL addr instruction */
                let addr = opcode & 0x0FFF;
                self.memory.push_stack(self.memory.get_pc());
                self.memory.set_pc(addr);
            }
            0x3000 => {
                /* SE Vx, byte instruction */
                let reg: u8 = ((opcode & 0x0F00) >> 8) as u8;
                let k = (opcode & 0x00FF) as u8;
                if self.memory.read_reg(reg) == k
                /* Vx == kk */
                {
                    self.memory.set_pc(self.memory.get_pc() + 2);
                }
            }
            0x4000 => {
                /* SNE Vx, byte instruction */
                let reg: u8 = ((opcode & 0x0F00) >> 8) as u8;
                let k = (opcode & 0x00FF) as u8;
                if self.memory.read_reg(reg) != k
                /* Vx != kk */
                {
                    self.memory.set_pc(self.memory.get_pc() + 2);
                }
            }
            0x5000 => {
                /* SE Vx, Vy instruction */
                let reg_x: u8 = ((opcode & 0x0F00) >> 8) as u8;
                let reg_y: u8 = ((opcode & 0x00F0) >> 4) as u8;
                if self.memory.read_reg(reg_x) == self.memory.read_reg(reg_y)
                /* Vx == Vy */
                {
                    self.memory.set_pc(self.memory.get_pc() + 2); /* Skip next instruction */
                }
            }
            0x6000 => {
                /* LD Vx, Vy instruction */
                let reg_x: u8 = ((opcode & 0x0F00) >> 8) as u8;
                let k = (opcode & 0x00FF) as u8;
                self.memory.write_reg(reg_x, k); /* Set Vx to kk */
            }
            0x7000 => {
                /* ADD Vx, byte instruction */
                let reg_x: u8 = ((opcode & 0x0F00) >> 8) as u8;
                let k = (opcode & 0x00FF) as u8;
                self.memory
                    .write_reg(reg_x, k + self.memory.read_reg(reg_x)); /* Vx = Vx + kk */
            }
            0x8000 => {
                let reg_x = (opcode & 0x0F00) as u8;
                let reg_y = (opcode & 0x00F0) as u8;
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
                        self.memory.write_reg(reg_x, val_x.wrapping_add(val_y));
                        /* ADD the value of Vx and Vy and write the result to Vx */
                    }
                    0x0005 => {
                        /* SUB Vx, Vy instruction */
                        self.memory.write_reg(reg_x, val_x.wrapping_sub(val_y));
                        /* SUB the value of Vx and Vy and write the result to Vx */
                    }
                    0x0006 => {
                        /* SHR Vx {, Vy} instruction */
                        let x_ls_bit = val_x & 0x1;
                        self.memory.write_reg(0xF, x_ls_bit); /* Set VF to the least significant bit of Vx */
                        self.memory.write_reg(reg_x, val_x >> 1); /* DIV the value of Vx by 2 and write the result to Vx */
                    }
                    0x0007 => {
                        /* SUBN Vx, Vy instruction */
                        self.memory.write_reg(0xF, (val_y > val_x) as u8); /* Set VF to the most significant bit of Vy */
                        self.memory.write_reg(reg_x, reg_y.wrapping_sub(val_x)); /* SUB the value of Vx and Vy and write the result to Vx */
                    }
                    0x000E => {
                        /* SHL Vx {, Vy} instruction */
                        let x_ms_bit = val_x & 0x80 >> 7;
                        self.memory.write_reg(0xF, x_ms_bit); /* Set VF to the most significant bit of Vx */
                        self.memory.write_reg(reg_x, val_x << 1); /* DIV the value of Vx by 2 and write the result to Vx */
                    }
                    _ => {}
                }
            }
            0x9000 => {
                /* SNE Vx, Vy instruction */
                let reg_x: u8 = ((opcode & 0x0F00) >> 8) as u8;
                let reg_y: u8 = ((opcode & 0x00F0) >> 4) as u8;
                if self.memory.read_reg(reg_x) != self.memory.read_reg(reg_y) {
                    /* Vx != Vy */
                    self.memory.set_pc(self.memory.get_pc() + 2); /* Skip next instruction */
                }
            }
            0xA000 => {
                /* LD I, addr instruction */
                let n = opcode & 0x0FFF;
                self.memory.set_i(n); /* Set I to addr */
            }
            0xB000 => {
                /* JP V0, addr instruction */
                let n = opcode & 0x0FFF;
                self.memory.set_pc(n + self.memory.read_reg(0x0) as u16); /* Jump to addr + V0 */
            }
            0xC000 => {
                /* RND Vx, byte instruction */
                let reg_x = ((opcode & 0x0F00) >> 8) as u8;
                let k = (opcode & 0x00FF) as u8;
                let mut rng = rand::thread_rng();
                let rand_u8 = rng.gen::<u8>();
                self.memory.write_reg(reg_x, rand_u8 & k); /* Set Vx to a random byte ANDed with kk */
            }
            0xD000 => {
                /* DRW Vx, Vy, nibble instruction */
                unimplemented!();
            }
            0xE000 => {
                /* SKP Vx instruction */
                unimplemented!();
            }
            0xF000 => {
                let reg_x = ((opcode & 0x0F00) >> 8) as u8;
                match opcode & 0x0FF {
                    0x0007 => {
                        unimplemented!()
                    }
                    0x000A => {
                        unimplemented!()
                    }
                    0x0015 => {
                        self.memory.set_dt(self.memory.read_reg(reg_x)); /* Set delay timer to Vx */
                    }
                    0x0018 => {
                        self.memory.set_st(self.memory.read_reg(reg_x)); /* Set sound timer to Vx */
                    }
                    0x001E => {
                        self.memory.set_i(
                            self.memory.read_reg(reg_x) as u16 + self.memory.read_reg(0x0) as u16,
                        ); /* Set I to I + Vx */
                    }
                    0x0029 => {
                        unimplemented!();
                    }
                    0x0033 => {
                        unimplemented!();
                    }
                    0x0055 => {
                        let mut offset: u8 = 0x0;
                        for i in 0..0x10 {
                            self.memory.write(i + offset as u16, self.memory.read_reg(reg_x + offset));
                            offset += 1;
                        }
                    }
                    0x0065 => {
                        let mut offset = 0x00;
                        for i in 0..0x10 {
                            self.memory
                                .write_reg(offset, self.memory.read(i + offset as u16));
                            offset += 1;
                        }
                    }
                    _ => {}
                }
            }
            _ => {}
        }
        self.memory.set_pc(self.memory.get_pc() + 2);
    }
}
