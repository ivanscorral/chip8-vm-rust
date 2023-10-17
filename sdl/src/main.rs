
use chip8::cpu;
use sdl::binaryreader::BinaryFileReader;

fn main() {
    let mut sdl = sdl::SDL::new();
    let mut cpu = cpu::CPU::new();

    let program_path = "roms/stars_snaydenov.bin";

    let program = match read_chip8_rom(program_path) {
        Ok(data) => data,
        Err(e) => {
            println!("Error loading CHIP-8 program: {}", e);
            return;
        }
    };

    cpu.load_program(&program);

    sdl.run(&mut cpu)
}


fn read_chip8_rom(path: &str) -> std::io::Result<Vec<u8>> {
    BinaryFileReader::read(path)
}
