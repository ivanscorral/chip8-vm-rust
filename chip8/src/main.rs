pub mod cpu;
pub mod gpu;
pub mod memory;

use std::time::{Instant, Duration};
use cpu::CPU;

fn wait_for_next_cycle(target_clock: u64, last_timestamp: &mut Instant) {
    let target_duration = Duration::from_secs_f64(1.0 / target_clock as f64);
    let mut elapsed = last_timestamp.elapsed();
    while elapsed < target_duration {
        elapsed = last_timestamp.elapsed();
    }
    *last_timestamp = Instant::now();
}

fn main() {
    let mut cpu = CPU::new();
    let program = [
        0xAF,
        0xF0,
        0x60,
        0x02,
        0x61,
        0x0A,
        0x80,
        0x14,
        0xF0,
        0x55,];

    println!("Loading program:");
    cpu.load_program(&program);
    let mut counter = 0;
    println!("Sprites:");
    cpu.print_memory_region(0x0, 0x50, 5);
    'main: loop {
        if !cpu.halt {
            cpu.cycle();
        } else if counter < 0 {
            cpu.reset();
            counter += 1;
        } else {
            println!("Registers:");
            cpu.print_registers();
            println!("Memory:");
            cpu.print_memory_region(0x0F00, 0x1000, 16);
            break 'main;
        }
        let mut last_timestamp = Instant::now();
        wait_for_next_cycle(32, &mut last_timestamp);
    }
}
