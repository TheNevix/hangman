use termcolor::{ColorSpec, StandardStream, WriteColor, Color};
use cliclr::print_colored_text;
use cliclr::ConsoleLine;

//text to print at the start
pub fn print_start_screen(stdout: &mut StandardStream){
    print_colored_text(&ConsoleLine { text: String::from("Welcome to Nevix's version of hangman!"), color: Color::Red }, stdout);
    println!("");
    print_colored_text(&ConsoleLine { text: String::from("This awesome hangman game was made with the Rust programming language!"), color: Color::Red }, stdout);
    print_colored_text(&ConsoleLine { text: String::from("To play the game, type 'play', to get more info about the game, type 'info'"), color: Color::Red }, stdout);
}

pub fn print_info_screen(stdout: &mut StandardStream){
    println!("");
    print_colored_text(&ConsoleLine { text: String::from("Hangman is a simple word guessing game where you need guess the word before the hangman has been completed."), color: Color::Red }, stdout);
    print_colored_text(&ConsoleLine { text: String::from("You have a certain amount of guesses, all words are English words!"), color: Color::Red }, stdout);
    println!("");
    print_colored_text(&ConsoleLine { text: String::from("Type 'play' to start."), color: Color::Red }, stdout);

}

pub fn print_stripes_by_length(guessed_chars: &Vec<char>, word_to_guess: &Vec<char>, stdout: &mut StandardStream) {
    let mut stripes_to_print = String::new();

    for char in word_to_guess {
        let is_guessed = guessed_chars.contains(char);
        if is_guessed {
            let text = format!("{} ", char);
            stripes_to_print.push_str(&text);
        }
        else{
            stripes_to_print.push_str("_ ");
        }
    }

    print_colored_text(&ConsoleLine { text: stripes_to_print, color: Color::Red }, stdout)
}

pub fn print_ask_for_guess(guess_count: &u32, stdout: &mut StandardStream){
    let text = format!("Enter guess {}", guess_count);
    print_colored_text(&ConsoleLine { text: text, color: Color::Red }, stdout);
}

pub fn print_guess_found(guess: &char, stdout: &mut StandardStream){
    let text = format!("You guessed a correct letter! The word contains at least 1 '{}'", guess);
    print_colored_text(&ConsoleLine { text: text, color: Color::Green }, stdout);
}

pub fn print_guess_not_found(guess: &char, stdout: &mut StandardStream){
    let text = format!("You guessed a incorrect letter! The word does not contain the letter '{}'", guess);
    print_colored_text(&ConsoleLine { text: text, color: Color::Red}, stdout);
}

pub fn print_hangman_phase(phase: &u32, stdout: &mut StandardStream){
    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Red))).unwrap();

    match phase {
        0 => println!("\n\n\n\n\n"),
        1 => println!("\n\n\n\n\n========="),
        2 => {
            println!("   |");
            println!("   |");
            println!("   |");
            println!("   |");
            println!("   |");
            println!("=========");
        },
        3 => {
            println!("   +---+");
            println!("   |   |");
            println!("   O   |");
            println!("       |");
            println!("       |");
            println!("=========");
        },
        4 => {
            println!("   +---+");
            println!("   |   |");
            println!("   O   |");
            println!("   |   |");
            println!("       |");
            println!("=========");
        },
        5 => {
            println!("   +---+");
            println!("   |   |");
            println!("   O   |");
            println!("  /|   |");
            println!("       |");
            println!("=========");
        },
        6 => {
            println!("   +---+");
            println!("   |   |");
            println!("   O   |");
            println!("  /|\\  |");
            println!("       |");
            println!("=========");
        },
        7 => {
            println!("   +---+");
            println!("   |   |");
            println!("   O   |");
            println!("  /|\\  |");
            println!("  /    |");
            println!("=========");
        },
        8 => {
            println!("   +---+");
            println!("   |   |");
            println!("   O   |");
            println!("  /|\\  |");
            println!("  / \\  |");
            println!("=========");
        },
        9 => {
            println!("   +---+");
            println!("   |   |");
            println!("   O   |");
            println!("  /|\\  |");
            println!("  / \\  |");
            println!(" /     |");
            println!("=========");
        },
        10 => {
            println!("   +---+");
            println!("   |   |");
            println!("   O   |");
            println!("  /|\\  |");
            println!("  / \\  |");
            println!(" /   \\ |");
            println!("=========");
        },
        11 => {
            println!("   +---+");
            println!("   |   |");
            println!("  [O]  |");
            println!("  /|\\  |");
            println!("  / \\  |");
            println!(" /   \\ |");
            println!("=========");
        },
        _ => println!("Invalid phase"),
    }

    stdout.reset().unwrap();
}

pub fn print_lost_text(word_to_guess: &String, stdout: &mut StandardStream){
    let text = format!("You ran out of guesses and lost! The word was '{}'", word_to_guess);
    print_colored_text(&ConsoleLine { text: text, color: Color::Red }, stdout);
}

pub fn print_win_text(word_to_guess: &String, amount_of_guesses: &u32, stdout: &mut StandardStream){
    let text = format!("You win! The word is '{}'. You got the anwser in {} guesses!", word_to_guess, amount_of_guesses);
    print_colored_text(&ConsoleLine { text: text, color: Color::Green }, stdout);
}

pub fn print_ask_another_game(stdout: &mut StandardStream){
    print_colored_text(&ConsoleLine { text: String::from("Would you like to play another game of hangman?"), color: Color::Red }, stdout);
    print_colored_text(&ConsoleLine { text: String::from("Type 'yes' to play another game of hangman and 'no' to quit."), color: Color::Red }, stdout);
}

pub fn print_alr_guessed_letter(stdout: &mut StandardStream){
    print_colored_text(&ConsoleLine { text: String::from("You already guessed that letter! Try to guess a different letter!"), color: Color::Red }, stdout);
}
