use crate::console_line::ConsoleLine;
use termcolor::{Color, StandardStream, ColorChoice };
use std::io;

mod console_line;


fn main() {
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    console_line::print_colored_text(ConsoleLine { text: String::from("Welcome to Nevix's version of hangman!"), color: Color::Red }, &mut stdout);
    println!("");




    //dont close
    //exit
    println!("Press Enter to exit...");
    let mut _input = String::new();
    io::stdin().read_line(&mut _input).expect("Failed to read input");
}
