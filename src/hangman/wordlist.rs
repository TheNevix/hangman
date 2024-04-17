use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct WordList {
    pub words: Vec<String>
}