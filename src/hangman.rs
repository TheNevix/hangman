use rand::seq::SliceRandom;
use rand::thread_rng;
use termcolor::StandardStream;
use crate::console_line::{print_ask_for_guess, print_guess_found, print_guess_not_found, print_hangman_phase, print_lost_text, print_stripes_by_length, print_win_text};
use std::error::Error;
use std::io;
use self::word_to_guess::WordToGuess;
use self::wordlist::WordList;

mod wordlist;
mod word_to_guess;

pub fn play_game(stdout: &mut StandardStream){
    
    let word_to_guess = generate_word_to_guess().unwrap();
    let mut amount_of_guesses: u32 = 1;
    let mut amount_of_incorrect_guesses = 0;
    let max_incorrect_guesses: u32 = 11;
    let mut incorrect_letters_guessed: Vec<char> = vec![];
    let mut correct_letters_guessed: Vec<char> = vec![];
    let mut letters_guessed: u32 = 0;
    let word_to_guess_vec: Vec<char> = word_to_guess.word.chars().collect();

    loop{
        print_stripes_by_length(&correct_letters_guessed, &word_to_guess_vec, stdout);
        print_ask_for_guess(&amount_of_guesses, stdout);

        //println!("{}", word_to_guess.word);

        //get user input char
        let entered_char = ask_guess().unwrap();
        let char_found: bool = word_to_guess_vec.contains(&entered_char);

        amount_of_guesses = amount_of_guesses + 1;

        if char_found {
            //tell user that they found a letter
            for char in &word_to_guess_vec {
                if &entered_char == char{
                    letters_guessed = letters_guessed + 1;
                }
            }
            print_guess_found(&entered_char, stdout);
            correct_letters_guessed.push(entered_char);
        } 
        else{
            //tell user that they didnt found a letter
            print_guess_not_found(&entered_char, stdout);
            amount_of_incorrect_guesses = amount_of_incorrect_guesses + 1;
            incorrect_letters_guessed.push(entered_char);
        }

        //draw hangman
        print_hangman_phase(&amount_of_incorrect_guesses, stdout);

        //check if 11 wrong guesses
        if amount_of_incorrect_guesses == max_incorrect_guesses {
            //print you lost
            //info to play another game
            print_lost_text(&word_to_guess.word, stdout);
            break;
        }
        
        //check if chars are guessed
        if letters_guessed == word_to_guess.length {
            print_win_text(&word_to_guess.word, &amount_of_guesses, stdout);
            break;
        }
    }
}


fn generate_word_to_guess() -> Result<WordToGuess, Box<dyn Error>> {
    // Read the words file
    let data = include_str!("words.json");
    let words: WordList = serde_json::from_str(&data)?;

    // Get a random word from the list
    let mut rng = thread_rng();
    let random_word = words.words.choose(&mut rng)
        .ok_or("No words available in the list")?; // Handle the case where no words are available

    Ok(WordToGuess { word: random_word.to_string(), length: random_word.len() as u32 }) // Return the chosen word
}

fn ask_guess() -> Result<char, Box<dyn Error>> {
    //create input var
    let mut result = String::new();

        // Read a line of input from the user
    io::stdin().read_line(&mut result).map_err(|e| e)?; // If an error occurs, return it

    // Attempt to parse the first character from the input
    let first_char = result.trim().chars().next().ok_or_else(|| {
        io::Error::new(io::ErrorKind::InvalidInput, "No input provided")
    })?;

    Ok(first_char)
}