use serde::{Deserialize};
use std::fs;

#[derive(Deserialize, Debug)]
pub struct WordList {
    pub words: Vec<String>
}

impl WordList {
    pub fn get_words(){
        let data = fs::read_to_string("words.json").expect("Unable to read file");
        let wordslist: WordList = serde_json::from_str(&data).expect("Unable to parse JSON");

        println!("Easy words: {:?}", wordslist.words);
    }
}