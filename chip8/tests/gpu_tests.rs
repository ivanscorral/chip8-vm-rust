#[cfg(test)]

// This is gpu_tests.rs located in chip8/tests directory
use chip8::gpu::GPU;
use chip8::gpu::VRAM_WIDTH;
use chip8::gpu::VRAM_HEIGHT;

#[test]
fn test_gpu_initialization() {
    let gpu = GPU::new();
    for y in 0..VRAM_HEIGHT {
        for x in 0..VRAM_WIDTH {
            assert_eq!(gpu.video_buffer[y][x], 0);
        }
    }
}

#[test]
fn test_xor_pixel() {
    let mut gpu = GPU::new();

    // Set a pixel to 1 and XOR with 1, expect a 0 and a collision
    gpu.video_buffer[10][10] = 1;
    let collision = gpu.xor_pixel((10, 10), 1);
    assert_eq!(gpu.video_buffer[10][10], 0);
    assert_eq!(collision, 1);

    // Set a pixel to 0 and XOR with 1, expect a 1 and no collision
    gpu.video_buffer[10][10] = 0;
    let collision = gpu.xor_pixel((10, 10), 1);
    assert_eq!(gpu.video_buffer[10][10], 1);
    assert_eq!(collision, 0);
}

#[test]
fn test_draw_row() {
    let mut gpu = GPU::new();

    // Drawing a row with all pixels on
    let collision = gpu.draw_sprite_row((0, 0), 0xFF);
    for x in 0..8 {
        assert_eq!(gpu.video_buffer[0][x], 1);
    }
    assert_eq!(collision, 0); // No collision expected as screen was initially blank

    // Drawing another row with alternating pixels
    let collision = gpu.draw_sprite_row((0, 1), 0xAA);
    for x in (0..8).step_by(2) {
        assert_eq!(gpu.video_buffer[1][x], 1);
        assert_eq!(gpu.video_buffer[1][x + 1], 0);
    }
    assert_eq!(collision, 0); // No collision expected as no pixels overlap
}


#[test]
fn test_clear_screen() {
    let mut gpu = GPU::new();
    // Set some pixels in the video buffer
    for y in 0..VRAM_HEIGHT {
        for x in 0..VRAM_WIDTH {
            gpu.video_buffer[y][x] = 1;
        }
    }
    gpu.clear();
    for y in 0..VRAM_HEIGHT {
        for x in 0..VRAM_WIDTH {
            assert_eq!(gpu.video_buffer[y][x], 0);
        }
    }
}

#[test]
fn test_draw_after_clear() {
    let mut gpu = GPU::new();
    gpu.clear();
    gpu.draw_sprite((0, 0), vec![0xFF, 0xAA]);
    for x in 0..8 {
        assert_eq!(gpu.video_buffer[0][x], 1);
        if x % 2 == 0 {
            assert_eq!(gpu.video_buffer[1][x], 1);
        } else {
            assert_eq!(gpu.video_buffer[1][x], 0);
        }
    }
}

#[test]
fn test_screen_wrapping() {
    let mut gpu = GPU::new();
    gpu.clear();
    let last_col = VRAM_WIDTH - 1;
    let last_row = VRAM_HEIGHT - 1;
    // Drawing a sprite that should wrap around the screen
    gpu.draw_sprite((last_col, last_row), vec![0x80, 0x01]);
    assert_eq!(gpu.video_buffer[last_row][last_col], 1); // Bottom-right pixel
    assert_eq!(gpu.video_buffer[0][0], 1);               // Top-left pixel (due to wrapping)
    assert_eq!(gpu.video_buffer[last_row][0], 1);        // Wrapped to the beginning of the same row
    assert_eq!(gpu.video_buffer[0][last_col], 0);        // This shouldn't be affected, should remain 0
}
