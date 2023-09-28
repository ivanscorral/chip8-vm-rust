pub const VRAM_WIDTH: usize = 64;
pub const VRAM_HEIGHT: usize = 32;

pub type Coordinate = (usize, usize);

pub struct GPU {
    video_buffer: [[u8; VRAM_WIDTH]; VRAM_HEIGHT],
}

impl GPU {
    pub fn new() -> GPU {
        GPU {
            video_buffer: [[0x0; 64]; 32],
        }
    }

    fn xor_pixel(&mut self, coords: Coordinate, value: u8) -> u8 {
        let collision = self.video_buffer[coords.1][coords.0] & value;
        self.video_buffer[coords.1][coords.0] ^= value;
        collision
    }

    /// Draws a single row of a sprite onto the screen buffer.
    ///
    /// # Arguments
    ///
    /// * `coords` - The (x, y) coordinates where the row should start.
    /// * `row` - The row data as an 8-bit unsigned integer, where each bit represents a pixel (1 is on, 0 is off).
    ///
    /// # Returns
    ///
    /// * An 8-bit unsigned integer as a collision flag. A value of 1 indicates that drawing this sprite row resulted in a pixel being flipped from set (1) to unset (0).
    ///
    /// # Screen Wrapping
    ///
    /// The function features screen wrapping to stay true to typical Chip-8 behavior. This means if a sprite
    /// is drawn near the edge of the screen, it will wrap around to the opposite edge.
    ///
    /// ## Example
    ///
    /// If you try to draw a sprite at (63, 31) on a 64x32 screen, and the sprite is 8 pixels wide:
    /// * The first pixel will be drawn at (63, 31)
    /// * The second pixel will wrap around and be drawn at (0, 31)
    /// * The third pixel will be drawn at (1, 31)
    /// * And so on, until the eighth pixel is drawn at (6, 31)
    ///

    fn draw_sprite_row(&mut self, coords: Coordinate, row: u8) -> u8 {
        let mut collision = 0;
        let wrapped_y = coords.1 % VRAM_HEIGHT;
        for offset_col in 0..8 {
            let wrapped_x = (coords.0 + offset_col) % VRAM_WIDTH;
            let pixel = (row >> (7 - offset_col)) & 0x1; /* Get the offset bit from the sprite */
            collision |= self.xor_pixel((wrapped_x, wrapped_y), pixel);
        }
        collision
    }
    pub fn reset(&mut self) {
        self.video_buffer = [[0x0; 64]; 32];
    }

    pub(crate) fn draw_sprite(&mut self, coords: Coordinate, sprite: Vec<u8>) -> u8 {
        let mut collision = 0;
        for (offset_row, row) in sprite.iter().enumerate() {
            collision |= self.draw_sprite_row((coords.0, coords.1 + offset_row), *row);
        }
        collision
    }
}
