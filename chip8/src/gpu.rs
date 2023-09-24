pub const VRAM_WIDTH: usize = 64;
pub const VRAM_HEIGHT: usize = 32;

pub struct GPU {
    video_buffer: [[u8; VRAM_WIDTH]; VRAM_HEIGHT],
}

impl GPU {
    pub fn new() -> GPU {
        GPU {
            video_buffer: [[0x0; 64]; 32],
        }
    }

    pub fn toggle_pixel(&mut self, x: usize, y: usize) {
        if x < VRAM_WIDTH && y < VRAM_HEIGHT {
            self.video_buffer[y][x] ^= 1; /* XOR the pixel at (x, y) */
        } else {
            panic!("PANIC: VRAM index (x: {}, y: {}) out of bounds!", x, y); /* Unsupported VRAM index */
        }
    }

    pub fn reset(&mut self) {
        self.video_buffer = [[0x0; 64]; 32];
    }
}
