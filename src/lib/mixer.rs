//! Password generation and mixing module for the Penguin password generator.
//!
//! This module is responsible for the core password generation functionality, providing
//! multiple strategies for creating secure yet memorable passwords. The module offers
//! flexible configuration through complexity levels and mixing approaches to balance
//! security with usability.
//!
//! # Password Generation Strategies
//!
//! The module supports two main approaches to password generation. The first approach
//! uses complete words from the input and adds separators between them (numbers and/or
//! special characters). This creates more memorable passwords while maintaining security.
//! The second approach breaks words into individual characters and mixes them randomly
//! with numbers and special characters, providing higher entropy and randomness.
//!
//! # Complexity Levels
//!
//! The complexity system offers four distinct levels of security. The Basic level
//! combines whole words with numbers as separators. The Medium level enhances this
//! by using both numbers and special characters. The Hard level thoroughly mixes
//! characters with numbers and special characters. Finally, the Penguin level creates
//! a maximum-security 64-character random password.
//!
//! # Examples
//!
//! Here's how to generate a basic, memorable password using whole words:
//!
//! ```
//! use penguin::mixer::{PenguinMixer, ComplexityLevel};
//!
//! let mixer = PenguinMixer::new(ComplexityLevel::Basic, true, 12);
//! let password = mixer.mix_password(&vec!["hello", "world"]);
//! // Might generate: "hello2world5"
//! ```
//!
//! For a more secure password using character mixing:
//!
//! ```
//! let mixer = PenguinMixer::new(ComplexityLevel::Hard, false, 16);
//! let password = mixer.mix_password(&vec!["penguin", "secure"]);
//! // Might generate: "p3n@gu1nS#cur3"
//! ```
//!
//! And for maximum security with a random password:
//!
//! ```
//! let mixer = PenguinMixer::new(ComplexityLevel::Penguin, false, 64);
//! let password = mixer.mix_password(&vec!["not", "used"]);
//! // Generates a 64-character random string using all possible characters
//! ```

use rand::prelude::SliceRandom;
use rand::Rng;

// Character sets used for password generation
const NUMBERS: &str = "0123456789";
const SPECIAL_CHARS: &str = "!@#$%^&*";
const LOWERCASE: &str = "abcdefghijklmnopqrstuvwxyz";
const UPPERCASE: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";

/// Defines the complexity level for password generation.
///
/// The complexity levels provide different balances between security and memorability.
/// Basic level is suitable for systems with minimal complexity requirements. Medium
/// level offers a good balance of security and usability for most purposes. Hard
/// level provides high security while maintaining some structure. The Penguin level
/// ensures maximum security through complete randomness.
#[derive(Debug, Clone, Copy)]
pub enum ComplexityLevel {
    Basic,   // Uses whole words + numbers
    Medium,  // Uses whole words + numbers + special chars
    Hard,    // Mixes characters from words + numbers + special chars
    Penguin, // Ultimate 64-char random password with all possible combinations
}

/// Main password mixer struct that handles password generation with various settings.
///
/// The PenguinMixer combines the input words and complexity settings to generate
/// passwords. It can be configured to use either whole words or mix individual
/// characters, and supports different complexity levels and length requirements.
pub struct PenguinMixer {
    pub length: usize,
    pub complexity: ComplexityLevel,
    pub use_whole_words: bool,
}

/// Default implementation providing medium complexity with whole words and 12 character length.
///
/// These defaults are chosen to create passwords that are secure enough for most
/// common uses, while remaining easy to remember due to whole word usage. The
/// resulting passwords are compatible with most password requirements found in
/// modern systems.
impl Default for PenguinMixer {
    fn default() -> Self {
        Self {
            length: 12,
            complexity: ComplexityLevel::Medium,
            use_whole_words: true,
        }
    }
}

impl PenguinMixer {
    /// Creates a new PenguinMixer with custom settings.
    pub fn new(complexity: ComplexityLevel, use_whole_words: bool, length: usize) -> Self {
        Self {
            complexity,
            use_whole_words,
            length,
        }
    }

    /// Main password generation method that handles both Penguin and regular complexity levels.
    /// Returns an empty string if no input words are provided.
    pub fn mix_password(&self, base_input: &Vec<&str>) -> String {
        if base_input.is_empty() {
            return String::new();
        }

        match self.complexity {
            ComplexityLevel::Penguin => Self::generate_penguin_password(),
            _ => self.generate_regular_password(base_input),
        }
    }

