extern crate term_size;
mod frames;

fn main() {
    let (cols, rows) = term_size::dimensions().unwrap();
    if rows < 18 || cols < 50 {
        println!("Please resize your terminal to at least 50x18");
        std::process::exit(1);
    }
    print!("\x1B[2J\x1B[1;1H");
    loop {
        unsafe {
            // move cursor to top left
            println!("\x1B[1;80H");
            print!("{}", frames::next_frame());
            std::thread::sleep(std::time::Duration::from_millis(75));
        }
    }
}
