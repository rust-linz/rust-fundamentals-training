use std::io::Write;

use crossterm::{execute, terminal::ClearType};
use seven_segments::{decimal_to_segments, draw_7_segments};

fn main() {
    let mut stdout = std::io::stdout();
    execute!(stdout, crossterm::terminal::Clear(ClearType::All)).unwrap();
    execute!(stdout, crossterm::cursor::Hide).unwrap();

    let number_to_display: u32 = 1234567890;

    // Integer logarithm avoids floating-point rounding errors. It is undefined
    // for 0 (hence `checked_`), but 0 has one digit, too.
    let length_of_number = number_to_display.checked_ilog10().map_or(1, |log| log + 1);

    for ix in 0..length_of_number {
        let digit = (number_to_display / 10u32.pow((length_of_number - 1) - ix)) % 10;
        let segments = draw_7_segments(decimal_to_segments(digit as u8));

        for (pos, c) in segments {
            let x = (pos as u16 & 0xf0) >> 4;
            let y = pos as u16 & 0xf;
            execute!(stdout, crossterm::cursor::MoveTo(x + 4 * ix as u16, y)).unwrap();
            write!(stdout, "{c}").unwrap();
        }
    }

    execute!(stdout, crossterm::cursor::MoveTo(0, 4)).unwrap();
    execute!(stdout, crossterm::cursor::Show).unwrap();
}
