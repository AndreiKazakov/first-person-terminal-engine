use std::io::{stdout, Stdout, Write};

use termion::raw::{IntoRawMode, RawTerminal};
use termion::{color, cursor, style};

pub struct Screen {
    stdout: RawTerminal<Stdout>,
}
impl Screen {
    pub fn new() -> Self {
        let mut std = stdout().into_raw_mode().unwrap();
        write!(std, "{}", termion::clear::All).unwrap();
        write!(std, "{}", cursor::Hide).unwrap();
        Self { stdout: std }
    }
    
    pub fn move_to(&mut self, x: usize, y: usize) {
        write!(self.stdout, "{}", cursor::Goto(x as u16, y as u16));
    }

    pub fn draw_square(&mut self, r: u8, g: u8, b: u8) {
        write!(self.stdout, "{} ", color::Rgb(r, g, b).bg_string());
    }

    pub fn clear_screen(&mut self) {
        write!(self.stdout, "{}", termion::clear::All).unwrap();
    }

    pub fn reset(&mut self) {
        write!(self.stdout, "{}", cursor::Show).unwrap();
        write!(self.stdout, "{}", style::Reset).unwrap();
    }
}
