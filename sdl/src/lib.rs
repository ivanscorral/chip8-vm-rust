extern crate sdl2;

use sdl2::{event::Event, keyboard::{Keycode, Scancode}, pixels::Color};
use std::time::Duration;

use chip8::{cpu, gpu};



// Defines the refresh rate of the screen in Hz.
pub const REFRESH_RATE: u32 = 60;
// Defines the window width in pixels.
pub const WINDOW_WIDTH: usize = 640;
// Defines the window height in pixels.
pub const WINDOW_HEIGHT: usize = 320;
// Defines the time per frame in nanoseconds.
const TIME_PER_FRAME: Duration = Duration::new(0, 1_000_000_000u32 / REFRESH_RATE);

const CPU_CLOCK_HZ: u32 = 500;
const CPU_CYCLES_PER_FRAME: u32 = CPU_CLOCK_HZ / REFRESH_RATE;

pub struct SDL {
    context: sdl2::Sdl,
    canvas: sdl2::render::Canvas<sdl2::video::Window>,
}

impl Drop for SDL {
    fn drop(&mut self) {
        // TODO: Clean up SDL
    }
}

impl SDL {
    /// Initializes a new instance of the SDL struct.
    ///
    /// # Examples
    ///
    /// ```
    /// use sdl::SDL;
    /// let sdl = SDL::new();
    /// ```
    ///
    /// # Returns
    ///
    /// A new instance of the SDL struct.
    ///
    /// # Panics
    ///
    /// This function will panic if SDL2 fails to initialize or if the window fails to build.
    ///
    /// # Arguments
    ///
    /// None.
    ///
    /// # Remarks
    ///
    /// This function initializes SDL2, creates a window, and sets up a canvas for rendering.
    ///
    /// The `run` method can be called on the returned `SDL` instance to start the main loop.
    ///
    /// The `run` method will block the current thread until the user presses the Escape key or the window is closed.
    ///
    /// The `TODO` comments in the `run` method indicate areas where additional functionality can be added.
    ///
    /// The `TIME_PER_FRAME` constant is defined in the `chip8::constants` module and represents the time per frame in milliseconds
    pub fn new() -> Self {
        let context = sdl2::init().unwrap();
        let video_subsystem = context.video().unwrap();

        let window = video_subsystem
            .window("Chip-8-VM-Rust", WINDOW_WIDTH as u32, WINDOW_HEIGHT as u32)
            .position_centered()
            .build()
            .unwrap();

        let canvas = window.into_canvas().build().unwrap();
        println!("CPU CLOCK: {}, REFRESH RATE: {}, CYCLES PER FRAME: {}", CPU_CLOCK_HZ, REFRESH_RATE, CPU_CYCLES_PER_FRAME);
        SDL { context, canvas }
    }

    /// Runs the main loop of the SDL application.
    ///
    /// # Examples
    ///
    /// ```
    /// use sdl::SDL;
    /// let mut sdl = SDL::new();
    /// let mut cpu = chip8::cpu::CPU::new();
    /// let mut gpu = chip8::gpu::GPU::new();
    /// sdl.run(&mut cpu, &mut gpu);
    /// ```
    ///
    /// # Returns
    ///
    /// None.
    ///
    /// # Panics
    ///
    /// This function will panic if the event pump fails to initialize.
    ///
    /// # Arguments
    ///
    /// * `cpu` - A mutable reference to the CPU instance.
    /// * `gpu` - A mutable reference to the GPU instance.
    ///
    /// # Remarks
    ///
    /// This function runs the main loop of the SDL application.
    ///
    /// The `cpu` and `gpu` instances are passed in as mutable references so that they can be updated during the main loop.
    ///
    /// The main loop will block the current thread until the user presses the Escape key or the window is closed.
    ///
    /// The `TODO` comments in the method indicate areas where additional functionality can be added.
    ///
    /// The `TIME_PER_FRAME` constant is defined in the `chip8::constants` module and represents the time per frame in milliseconds.
    pub fn run(&mut self, cpu: &mut chip8::cpu::CPU) {
        let mut event_pump = self.context.event_pump().unwrap();
        'running: loop {

            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit {..} |
                    Event::KeyDown {
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

    /*
       Keyboard mapping layout:
        1 2 3 4
        Q W E R
        A S D F
        Z X C V
    */
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
                Event::KeyDown { scancode: Some(scancode), .. } => {
                    if let Some(&(_, index)) = key_mappings.iter().find(|&&(key, _)| key == scancode) {
                        cpu.key_pressed(index);
                    }
                }
                Event::KeyUp { scancode: Some(scancode), .. } => {
                    if let Some(&(_, index)) = key_mappings.iter().find(|&&(key, _)| key == scancode) {
                        cpu.key_released(index);
                    }
                }
                _ => {}
        }

    }

}
