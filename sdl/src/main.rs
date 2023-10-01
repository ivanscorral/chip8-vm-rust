extern crate sdl;

use chip8::cpu;
fn main() {
    let mut sdl = sdl::SDL::new();
    let mut cpu = cpu::CPU::new();

    sdl.run(&mut cpu)
}
