use crate::console_line::{print_ask_another_game, print_info_screen, ConsoleLine};
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
        play_hangman_game(&mut stdout);
    }
    else if input.trim() == "info" {
        console_line::print_info_screen(&mut stdout);
        let mut input = String::new();
        match io::stdin()
        .read_line(&mut input) {
            Ok(_) => {
                let input = input.trim();
                if input == "play" {
                    play_hangman_game(&mut stdout);
                }
                else{
                    //error
                }
            }
            Err(_) => {

            }
        }
        
    }

    



    //dont close
    //exit
    println!("Press Enter to exit...");
    let mut _input = String::new();
    io::stdin().read_line(&mut _input).expect("Failed to read input");
}

fn play_hangman_game(stdout: &mut StandardStream){
    loop{
        hangman::play_game(stdout);

        //ask to play another game? check for 'yes' or 'no', with no quiting
        print_ask_another_game(stdout);

        let mut input = String::new();
        match io::stdin()
        .read_line(&mut input) 
        {
            Ok(_) => 
            {
                let input = input.trim();
                if input == "no" {
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
}
