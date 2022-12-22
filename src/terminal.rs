use crate::cpu::*;
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{
    io::{self, Stdout},
    time::Duration,
};
use tui::{
    backend::CrosstermBackend,
    layout::Rect,
    style::{Color, Style},
    text::Span,
    widgets::{canvas::Canvas, Block, Borders},
    Terminal,
};

const SCALE: u32 = 1;
const WINDOW_WIDTH: u32 = (SCREEN_WIDTH as u32) * SCALE;
const WINDOW_HEIGHT: u32 = (SCREEN_HEIGHT as u32) * SCALE;
const TICKS_PER_FRAME: usize = 1;

fn key2btn(key: KeyCode) -> Option<usize> {
    match key {
        KeyCode::Char('1') => Some(0x1),
        KeyCode::Char('2') => Some(0x2),
        KeyCode::Char('3') => Some(0x3),
        KeyCode::Char('4') => Some(0xC),
        KeyCode::Char('Q') | KeyCode::Char('q') => Some(0x4),
        KeyCode::Char('W') | KeyCode::Char('w') => Some(0x5),
        KeyCode::Char('E') | KeyCode::Char('e') => Some(0x6),
        KeyCode::Char('R') | KeyCode::Char('r') => Some(0xD),
        KeyCode::Char('A') | KeyCode::Char('a') => Some(0x7),
        KeyCode::Char('S') | KeyCode::Char('s') => Some(0x8),
        KeyCode::Char('D') | KeyCode::Char('d') => Some(0x9),
        KeyCode::Char('F') | KeyCode::Char('f') => Some(0xE),
        KeyCode::Char('Z') | KeyCode::Char('z') => Some(0xA),
        KeyCode::Char('X') | KeyCode::Char('x') => Some(0x0),
        KeyCode::Char('C') | KeyCode::Char('c') => Some(0xB),
        KeyCode::Char('V') | KeyCode::Char('v') => Some(0xF),
        _ => None,
    }
}

pub fn draw_screen(chip8: &mut Emu, terminal: &mut Terminal<CrosstermBackend<Stdout>>) {
    terminal
        .draw(|f| {
            let area = Rect::new(0, 0, WINDOW_WIDTH as u16, WINDOW_HEIGHT as u16);
            let canvas = Canvas::default()
                .block(Block::default().borders(Borders::ALL))
                .paint(|ctx| {
                    let screen = chip8.get_display();
                    for (i, pixel) in screen.iter().enumerate() {
                        if *pixel {
                            let x = (i as u32 % WINDOW_WIDTH) as f64;
                            let y = (WINDOW_HEIGHT - (i as u32 / WINDOW_WIDTH)) as f64;

                            ctx.print(x, y, Span::styled("█", Style::default().fg(Color::White)));
                        }
                    }
                })
                .x_bounds([0.0, WINDOW_WIDTH as f64])
                .y_bounds([0.0, WINDOW_HEIGHT as f64]);
            f.render_widget(canvas, area);
        })
        .unwrap();
}

pub fn init_terminal(chip8: &mut Emu) {
    enable_raw_mode().unwrap();
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen).unwrap();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend).unwrap();
    terminal.clear().unwrap();
    let mut key_g: usize = 0;

    'gameloop: loop {
        if event::poll(Duration::from_micros(1)).unwrap() {
            if let Event::Key(key) = event::read().unwrap() {
                if let Some(key) = key2btn(key.code) {
                    key_g = key;
                    chip8.keypress(key_g, true);
                } else {
                    match key.code {
                        KeyCode::Esc => {
                            break 'gameloop;
                        }
                        _ => {}
                    }
                }
            }
        } else {
            chip8.keypress(key_g, false);
        }
        for _ in 0..TICKS_PER_FRAME {
            chip8.tick();
        }
        chip8.tick_timer();
        draw_screen(chip8, &mut terminal);
    }
    execute!(terminal.backend_mut(), LeaveAlternateScreen).unwrap();
    disable_raw_mode().unwrap();
    terminal.clear().unwrap();
}
