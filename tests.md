# Tests for the Chip-8 interpreter

## Opcode tests

Here's a table with the tests not implemented, with suggested function names:

| Opcode | Description           | Suggested Test Name         | Implemented |
|--------|-----------------------|-----------------------------|-------------|
| 9xy0   | SNE Vx, Vy            | test_sne_vx_vy              | [x]         |
| Annn   | LD I, addr            | test_ld_i_addr              | [ ]         |
| Bnnn   | JP V0, addr           | test_jp_v0_addr             | [ ]         |
| Cxkk   | RND Vx, byte          | test_rnd_vx_byte            | [ ]         |
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
