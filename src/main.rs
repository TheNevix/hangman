use crate::console_line::ConsoleLine;
use termcolor::{Color, StandardStream, ColorChoice };
use std::io;

mod console_line;
mod hangman;


fn main() {
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    
    console_line::print_start_screen(&mut stdout);

    let mut input = String::new();

    loop{
        input.clear();
        //wait for either play or command input
        match io::stdin()
        .read_line(&mut input) {
            Ok(_) => {
                let input = input.trim();
                if input == "play" || input == "info" {
                    break;
                }
                else{
                    //error
                }
            }
            Err(_) => {

            }
        }
    }

    if input.trim() == "play" {
        hangman::play_game(&mut stdout);
    }

    



    //dont close
    //exit
    println!("Press Enter to exit...");
    let mut _input = String::new();
    io::stdin().read_line(&mut _input).expect("Failed to read input");
}
