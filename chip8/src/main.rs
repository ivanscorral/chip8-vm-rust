use std::thread;
use std::time::{Instant, Duration};

use cpu::CPU;

mod cpu;
mod memory;

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
    'main: loop {

        let mut last_timestamp = Instant::now();
        wait_for_next_cycle(5, &mut last_timestamp);
    }
}
