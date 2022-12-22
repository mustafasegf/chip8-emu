use chip8::cpu::*;
use chip8::gui::*;
use chip8::terminal::*;
use std::fs::File;
use std::io::Read;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
/// simple chip8 emulator for learning
pub struct Args {
    pub file: String,
    /// use terminal mode
    #[arg(short, long)]
    pub terminal: bool,
}

fn main() {
    let args = Args::parse();

    let mut chip8 = Emu::new();
    let mut rom = File::open(&args.file).expect("Could not open file");
    let mut buffer = Vec::new();
    rom.read_to_end(&mut buffer).expect("Could not read file");
    chip8.load(&buffer);

    if args.terminal {
        init_terminal(&mut chip8);
    } else {
        init_gui(&mut chip8);
    }
}
