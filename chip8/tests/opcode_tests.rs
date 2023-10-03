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

        // Set V0 to 0x02
        cpu.memory.write_reg(0, 0x02);

        // ADD V0, 0x02
        cpu.execute(0x7002);

        // Verify that V0 is now 0x04
        assert_eq!(cpu.memory.v[0], 0x04);
    }

    #[test]
    fn test_skip_if_equal_byte() {
        let mut cpu = CPU::new();

        // Set V0 to 0x02
        cpu.memory.write_reg(0, 0x02);

        // SE V0, 0x02 (Should Skip)
        cpu.execute(0x3002);
        assert_eq!(cpu.memory.pc, 0x204);

        // SE V0, 0x03 (Should Not Skip)
        cpu.execute(0x3003);
        assert_eq!(cpu.memory.pc, 0x206);
    }

    #[test]
    fn test_skip_if_not_equal_byte() {
        let mut cpu = CPU::new();

        // Set V0 to 0x02
        cpu.memory.write_reg(0, 0x02);

        // SNE V0, 0x02 (Should Not Skip)
        cpu.execute(0x4002);
        assert_eq!(cpu.memory.pc, 0x202);

        // SNE V0, 0x03 (Should Skip)
        cpu.execute(0x4003);
        assert_eq!(cpu.memory.pc, 0x206);
    }

    #[test]
    fn test_load_into_reg() {
        let mut cpu = CPU::new();

        // For each register, set its value and then check its value
        for reg_i in 0..16 {
            // Execute the LD Vx, NN opcode
            cpu.execute(0x6000 + (reg_i << 8) + 0x02);

            // Verify the value in the register
            assert_eq!(cpu.memory.v[reg_i as usize], 0x02);
        }
    }

    #[test]
    fn test_or_reg() {
        let mut cpu = CPU::new();

        // Set initial register values
        cpu.memory.write_reg(0, 0x01); // V0 = 0x01
        cpu.memory.write_reg(1, 0x02); // V1 = 0x02

        // Execute OR V0, V1
        cpu.execute(0x8011);

        // Verify that V0 now holds the result of the OR operation
        assert_eq!(cpu.memory.v[0], 0x3);
    }

    #[test]
    fn test_skip_if_equals_reg() {
        let mut cpu = CPU::new();

        // Set initial register values
        cpu.memory.write_reg(0, 0x01); // V0 = 0x01
        cpu.memory.write_reg(1, 0x01); // V1 = 0x01

        // Execute SE V0, V1
        cpu.execute(0x5010);

        // Verify the program counter was incremented by 2 (skipped)
        assert_eq!(cpu.memory.pc, 0x204);
    }

    #[test]
    fn test_load_reg_into_reg() {
        let mut cpu = CPU::new();

        // Set initial register values
        cpu.memory.write_reg(0, 0x01); // V0 = 0x01
        cpu.memory.write_reg(1, 0x02); // V1 = 0x02

        // Execute LD V2, V0
        cpu.execute(0x8200);

        // Validate register values
        assert_eq!(cpu.memory.read_reg(0), 0x1);
        assert_eq!(cpu.memory.read_reg(1), 0x2);
        assert_eq!(cpu.memory.read_reg(2), 0x1);
    }

    #[test]
    fn test_and_reg() {
        let mut cpu = CPU::new();

        // Set initial register values
        cpu.memory.write_reg(0, 0x01); // V0 = 0x01
        cpu.memory.write_reg(1, 0x02); // V1 = 0x02
        cpu.memory.write_reg(2, 0x02); // V2 = 0x02

        // Execute AND V0, V1 and validate the result
        cpu.execute(0x8012);
        assert_eq!(cpu.memory.read_reg(0), 0x0);

        // Execute AND V2, V1 and validate the result
        cpu.execute(0x8212);
        assert_eq!(cpu.memory.read_reg(2), 0x2);
    }

    #[test]

    fn test_xor_vx_vy() {
        let mut cpu = CPU::new();

        // Set some initial values for registers Vx and Vy
        cpu.memory.write_reg(0, 0b10101010); // V1 = 0b10101010
        cpu.memory.write_reg(1, 0b11001100); // V2 = 0b11001100

        // Execute XOR Vx, Vy opcode (assuming 0x8013 represents XOR V1, V2)
        cpu.execute(0x8013);

        // The result should be the bitwise XOR of the initial values
        assert_eq!(cpu.memory.read_reg(0), 0b01100110); // The result should be 0b01100110
    }

    #[test]
    fn test_subtract_vx_vy() {
        let mut cpu = CPU::new();

        cpu.memory.write_reg(0, 6); // V0 = 6
        cpu.memory.write_reg(1, 4); // V1 = 4

        cpu.execute(0x8015);

        assert_eq!(cpu.memory.read_reg(0), 0x2);
        assert_eq!(cpu.memory.read_reg(0xF), 0x0);

        cpu.execute(0x8015);

        assert_eq!(cpu.memory.read_reg(0), 0xFE); //
        assert_eq!(cpu.memory.read_reg(0xF), 0x1);
    }
}
