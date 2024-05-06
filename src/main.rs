use crossterm::event::{poll, read, Event, KeyCode};
use crossterm::{cursor, terminal, style};
use crossterm::style::Color;
use crossterm::{execute, queue};
use fastrand;
use std::collections::vec_deque::VecDeque;
use std::io::{self, Write, BufRead, BufReader};
use std::thread::sleep;
use std::time::Duration;
use std::fs::File;
use std::env;

const DROP_TIME: usize = 20;
const COLOR_SPEED: usize = 1; // smaller is faster
const BUILTIN_COLORS: [Color; 30] = [
    Color::AnsiValue(196),
    Color::AnsiValue(202),
    Color::AnsiValue(208),
    Color::AnsiValue(208),
    Color::AnsiValue(220),
    Color::AnsiValue(226),
    Color::AnsiValue(190),
    Color::AnsiValue(154),
    Color::AnsiValue(118),
    Color::AnsiValue(082),
    Color::AnsiValue(046),
    Color::AnsiValue(047),
    Color::AnsiValue(048),
    Color::AnsiValue(049),
    Color::AnsiValue(050),
    Color::AnsiValue(051),
    Color::AnsiValue(045),
    Color::AnsiValue(039),
    Color::AnsiValue(033),
    Color::AnsiValue(027),
    Color::AnsiValue(021),
    Color::AnsiValue(057),
    Color::AnsiValue(093),
    Color::AnsiValue(129),
    Color::AnsiValue(165),
    Color::AnsiValue(201),
    Color::AnsiValue(200),
    Color::AnsiValue(199),
    Color::AnsiValue(198),
    Color::AnsiValue(197)
];
fn main() -> io::Result<()> {
    let mut colors = Vec::new();
    let (color_num, colors) = if let Some(file) = env::args().skip(1).next() {
        let file = File::open(file)?;
        let file = BufReader::new(file);
        for line in file.lines() {
            colors.push(Color::AnsiValue(line?.parse().expect("Unexpected element in file")));
        }
        (colors.len(), &colors[..])
    } else {
        let slice = &BUILTIN_COLORS[..];
        (slice.len(), slice)
    };
    let mut stdout = io::stdout();
    terminal::enable_raw_mode()?;
    execute!(stdout, terminal::EnterAlternateScreen, cursor::Hide)?;

    let (mut w, mut h) = terminal::size()?;
    let mut drops: Vec<(usize, VecDeque<(u32, u16, u16)>)> =
        (0..(w / 2)).map( |_| (
            fastrand::usize(0..DROP_TIME),
            VecDeque::new()
        )).collect();

    let mut run = true;
    while run {
        if poll(Duration::from_secs(0))? {
            match read()? {
                Event::Key(event) => {
                    if let KeyCode::Char('q') = event.code {
                        run = false
                    }
                }
                Event::Resize(nw, nh) => {
                    w = nw;
                    h = nh;
                    while (drops.len() as u16) < w / 2 {
                        drops.push((
                            fastrand::usize(DROP_TIME..DROP_TIME*2),
                            VecDeque::new()
                        ));
                    }
                    queue!(stdout, terminal::Clear(terminal::ClearType::All))?;
                }
                _ => (),
            }
        }
        for (x, (timer, column)) in drops.iter_mut().enumerate() {
            if *timer == 0 {
                *timer = fastrand::usize(DROP_TIME..DROP_TIME*2); 
                column.push_front((
                    fastrand::u32(0..(color_num << COLOR_SPEED) as u32),
                    0,
                    fastrand::u16((DROP_TIME/2) as u16..(DROP_TIME-1) as u16)
                ));
            }
            *timer -= 1;

            if let Some((_, position, height)) = column.back() {
                if position > height && position - height > h {
                    column.pop_back();
                }
            }
            for (color, position, height) in column.iter_mut() {
                if x * 2 < w as usize {
                    let character = fastrand::u8(33..=126) as char;
                    if *position < h {
                        queue!(stdout,
                            cursor::MoveTo(x as u16 * 2, *position as u16),
                            style::SetForegroundColor(colors[(*color as usize) >> COLOR_SPEED]),
                            style::Print(character),
                        )?;
                    }
                    if *position < h - 1 {
                        queue!(stdout,
                            cursor::MoveTo(x as u16 * 2, *position as u16 + 1),
                            style::SetForegroundColor(Color::AnsiValue(231)),
                            style::Print(character),
                        )?;
                    }
                    if position >= height {
                        queue!(stdout,
                            cursor::MoveTo(x as u16 * 2, (*position - *height) as u16),
                            style::Print(' '),
                        )?;
                    }
                }
                *position += 1;
                *color = (*color + 1) % (color_num << COLOR_SPEED) as u32;
            }
        }
        stdout.flush()?;
        sleep(Duration::from_millis(40));
    }

    terminal::disable_raw_mode()?;
    execute!(stdout, terminal::LeaveAlternateScreen, cursor::Show)?;
    Ok(())
}
