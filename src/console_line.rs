use termcolor::{ColorSpec, StandardStream, WriteColor, Color};

pub struct ConsoleLine {
    pub text: String,
    pub color: termcolor::Color
}

//fn to print colored text
pub fn print_colored_text(line: ConsoleLine, stdout: &mut StandardStream){
    stdout.set_color(ColorSpec::new().set_fg(Some(line.color))).unwrap();
    println!("{}", line.text);
    stdout.reset().unwrap();
}

//text to print at the start
pub fn print_start_screen(stdout: &mut StandardStream){
    print_colored_text(ConsoleLine { text: String::from("Welcome to Nevix's version of hangman!"), color: Color::Red }, stdout);
    println!("");
    print_colored_text(ConsoleLine { text: String::from("This awesome hangman game was made with the Rust programming language!"), color: Color::Red }, stdout);
    print_colored_text(ConsoleLine { text: String::from("To play the game, type 'play', to get more info about the game, type 'info'"), color: Color::Red }, stdout);
}