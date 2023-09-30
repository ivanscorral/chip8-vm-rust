#[cfg(test)]

mod tests {
 use chip8::cpu::CPU;

 #[test]
 fn test_jump() {
    let mut cpu = CPU::new();
    let program = [
        0x12, 0x02, /* JP 0x202 */
        0x00, 0x00, /* NOP */
    ];
    cpu.load_program(&program);
    cpu.cycle();
    assert_eq!(cpu.memory.pc, 0x202);
 }

 #[test]
 fn test_call() {
    let mut cpu = CPU::new();
    let program = [
        0x22, 0x02, /* CALL 0x202 */
        0x00, 0x00, /* NOP */
    ];
    cpu.load_program(&program);
    cpu.cycle();
    assert_eq!(cpu.memory.pc, 0x202);
    assert_eq!(cpu.memory.sp, 1);
    assert_eq!(cpu.memory.stack[0], 0x200);
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
        0x40, 0x02,
        0x40, 0x03, /* SNE V0, 0x03 */
    ];
    cpu.load_program(&program);
    cpu.cycle();
    cpu.cycle();
    assert_eq!(cpu.memory.pc, 0x204);
    cpu.cycle();
    assert_eq!(cpu.memory.pc, 0x208);
 }
}
