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
                match opcode & 0x000F {
                    0x0000 => {
                        /* LD Vx, Vy instruction */
                        self.memory.write_reg(reg_x, val_y); /* Read the value stored in Vy and write it to Vx */
                    }
                    0x0001 => {
                        /* OR Vx, Vy instruction */
                        let val_x = self.memory.read_reg(reg_x);
                        self.memory.write_reg(reg_x, val_x | val_y); /* OR the value of Vx and Vy and write the result to Vx */
                    }
                    0x0002 => {
                        /* AND Vx, Vy instruction */
                        let val_x = self.memory.read_reg(reg_x);
                        self.memory.write_reg(reg_x, val_x & val_y); /* AND the value of Vx and Vy and write the result to Vx */
                    }
                    0x0003 => {
                        /* XOR Vx, Vy instruction */
                        let val_x = self.memory.read_reg(reg_x);
                        self.memory.write_reg(reg_x, val_x ^ val_y); /* XOR the value of Vx and Vy and write the result to Vx */
                    }
                    0x0004 => {
                        /* ADD Vx, Vy instruction */
                        let val_x = self.memory.read_reg(reg_x);
                        self.memory.write_reg(reg_x, val_x + val_y); /* ADD the value of Vx and Vy and write the result to Vx */
                    }
                    0x0005 => {
                        /* SUB Vx, Vy instruction */
                        let val_x = self.memory.read_reg(reg_x);
                        self.memory.write_reg(reg_x, val_x - val_y); /* SUB the value of Vx and Vy and write the result to Vx */
                    }
                    _ => {}
                }
            }

            _ => {}
        }
    }
}
