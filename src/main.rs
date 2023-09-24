use std::io::stdout;
use crossterm::{ExecutableCommand, QueueableCommand};

mod frames;

fn main() {
    let (cols, rows) = term_size::dimensions().unwrap();
    if rows < 18 || cols < 50 {
        println!("Please resize your terminal to at least 50x18");
        std::process::exit(1);
    }
    stdout().execute(
        crossterm::terminal::Clear(crossterm::terminal::ClearType::All)
    ).expect("Could not clear terminal.");
    loop {
        unsafe {
            // move cursor to top left
            stdout().queue(
              crossterm::style::SetForegroundColor(frames::next_color())
            ).expect("Could not set foreground color.").queue(
                crossterm::cursor::MoveTo(0, 0)
            ).expect("Could not move cursor.");
            println!("{}", frames::next_frame());
            std::thread::sleep(std::time::Duration::from_millis(75));
        }
    }
}
