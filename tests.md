# Tests for the Chip-8 interpreter

This document outlines the current state of testing for the Chip-8 interpreter. It serves as both a checklist for test coverage and a guide for future test implementation.

## Opcode tests

In order to ensure the accuracy and reliability of the Chip-8 interpreter, it's essential to cover each opcode with appropriate tests. Below is a table showcasing which opcodes currently lack tests, along with suggested function names for each test:

| Opcode | Description           | Suggested Test Name         | Implemented |
|--------|-----------------------|-----------------------------|-------------|
| 9xy0   | SNE Vx, Vy            | test_sne_vx_vy              | [x]         |
| Annn   | LD I, addr            | test_ld_i_addr              | [x]         |
| Bnnn   | JP V0, addr           | test_jp_v0_addr             | [x]         |
| Cxkk   | RND Vx, byte          | test_rnd_vx_byte            | [x]         |
| Dxyn   | DRW Vx, Vy, nibble    | test_drw_vx_vy_nibble       | [ ]         |
| Ex9E   | SKP Vx                | test_skp_vx                 | [ ]         |
| ExA1   | SKNP Vx               | test_sknp_vx                | [ ]         |
| Fx07   | LD Vx, DT             | test_ld_vx_dt               | [ ]         |
| Fx0A   | LD Vx, K              | test_ld_vx_k                | [ ]         |
| Fx15   | LD DT, Vx             | test_ld_dt_vx               | [ ]         |
| Fx18   | LD ST, Vx             | test_ld_st_vx               | [ ]         |
| Fx1E   | ADD I, Vx             | test_add_i_vx               | [ ]         |
| Fx29   | LD F, Vx              | test_ld_f_vx                | [ ]         |
| Fx33   | LD B, Vx              | test_ld_b_vx                | [ ]         |
| Fx55   | LD [I], Vx            | test_ld_i_vx                | [ ]         |
| Fx65   | LD Vx, [I]            | test_ld_vx_i                | [ ]         |

## Mocking Random Byte Generation

For opcodes like `RND Vx, byte` , which involve random byte generation, the actual randomness can be replaced with a predetermined value during testing. This ensures the opcode's behavior can be verified without the unpredictability of actual random values.

Here's how mocking is implemented for random byte generation:

```rust
pub struct CPU {
    ...
    mock_rand: (bool, u8),
}

impl CPU {
    ...
    pub fn mock_random_byte(&mut self, val: u8) {
        self.mock_rand = (true, val);
    }
}
```

When `mock_random_byte` is called, the subsequent random byte generation opcode will use the provided mock value instead of generating an actual random byte. This approach allows for precise testing of the opcode's behavior.

It's essential to ensure that tests are implemented both with and without mocking to verify the opcode's behavior in all scenarios.

