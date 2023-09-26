use std::time::{Instant, Duration};

use cpu::CPU;

mod cpu;
mod memory;
mod gpu;

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
    cpu.load_program(&program);
    let mut counter = 0;

    'main: loop {
        if !cpu.halt {
            cpu.cycle();
        } else if counter < 1 {
            cpu.reset();
            counter += 1;
        } else {
            cpu.print_registers();
            cpu.print_memory_region(0xF00, 0xFFF, 16);
            break 'main;
        }
        let mut last_timestamp = Instant::now();
        wait_for_next_cycle(4, &mut last_timestamp);
    }
}