    /// Generates a maximum-security 64-character password using all possible character types.
    /// This method ignores the input words and generates a completely random password.
    fn generate_penguin_password() -> String {
        let mut rng = rand::thread_rng();
        let all_chars = format!("{}{}{}{}", LOWERCASE, UPPERCASE, NUMBERS, SPECIAL_CHARS);
        let chars: Vec<char> = all_chars.chars().collect();

        let mut password = String::with_capacity(64);
        for _ in 0..64 {
            password.push(chars[rng.gen_range(0..chars.len())]);
        }
        password
    }

    /// Generates passwords based on input words with various complexity levels.
    /// Supports both whole-word and character mixing approaches.
    fn generate_regular_password(&self, base_input: &Vec<&str>) -> String {
        let mut rng = rand::thread_rng();
        let mut password = String::new();

        if self.use_whole_words {
            // Create a vector of available indices
            let mut available_indices: Vec<usize> = (0..base_input.len()).collect();
            available_indices.shuffle(&mut rng);

            // Use whole words approach
            let mut index = 0;
            while password.len() < self.length && index < available_indices.len() {
                let word = base_input[available_indices[index]];
                password.push_str(word);
                index += 1;

                // Add separators based on complexity
                match self.complexity {
                    ComplexityLevel::Basic => {
                        password.push(
                            NUMBERS
                                .chars()
                                .nth(rng.gen_range(0..NUMBERS.len()))
                                .unwrap(),
                        );
                    }
                    ComplexityLevel::Medium | ComplexityLevel::Hard => {
                        password.push(
                            SPECIAL_CHARS
                                .chars()
                                .nth(rng.gen_range(0..SPECIAL_CHARS.len()))
                                .unwrap(),
                        );
                        password.push(
                            NUMBERS
                                .chars()
                                .nth(rng.gen_range(0..NUMBERS.len()))
                                .unwrap(),
                        );
                    }
                    ComplexityLevel::Penguin => unreachable!(),
                }
            }

            // If we've used all words but still haven't reached desired length,
            // fill the rest with random characters
            if password.len() < self.length {
                let all_chars = format!("{}{}{}{}", LOWERCASE, UPPERCASE, NUMBERS, SPECIAL_CHARS);
                let chars: Vec<char> = all_chars.chars().collect();
                while password.len() < self.length {
                    password.push(chars[rng.gen_range(0..chars.len())]);
                }
            }
        } else {
            // Mix characters approach
            let mut combined = String::new();
            let mut available_indices: Vec<usize> = (0..base_input.len()).collect();
            available_indices.shuffle(&mut rng);

            // Combine words in random order without repetition
            for &idx in &available_indices {
                combined.push_str(base_input[idx]);
            }
            let chars: Vec<char> = combined.chars().collect();

            while password.len() < self.length {
                match self.complexity {
                    ComplexityLevel::Basic => {
                        if password.len() % 4 == 0 {
                            password.push(
                                NUMBERS
                                    .chars()
                                    .nth(rng.gen_range(0..NUMBERS.len()))
                                    .unwrap(),
                            );
                        } else if !chars.is_empty() {
                            password.push(chars[rng.gen_range(0..chars.len())]);
                        } else {
                            // If we've used all chars, use random ones
                            password.push(
                                LOWERCASE
                                    .chars()
                                    .nth(rng.gen_range(0..LOWERCASE.len()))
                                    .unwrap(),
                            );
                        }
                    }
                    ComplexityLevel::Medium | ComplexityLevel::Hard => match password.len() % 4 {
                        0 => password.push(
                            SPECIAL_CHARS
                                .chars()
                                .nth(rng.gen_range(0..SPECIAL_CHARS.len()))
                                .unwrap(),
                        ),
                        1 => password.push(
                            NUMBERS
                                .chars()
                                .nth(rng.gen_range(0..NUMBERS.len()))
                                .unwrap(),
                        ),
                        _ => {
                            if !chars.is_empty() {
                                password.push(chars[rng.gen_range(0..chars.len())]);
                            } else {
                                // If we've used all chars, use random ones
                                password.push(
                                    LOWERCASE
                                        .chars()
                                        .nth(rng.gen_range(0..LOWERCASE.len()))
                                        .unwrap(),
                                );
                            }
                        }
                    },
                    ComplexityLevel::Penguin => unreachable!(),
                }
            }
        }

        // Trim to exact length and shuffle if using Hard complexity
        password.truncate(self.length);
        if matches!(self.complexity, ComplexityLevel::Hard) {
            let mut password_chars: Vec<char> = password.chars().collect();
            password_chars.shuffle(&mut rng);
            password = password_chars.into_iter().collect();
        }

        password
    }
}
