extern crate rand;
extern crate sdl2;
mod drivers;
mod font;
mod processor;

use std::env;
use std::thread;
use std::time::Duration;

use crate::drivers::{AudioDriver, CartridgeDriver, DisplayDriver, InputDriver};
use crate::processor::Processor;

pub const CHIP8_WIDTH: usize = 64;
pub const CHIP8_HEIGHT: usize = 32;
pub const CHIP8_RAM: usize = 0x1000;
pub const CHIP8_ROM_START_OFFSET: usize = 0x200;

const SLEEP_DURATION: Duration = Duration::from_millis(1);

fn main() {
    let sdl_context = sdl2::init().unwrap();

    let cartridge_filename = env::args()
        .nth(1)
        .expect("Error: no cartrige path as a first argument of program");

    let cartridge_driver = CartridgeDriver::new(&cartridge_filename);
    let audio_driver = AudioDriver::new(&sdl_context);
    let mut display_driver = DisplayDriver::new(&sdl_context);
    let mut input_driver = InputDriver::new(&sdl_context);
    let mut processor = Processor::new();

    processor.load(&cartridge_driver.rom);

    while let Ok(keypad) = input_driver.poll() {
        let output = processor.tick(keypad);

        if output.vram_changed {
            display_driver.draw(output.vram);
        }

        if output.beep {
            audio_driver.start_beep();
        } else {
            audio_driver.stop_beep();
        }

        thread::sleep(SLEEP_DURATION);
    }
}
