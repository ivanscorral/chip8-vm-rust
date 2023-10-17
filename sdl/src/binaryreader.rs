use std::{fs::File, path::Path};
use std::io::{self, Read};

pub struct BinaryFileReader;


impl BinaryFileReader {
    pub fn read<P: AsRef<Path>>(path: P) -> io::Result<Vec<u8>> {
        let mut file = File::open(path)?;

        let mut buffer = Vec::new();

        file.read_to_end(&mut buffer)?;
        Ok(buffer)
    }
}
