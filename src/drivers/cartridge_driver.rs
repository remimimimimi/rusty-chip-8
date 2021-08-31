use std::fs::File;
use std::io::prelude::*;

use crate::{CHIP8_RAM, CHIP8_ROM_START_OFFSET};

const CHIP8_ROM: usize = CHIP8_RAM - CHIP8_ROM_START_OFFSET;

pub struct CartridgeDriver {
    pub rom: [u8; 3584],
    pub size: usize,
}

impl CartridgeDriver {
    pub fn new(filename: &str) -> Self {
        let mut f = File::open(filename).expect("Error: file not found");
        let mut buffer = [0u8; CHIP8_ROM];

        let bytes_read = if let Ok(bytes_read) = f.read(&mut buffer) {
            bytes_read
        } else {
            0
        };

        CartridgeDriver {
            rom: buffer,
            size: bytes_read,
        }
    }
}
