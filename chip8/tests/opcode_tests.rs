#[cfg(test)]

pub mod tests {

    use chip8::{cpu::CPU, gpu};
    #[test]
    fn test_jump_n() {
        let mut cpu = CPU::new();
        cpu.memory.pc = 0x200;
        cpu.execute(0x1234); // Jump to address 0x234
        assert_eq!(cpu.memory.pc, 0x234);
    }
    #[test]
    fn test_call_n() {
        let mut cpu = CPU::new();

        cpu.execute(0x2234); // Call address 0x234

        // Check if the return address (next instruction) is pushed onto the stack
        assert_eq!(cpu.memory.stack[(cpu.memory.sp - 1) as usize], 0x202);

        // Check if the PC is set to the address from the opcode
        assert_eq!(cpu.memory.pc, 0x234);
    }

    #[test]
    fn test_add_byte_vx() {
        let mut cpu = CPU::new();

        // Set V0 to 0x02
        cpu.memory.write_reg(0, 0x02);

        // ADD V0, 0x02
        cpu.execute(0x7002);

        // Verify that V0 is now 0x04
        assert_eq!(cpu.memory.v[0], 0x04);
    }

    #[test]
    fn test_skip_vx_equals_byte() {
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
    fn test_skip_vx_not_equal_byte() {
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
    fn test_load_byte_vx() {
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
    fn test_or_vx_vy() {
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
    fn test_skip_vx_eq_vy() {
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
        assert_eq!(cpu.memory.read_reg(0xF), 0x1);

        cpu.execute(0x8015);

        assert_eq!(cpu.memory.read_reg(0), 0xFE); //
        assert_eq!(cpu.memory.read_reg(0xF), 0x0);
    }

    #[test]

    fn test_return() {
        let mut cpu = CPU::new();

        cpu.memory.push_stack(0x300);
        cpu.execute(0x00EE);

        assert_eq!(cpu.memory.pc, 0x300);
        assert_eq!(cpu.memory.sp, 0x0);
    }

    #[test]
    fn test_clear_screen() {
        let mut cpu = CPU::new();

        cpu.get_gpu().video_buffer[5][5] = 1;

        cpu.execute(0x00E0);

        for x in 0..gpu::VRAM_WIDTH {
            for y in 0..gpu::VRAM_HEIGHT {
                assert_eq!(cpu.get_gpu().video_buffer[y][x], 0);
            }
        }
    }

    #[test]
    fn test_add_vx_vy() {
        let mut cpu = CPU::new();

        cpu.memory.write_reg(0, 0x1); // V0 = 1
        cpu.memory.write_reg(1, 0x2); // V1 = 2

        cpu.execute(0x8014);

        assert_eq!(cpu.memory.read_reg(0), 0x3);
    }

    #[test]
    fn test_shift_right() {
        let mut cpu = CPU::new();

        cpu.memory.write_reg(0, 0b00001100); // V0 = 0b10101010

        cpu.execute(0x8006);

        assert_eq!(cpu.memory.read_reg(0), 0b00000110);
        assert_eq!(cpu.memory.read_reg(0xF), 0b0);
    }

    #[test]
    fn test_shift_left() {
        let mut cpu = CPU::new();

        cpu.memory.write_reg(0, 0b00001100); // V0 = 0b10101010

        cpu.execute(0x800E);

        assert_eq!(cpu.memory.read_reg(0), 0b000011000);
        assert_eq!(cpu.memory.read_reg(0xF), 0b0);
    }

    #[test]

    fn test_subtract_vy_vx() {
        let mut cpu = CPU::new();

        // Normal Case: V1 - V0 = 2 - 1
        cpu.memory.write_reg(0, 1); // V0 = 1
        cpu.memory.write_reg(1, 2); // V1 = 2
        cpu.execute(0x8017);
        assert_eq!(cpu.memory.read_reg(0), 1); // 2 - 1 = 1
        assert_eq!(cpu.memory.read_reg(0xF), 1); // No borrow

        // Normal Case: Underflow, V1 - V0 = 1 - 2
        cpu.memory.write_reg(0, 2); // V0 = 2
        cpu.memory.write_reg(1, 1); // V1 = 1
        cpu.execute(0x8017);
        assert_eq!(cpu.memory.read_reg(0), 255); // 1 - 2 = -1, wraps to 255 in u8
        assert_eq!(cpu.memory.read_reg(0xF), 0); // Borrow occurred

        // Edge Case: Subtracting identical values, V1 - V0 = 1 - 1
        cpu.memory.write_reg(0, 1); // V0 = 1
        cpu.memory.write_reg(1, 1); // V1 = 1
        cpu.execute(0x8017);
        assert_eq!(cpu.memory.read_reg(0), 0);
        assert_eq!(cpu.memory.read_reg(0xF), 1); // No borrow

        // Edge Case: Subtracting from 0, V1 - V0 = 0 - 1
        cpu.memory.write_reg(0, 1); // V0 = 1
        cpu.memory.write_reg(1, 0); // V1 = 0
        cpu.execute(0x8017);
        assert_eq!(cpu.memory.read_reg(0), 255);
        assert_eq!(cpu.memory.read_reg(0xF), 0); // Borrow occurred

        // Edge Case: Subtracting max u8 value, V1 - V0 = 255 - 1
        cpu.memory.write_reg(0, 1); // V0 = 1
        cpu.memory.write_reg(1, 255); // V1 = 255
        cpu.execute(0x8017);
        assert_eq!(cpu.memory.read_reg(0), 254);
        assert_eq!(cpu.memory.read_reg(0xF), 1); // No borrow
    }

    #[test]
    fn test_sne_vx_vy() {
        let mut cpu = CPU::new();

        cpu.memory.write_reg(0, 1);
        cpu.memory.write_reg(1, 2);

        cpu.execute(0x9010);

        assert_eq!(cpu.memory.pc, 0x204);
    }

    #[test]
    fn test_ld_i_addr() {
        let mut cpu = CPU::new();

        cpu.execute(0xA145);

        assert_eq!(cpu.memory.i, 0x145);
    }

    #[test]
    fn test_jp_v0_addr() {
        let mut cpu = CPU::new();

        cpu.memory.write_reg(0, 0x10);

        cpu.execute(0xB100);

        assert_eq!(cpu.memory.pc, 0x110);
    }

    #[test]
    fn test_rnd_vx_byte() {
        let mut cpu = CPU::new();

        // Mock the random number generation to always return 0xAB

        cpu.mock_random_byte(0xAB);

        // Execute the RND Vx, byte opcode with kk = 0xCD
        cpu.execute(0xC0CD);

        // 0xAB AND 0xCD = 0x89
        assert_eq!(cpu.memory.read_reg(0), 0x89);
    }

    #[test]
    fn test_ld_b_vx() {
        let mut cpu = CPU::new();

        // Setting Vx register to a number. Let's say 123
        cpu.memory.write_reg(5, 105); // V5 = 123

        // Setting I register to a location in memory, say 0x400
        cpu.memory.i = 0x400;

        // Execute the opcode for `LD B, V5`
        cpu.execute(0xF533);

        assert_eq!(cpu.memory.load(0x400), 1); // 1
        assert_eq!(cpu.memory.load(0x401), 0); // 2
        assert_eq!(cpu.memory.load(0x402), 5); // 3
    }

    /// Ex9E - SKP Vx
    /// Skip next instruction if key with the value of Vx is pressed.
    #[test]
    fn test_skp_vx() {
        let mut cpu = CPU::new();

        // 1. Set V5 register to hold the value of key 5
        cpu.memory.write_reg(5, 5);

        // 2. Simulate pressing key 5
        cpu.key_pressed(5);

        // 3. Execute the SKP V5 opcode
        cpu.execute(0xE59E);

        // After executing the instruction, if the key corresponding to V5 (which is key 5) is pressed,
        // the CPU should skip the next instruction. Thus, the program counter should be incremented by 4.
        assert_eq!(cpu.memory.pc, 0x204);

        // Reset CPU and test for a key that is not pressed
        cpu.reset();

        // 1. Set V5 register to hold the value of key 6
        cpu.memory.write_reg(5, 6);

        // 2. We don't simulate pressing key 6

        // 3. Execute the SKP V5 opcode
        cpu.execute(0xE59E);

        // In this case, the key corresponding to V5 (which is key 6) is not pressed.
        // Thus, the CPU shouldn't skip the next instruction. So, the program counter should only be incremented by 2.
        assert_eq!(cpu.memory.pc, 0x202);
    }

    #[test]
    fn test_snkp_vx() {
        let mut cpu = CPU::new();

        // 1. Set V5 register to hold the value of key 5
        cpu.memory.write_reg(5, 5);

        // 2. Simulate pressing key 5
        cpu.key_pressed(5);

        // 3. Execute the SNKP V5 opcode
        cpu.execute(0xE5A1);

        // In this case, the key corresponding to V5 (which is key 5) is pressed.
        // Thus, the CPU shouldn't skip the next instruction. So, the program counter should only be incremented by 2.
        assert_eq!(cpu.memory.pc, 0x202);

        // Reset CPU and test for a key that is not pressed
        cpu.reset();

        // 1. Set V5 register to hold the value of key 6
        cpu.memory.write_reg(5, 6);

        // 2. We don't simulate pressing key 6

        // 3. Execute the SNKP V5 opcode
        cpu.execute(0xE5A1);

        // After executing the instruction, if the key corresponding to V5 (which is key 6) is not pressed,
        // the CPU should skip the next instruction. Thus, the program counter should be incremented by 4.
        assert_eq!(cpu.memory.pc, 0x204);
    }

    #[test]
    fn test_ld_vx_dt() {
        let mut cpu = CPU::new();

        // 1. Set delay timer to a known value
        cpu.memory.dt = 0x5A;

        // 2. Execute the LD V5, DT opcode
        cpu.execute(0xF507);

        // 3. Assert that V5 now holds the value of the delay timer
        assert_eq!(cpu.memory.read_reg(5), 0x5A);
    }

    #[test]
    fn test_ld_vx_k() {
        let mut cpu = CPU::new();

        // Simulate that key 7 is pressed
        cpu.key_pressed(7);

        // Execute the LD V5, K opcode
        cpu.execute(0xF50A);

        // Assert that V5 now holds the value of the key that was pressed
        assert_eq!(cpu.memory.read_reg(5), 7);
    }

    #[test]
    fn test_ld_st_vx() {
        let mut cpu = CPU::new();

        // Set V5 to a known value
        cpu.memory.write_reg(5, 0x5A);

        // Execute the LD ST, V5 opcode
        cpu.execute(0xF518);

        // Assert that the sound timer now holds the value in V5
        assert_eq!(cpu.memory.st, 0x5A);
    }

    #[test]
    fn test_ld_dt_vx() {
        let mut cpu = CPU::new();

        // Set V5 to a known value
        cpu.memory.write_reg(5, 0x5A);

        // Execute the LD DT, V5 opcode
        cpu.execute(0xF515);

        // Assert that the delay timer now holds the value in V5
        assert_eq!(cpu.memory.dt, 0x5A);
    }
}
