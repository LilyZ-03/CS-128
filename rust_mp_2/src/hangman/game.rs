use crate::hangman::error::{HangmanError, HangmanErrorKind};

/// DO NOT CHANGE THIS, you may use this as you see fit
/// The number of guesses allowed before the game ends.
pub const NUM_GUESSES_TOTAL : usize = 6;

/// Struct to handle a Hangman game and store all necessary game state.
/// Feel free to design this struct however you like, as long as you don't
/// modify any of the function signatures in the impl block.
/// TODO: design the Hangman struct.
#[derive(Debug, Default)]
pub struct Hangman {
    word: String,
    num_guess_left: usize,
    correct_guess: std::collections::HashSet<char>,
    incorrect_guess: std::collections::HashSet<char>
}

/// Hangman impl block.
/// Your task is to finish all the functions in this block. You may add any
/// helper functions as you need. After you have finished all the functionality
/// below, feel free to run the program and play with the command line interface which should be 
/// fully functional assuming that your code passes all tests. See the README for instructions.
/// Note: Make sure not to change the function signatures below!!!
/// TODO: finish all the functions in the impl block.
impl Hangman {
    /// Instantiates a new Hangman game object for the given word.
    /// Returns an Ok(Hangman) if the word was valid.
    /// Returns an Err(HangmanError) if the word was an empty string or contained non-alpha chars.
    pub fn new(word: String) -> Result<Self, HangmanError> {
       if word.len() <= 0 {
        return Err(HangmanError::new(HangmanErrorKind::InvalidWord, word));
       }
       let char_vec: Vec<char> = word.chars().collect();
       for c in char_vec {
           if !c.is_ascii_alphabetic() {
            return Err(HangmanError::new(HangmanErrorKind::InvalidWord, word));
           }
       }
       Ok( Hangman { word: word.to_lowercase(), 
        num_guess_left: NUM_GUESSES_TOTAL,
        correct_guess: std::collections::HashSet::new(),
        incorrect_guess: std::collections::HashSet::new() })
    }

    /// Guesses a character in the hangman game and updates the game state
    /// Note: the guess is case INSENSITIVE (e.g., if the word is "abc," both 'A' and 'a' are correct guesses).
    /// Returns Ok(true) if the guess was valid and correct
    /// Returns Ok(false) if the guess was valid, but incorrect
    /// v Returns a GameAlreadyOver error if the game was already finished before the guess
    /// v Returns an InvalidCharacter error if the character is not alphabetic
    /// v Returns an AlreadyGuessedCharacter error if the character was already guessed 
    /// (either correctly or incorrectly) in the game.
    pub fn guess(&mut self, c: char) -> Result<bool, HangmanError> {
        if self.num_guess_left <= 0 {
            return Err(HangmanError::new(HangmanErrorKind::GameAlreadyOver, c.to_string()));
        }
        if !c.is_ascii_alphabetic() {
            return Err(HangmanError::new(HangmanErrorKind::InvalidCharacter, c.to_string()));
        }
        let c_lower: char = c.to_ascii_lowercase();
        if self.correct_guess.contains(&c) || self.incorrect_guess.contains(&c) {
            return Err(HangmanError::new(HangmanErrorKind::AlreadyGuessedCharacter, c.to_string()));
        }
        let char_vec: Vec<char> = self.word.chars().collect(); 
        self.num_guess_left -= 1;
        for i in char_vec {
            if i == c_lower {
                self.correct_guess.insert(c_lower);
                return Ok(true);
            }
        }
        self.incorrect_guess.insert(c_lower);
        Ok(false)
    }

    /// Returns a reference to the game word converted to lowercase.
    pub fn get_word(&self) -> &String {
        return &self.word;
    }

    /// Returns the number of guesses left before the guesser loses.
    pub fn get_num_guesses_left(&self) -> usize {
        return self.num_guess_left;
    }

    /// Returns a reference to a HashSet of all correct guessed characters.
    pub fn get_correct_guesses(&self) -> &std::collections::HashSet<char> {
        return &self.correct_guess;
    }

    /// Returns a reference to a HashSet of all incorrectly guessed characters.
    pub fn get_incorrect_guesses(&self) -> &std::collections::HashSet<char> {
        return &self.incorrect_guess;
    }

    /// Returns the result of the game.
    /// Returns Some(true) if the user guessed all the characters in the word without exceeding
    /// the allowed number of guesses.
    /// Returns Some(false) if the user made too many incorrect guesses.
    /// Otherwise, the game is in progress, so return None.
    pub fn get_game_result(&self) -> Option<bool> {
        let word_v: Vec<char> = self.word.chars().collect();
        if self.num_guess_left > 0 {
            for c in word_v {
                if !self.correct_guess.contains(&c) {
                    return None;
                }
            }
            return Some(true);
        } else {
            for c in word_v {
                if !self.correct_guess.contains(&c) {
                    return Some(false);
                }
            }
            return Some(true);
        }
    }
}
