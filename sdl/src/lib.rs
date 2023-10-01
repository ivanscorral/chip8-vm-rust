use sdl2::{
    event::Event,
    keyboard::{Keycode, Scancode},
    pixels::Color,
};
use std::time::Duration;

use chip8::{cpu, gpu};

/// Defines the refresh rate of the screen in Hz.
pub const REFRESH_RATE: u32 = 60;
/// Defines the window width in pixels.
pub const WINDOW_WIDTH: usize = 640;
/// Defines the window height in pixels.
pub const WINDOW_HEIGHT: usize = 320;
/// Defines the time per frame in nanoseconds.
const TIME_PER_FRAME: Duration = Duration::new(0, 1_000_000_000u32 / REFRESH_RATE);
/// The CPU clock speed in Hz.
const CPU_CLOCK_HZ: u32 = 500;
/// The number of CPU cycles per frame.
const CPU_CYCLES_PER_FRAME: u32 = CPU_CLOCK_HZ / REFRESH_RATE;

/// The SDL context.
pub struct SDL {
    /// The SDL context.
    context: sdl2::Sdl,
    /// The canvas to render to.
    canvas: sdl2::render::Canvas<sdl2::video::Window>,
}

impl Drop for SDL {
    /// Clean up SDL.
    fn drop(&mut self) {
        // TODO: Clean up SDL
    }
}

impl SDL {
    ///
    /// # Examples
    ///
    /// ```rust
    /// use sdl::SDL;
    ///
    /// let sdl = SDL::new();
    /// ```
    pub fn new() -> Self {
        let context = sdl2::init().unwrap();
        let video_subsystem = context.video().unwrap();

        let window = video_subsystem
            .window("Chip-8-VM-Rust", WINDOW_WIDTH as u32, WINDOW_HEIGHT as u32)
            .position_centered()
            .build()
            .unwrap();

        let canvas = window.into_canvas().build().unwrap();
        println!(
            "CPU CLOCK: {}, REFRESH RATE: {}, CYCLES PER FRAME: {}",
            CPU_CLOCK_HZ, REFRESH_RATE, CPU_CYCLES_PER_FRAME
        );
        SDL { context, canvas }
    }

    /// Runs the main loop of the SDL application.
    ///
    /// This function drives the primary loop of the SDL application, handling events,
    /// executing CPU cycles, and rendering the GPU buffer onto the screen.
    ///
    /// # Arguments
    ///
    /// * `cpu` - A mutable reference to the `CPU` instance from the `chip8` crate.
    ///
    /// # Behavior
    ///
    /// - The function begins by initializing an `event_pump` to capture and handle SDL events.
    /// - Inside the main `'running` loop, the function listens for `Quit` events (like closing the window)
    ///   or the `Escape` key being pressed. Either of these will break out of the main loop.
    /// - For each frame, the function runs the CPU for a number of cycles defined by `CPU_CYCLES_PER_FRAME`.
    ///   However, if the CPU's `halt` flag is set, it will break out of the cycle loop early.
    /// - After the CPU cycles are executed, the function retrieves the GPU instance from the CPU and renders
    ///   its buffer onto the screen.
    /// - Finally, the function sleeps for a duration defined by `TIME_PER_FRAME` before starting the next iteration
    ///   of the main loop.
    ///
    /// # Panics
    ///
    /// This function will panic if the event pump fails to initialize.
    pub fn run(&mut self, cpu: &mut chip8::cpu::CPU) {
        let mut event_pump = self.context.event_pump().unwrap();
        'running: loop {
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => break 'running,
                    _ => {
                        self.handle_keyboard_input(cpu, event);
                    }
                }
            }

            // Poll keyboard events and send them to the CPU.

            // Run the CPU for one frame.
            'cpu_loop: for _ in 0..CPU_CYCLES_PER_FRAME {
                if !cpu.halt {
                    cpu.cycle();
                } else {
                    break 'cpu_loop;
                }
            }

            {
                let gpu = cpu.get_gpu();
                // Render the GPU buffer onto the screen.
                self.canvas.set_draw_color(Color::RGB(0, 0, 0));
                self.canvas.clear();
                self.render_gpu_buffer(gpu);
                self.canvas.present();
            }
            ::std::thread::sleep(TIME_PER_FRAME);
        }
    }

    /// Renders the GPU buffer to the screen.
    ///
    /// # Arguments
    ///
    /// * `gpu` - The GPU buffer to render.
    fn render_gpu_buffer(&mut self, gpu: &mut gpu::GPU) {
        let upscale_ratio: usize = WINDOW_WIDTH / gpu::VRAM_WIDTH;

        for y in 0..gpu::VRAM_HEIGHT {
            for x in 0..gpu::VRAM_WIDTH {
                if gpu.video_buffer[y][x] != 0 {
                    // Pixel is white
                    self.canvas.set_draw_color(Color::RGB(255, 255, 255));
                } else {
                    // Pixel is black
                    self.canvas.set_draw_color(Color::RGB(0, 0, 0));
                }
                self.canvas
                    .fill_rect(sdl2::rect::Rect::new(
                        (x * upscale_ratio) as i32,
                        (y * upscale_ratio) as i32,
                        upscale_ratio as u32,
                        upscale_ratio as u32,
                    ))
                    .unwrap();
            }
        }
    }

    /// Handles keyboard input.
    ///
    /// The keyboard mapping layout is as follows:
    ///
    ///
    /// 1 2 3 4
    /// Q W E R
    /// A S D F
    /// Z X C V
    ///
    ///
    /// When a key is pressed, the corresponding bit in the `cpu`'s `key_pressed` register is set.
    /// When a key is released, the corresponding bit in the `cpu`'s `key_released` register is set.
    fn handle_keyboard_input(&mut self, cpu: &mut cpu::CPU, event: Event) {
        let key_mappings: [(sdl2::keyboard::Scancode, usize); 16] = [
            (Scancode::Num1, 0x1),
            (Scancode::Num2, 0x2),
            (Scancode::Num3, 0x3),
            (Scancode::Num4, 0xC),
            (Scancode::Q, 0x4),
            (Scancode::W, 0x5),
            (Scancode::E, 0x6),
            (Scancode::R, 0xD),
            (Scancode::A, 0x7),
            (Scancode::S, 0x8),
            (Scancode::D, 0x9),
            (Scancode::F, 0xE),
            (Scancode::Z, 0xA),
            (Scancode::X, 0x0),
            (Scancode::C, 0xB),
            (Scancode::V, 0xF),
        ];

        match event {
            Event::KeyDown {
                scancode: Some(scancode),
                ..
            } => {
                if let Some(&(_, index)) = key_mappings.iter().find(|&&(key, _)| key == scancode) {
                    cpu.key_pressed(index);
                } else {
                    match scancode {
                        Scancode::Space => cpu.halt = !cpu.halt,
                        Scancode::M => cpu.cycle(),
                        Scancode::P => cpu.print_registers(),
                        Scancode::Return => cpu.reset(),
                        _ => {}
                    }
                }
            }
            Event::KeyUp {
                scancode: Some(scancode),
                ..
            } => {
                if let Some(&(_, index)) = key_mappings.iter().find(|&&(key, _)| key == scancode) {
                    cpu.key_released(index);
                }
            }
            _ => {}
        }
    }
}
