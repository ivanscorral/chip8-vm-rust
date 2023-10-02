#[cfg(test)]

pub mod tests {
    use chip8::cpu::CPU;
    #[test]
    fn test_jump_to_address() {
        let mut cpu = CPU::new();
        cpu.memory.pc = 0x200;
        cpu.execute(0x1234); // Jump to address 0x234
        assert_eq!(cpu.memory.pc, 0x234);
    }
    #[test]
    fn test_call_address() {
        let mut cpu = CPU::new();
        cpu.memory.pc = 0x200;
        cpu.execute(0x2234); // Call address 0x234

        // Check if the return address (next instruction) is pushed onto the stack
        assert_eq!(cpu.memory.stack[cpu.memory.sp as usize], 0x202);

        // Check if the PC is set to the address from the opcode
        assert_eq!(cpu.memory.pc, 0x234);
    }

    #[test]
    fn test_add_byte() {
        let mut cpu = CPU::new();
        let program = [
            0x60, 0x02, /* LD V0, 0x02 */
            0x70, 0x02, /* ADD V0, 0x02 */
        ];
        cpu.load_program(&program);
        cpu.cycle();
        assert_eq!(cpu.memory.v[0], 0x02);
        cpu.cycle();
        assert_eq!(cpu.memory.v[0], 0x04);
    }
    #[test]
    fn test_skip_if_equal_byte() {
        let mut cpu = CPU::new();
        let program = [
            0x60, 0x02, /* LD V0, 0x02 */
            0x30, 0x04, /* SE V0, 0x02 */
            0x30, 0x02, /* SE V0, 0x03 */
        ];
        cpu.load_program(&program);
        cpu.cycle();
        assert_eq!(cpu.memory.pc, 0x202);
        cpu.cycle();
        assert_eq!(cpu.memory.pc, 0x204);
        cpu.cycle();
        assert_eq!(cpu.memory.pc, 0x208);
    }
    #[test]
    fn test_skip_if_not_equal_byte() {
        let mut cpu = CPU::new();
        let program = [
            0x60, 0x02, /* LD V0, 0x02 */
            0x40, 0x02, 0x40, 0x03, /* SNE V0, 0x03 */
        ];
        cpu.load_program(&program);
        cpu.cycle();
        cpu.cycle();
        assert_eq!(cpu.memory.pc, 0x204);
        cpu.cycle();
        assert_eq!(cpu.memory.pc, 0x208);
    }
    #[test]
    /// Tests the load into register opcode for each register (0x6XNN).
    fn test_load_into_reg() {
        let mut cpu = CPU::new();
        let mut program: Vec<u8> = Vec::new();
        for reg_i in 0..16 {
            program.push(0x60 + reg_i);
            program.push(0x02);
        }
        cpu.load_program(program.as_mut_slice());
        for reg_i in 0..16 {
            cpu.cycle();
            assert_eq!(cpu.memory.v[reg_i], 0x02);
        }
    }
    #[test]
    fn test_or_reg() {
        let mut cpu = CPU::new();
        let  program = [
            0x60, 0x01, /* LD V0, 0x01 */
            0x61, 0x02, /* LD V1, 0x02 */
            0x80, 0x11, /* OR V0, V1 */
        ];
        cpu.load_program(&program);
        for _ in 0..program.len() / 2 {
            cpu.cycle();
        }
        assert_eq!(cpu.memory.v[0], 0x3);

    }

    #[test]
    fn test_skip_if_equals_reg() {
        let mut cpu = CPU::new();
        let program = [
            0x60, 0x01, /* LD V0, 0x01 */
            0x61, 0x01, /* LD V1, 0x01 */
            0x50, 0x10, /* SE V0, V1 */

        ];
        cpu.load_program(&program);
        for _ in 0..program.len() / 2 {
                cpu.cycle();
        }
        assert_eq!(cpu.memory.pc, 0x208);
    }

    #[test]

    fn test_load_reg_into_reg() {
        let mut cpu = CPU::new();
        let program = [
            0x60, 0x01, /* LD V0, 0x01 */
            0x61, 0x02, /* LD V1, 0x02 */
            0x82, 0x00, /* LD V2, V0 */
        ];
        cpu.load_program(&program);
        for _ in 0..program.len() / 2 {
            cpu.cycle();
        }
        assert_eq!(cpu.memory.v[0], 0x1);
        assert_eq!(cpu.memory.v[1], 0x2);
        assert_eq!(cpu.memory.v[2], 0x1);
    }

    #[test]
    fn test_and_reg() {
        let mut cpu = CPU::new();
        let program = [
            0x60, 0x01, /* LD V0, 0x01 */
            0x61, 0x02, /* LD V1, 0x02 */
            0x80, 0x12, /* AND V0, V1 */
            0x62, 0x02, /* LD V2, 0x02 */
            0x82, 0x12, /* AND V2, V1 */

        ];
        cpu.load_program(&program);
        for _ in 0..program.len() / 2 {
            cpu.cycle();
        }
        assert_eq!(cpu.memory.v[0], 0x0);
        assert_eq!(cpu.memory.v[2], 0x2);
    }

}
