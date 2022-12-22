use sdl2::audio::{AudioCallback, AudioSpecDesired};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

use crate::cpu::*;

const SCALE: u32 = 20;
const WINDOW_WIDTH: u32 = (SCREEN_WIDTH as u32) * SCALE;
const WINDOW_HEIGHT: u32 = (SCREEN_HEIGHT as u32) * SCALE;
const TICKS_PER_FRAME: usize = 1;

struct SquareWave {
    phase_inc: f32,
    phase: f32,
    volume: f32,
}

impl AudioCallback for SquareWave {
    type Channel = f32;

    fn callback(&mut self, out: &mut [f32]) {
        // Generate a square wave
        for x in out.iter_mut() {
            *x = if self.phase <= 0.5 {
                self.volume
            } else {
                -self.volume
            };
            self.phase = (self.phase + self.phase_inc) % 1.0;
        }
    }
}

pub fn draw_screen(emu: &Emu, canvas: &mut Canvas<Window>) {
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();

    let screen = emu.get_display();
    canvas.set_draw_color(Color::RGB(255, 255, 255));

    for (i, pixel) in screen.iter().enumerate() {
        if *pixel {
            let x = (i % SCREEN_WIDTH) as u32;
            let y = (i / SCREEN_WIDTH) as u32;

            let rect = Rect::new((x * SCALE) as i32, (y * SCALE) as i32, SCALE, SCALE);
            canvas.fill_rect(rect).unwrap();
        }
    }

    // let ttf_ctx = sdl2::ttf::init().unwrap();
    // let font = ttf_ctx
    //     .load_font("assets/PressStart2P-Regular.ttf", SCALE as u16 / 2)
    //     .unwrap();
    //
    // let offset = emu.get_offset() as usize;
    // let text = emu.get_ram()[RAM_OFFSET * offset..RAM_OFFSET * (offset + 1)]
    //     .chunks(32)
    //     .map(|chunk| {
    //         chunk
    //             .iter()
    //             .map(|byte| format!("{:02x} ", byte))
    //             .collect::<String>()
    //     })
    //     .collect::<Vec<String>>()
    //     .join("\n");
    //
    // let surface = font
    //     .render(&text)
    //     .blended_wrapped(Color::RGBA(255, 255, 255, 255), WINDOW_WIDTH)
    //     .unwrap();
    //
    // let texture_creator = canvas.texture_creator();
    // let texture = texture_creator
    //     .create_texture_from_surface(&surface)
    //     .unwrap();
    // canvas
    //     .copy(
    //         &texture,
    //         None,
    //         Some(Rect::new(
    //             0,
    //             WINDOW_HEIGHT as i32 / 2 + 2,
    //             WINDOW_WIDTH,
    //             WINDOW_HEIGHT / 2 - 2,
    //         )),
    //     )
    //     .unwrap();

    canvas.present();
}
fn key2btn(key: Keycode) -> Option<usize> {
    match key {
        Keycode::Num1 => Some(0x1),
        Keycode::Num2 => Some(0x2),
        Keycode::Num3 => Some(0x3),
        Keycode::Num4 => Some(0xC),
        Keycode::Q => Some(0x4),
        Keycode::W => Some(0x5),
        Keycode::E => Some(0x6),
        Keycode::R => Some(0xD),
        Keycode::A => Some(0x7),
        Keycode::S => Some(0x8),
        Keycode::D => Some(0x9),
        Keycode::F => Some(0xE),
        Keycode::Z => Some(0xA),
        Keycode::X => Some(0x0),
        Keycode::C => Some(0xB),
        Keycode::V => Some(0xF),
        _ => None,
    }
}

pub fn init_gui(chip8: &mut Emu) {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("Chip8", WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    canvas.clear();
    canvas.present();

    let audio_subsystem = sdl_context.audio().unwrap();
    let desired_spec = AudioSpecDesired {
        freq: Some(44100),
        channels: Some(1), // mono
        samples: None,     // default sample size
    };

    let device = audio_subsystem
        .open_playback(None, &desired_spec, |spec| {
            // initialize the audio callback
            SquareWave {
                phase_inc: 440.0 / spec.freq as f32,
                phase: 0.0,
                volume: 0.01,
            }
        })
        .unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();

    'gameloop: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'gameloop,
                Event::KeyDown {
                    keycode: Some(key), ..
                } => {
                    if let Some(btn) = key2btn(key) {
                        chip8.keypress(btn, true);
                    } else {
                        match key {
                            Keycode::Left => chip8.set_offset(chip8.get_offset() - 1),
                            Keycode::Right => chip8.set_offset(chip8.get_offset() + 1),
                            Keycode::Escape => break 'gameloop,
                            _ => (),
                        }
                    }
                }
                Event::KeyUp {
                    keycode: Some(key), ..
                } => {
                    if let Some(btn) = key2btn(key) {
                        chip8.keypress(btn, false);
                    }
                }
                _ => {}
            }
        }
        for _ in 0..TICKS_PER_FRAME {
            chip8.tick();
        }
        chip8.tick_timer();
        if chip8.get_beep() {
            device.resume();
        } else {
            device.pause();
        };

        draw_screen(&chip8, &mut canvas);
    }
}
