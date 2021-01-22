use std::io::{stdout, Stdout, Write};

use termion::raw::{IntoRawMode, RawTerminal};
use termion::{color, cursor, style};

pub struct RGB(pub u8, pub u8, pub u8);
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

    pub fn flush(&mut self) {
        self.stdout.flush().unwrap()
    }

    pub fn move_to(&mut self, x: usize, y: usize) {
        write!(self.stdout, "{}", cursor::Goto(x as u16, y as u16)).unwrap();
    }

    pub fn print_char(&mut self, c: char, bg: RGB) {
        write!(
            self.stdout,
            "{}{}",
            color::Rgb(bg.0, bg.1, bg.2).bg_string(),
            c
        )
        .unwrap();
    }

    pub fn clear_screen(&mut self) {
        write!(self.stdout, "{}", termion::clear::All).unwrap();
    }

    pub fn reset(&mut self) {
        write!(self.stdout, "{}", cursor::Show).unwrap();
        write!(self.stdout, "{}", style::Reset).unwrap();
    }
}
