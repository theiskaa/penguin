//! Main module for the Penguin password generator.
//!
//! This module provides the high-level interface for generating passwords with various
//! customization options. It wraps the lower-level mixer functionality into a simpler API.
//!
//! The Penguin password generator takes a set of base words and uses them to generate secure,
//! memorable passwords. You can customize the complexity level, length, and whether to use whole
//! words or mix characters.
//!
//! # Examples:
//!
//! Generate passwords using default settings (medium complexity, whole words, 12 characters):
//!
//! ```
//! use penguin::Penguin;
//!
//! let penguin = Penguin::new(vec!["hello", "world"]);
//! let passwords = penguin.generate_password(3, None, None, None);
//! ```
//!
//! Generate passwords with custom settings for more security:
//!
//! ```
//! use penguin::{Penguin, mixer::ComplexityLevel};
//!
//! let penguin = Penguin::new(vec!["secure", "password"]);
//! let passwords = penguin.generate_password(
//!     2,                              // Generate 2 passwords
//!     Some(ComplexityLevel::Hard),    // Use hard complexity
//!     Some(false),                    // Mix characters instead of whole words
//!     Some(16)                        // Make them 16 characters long
//! );
//! ```

use mixer::{ComplexityLevel, PenguinMixer};

pub mod mixer;

/// The main struct for generating passwords from a set of base words.
///
/// The Penguin struct takes a set of base words during initialization and uses them
/// to generate passwords. The base words serve as the foundation for password generation,
/// either being used whole or mixed character by character depending on the settings.
pub struct Penguin<'a> {
    base_input: Vec<&'a str>,
}

impl<'a> Penguin<'a> {
    /// Creates a new Penguin instance with the provided base words.
    ///
    /// The base words should be meaningful words that you can remember. They will be used
    /// as the foundation for generating passwords. The generator will either use these words
    /// whole (separated by numbers/special chars) or mix their characters randomly depending
    /// on the complexity settings used.
    pub fn new(base_input: Vec<&'a str>) -> Self {
        Self { base_input }
    }

    /// Generates multiple passwords with customizable settings.
    ///
    /// This method generates a specified number of passwords using the base words provided
    /// during initialization. You can customize several aspects of password generation:
    ///
    /// The complexity level determines how the passwords are generated:
    /// - Basic: Uses whole words with numbers
    /// - Medium: Uses whole words with numbers and special characters
    /// - Hard: Mixes characters from words with numbers and special characters
    /// - Penguin: Creates a completely random 64-character password
    ///
    /// The whole words setting determines whether to use complete words or mix individual
    /// characters. Using whole words creates more memorable passwords, while mixing characters
    /// provides more randomness.
    ///
    /// The length setting controls the final password length. If using whole words, passwords
    /// may be slightly longer to accommodate complete words plus separators. For mixed character
    /// passwords, the length will be exact.
    ///
    /// If no customization options are provided (all None), the generator uses medium complexity,
    /// whole words, and 12-character length as defaults.
    pub fn generate_password(
        self,
        count: usize,
        complexity: Option<ComplexityLevel>,
        use_whole_words: Option<bool>,
        length: Option<usize>,
    ) -> Vec<String> {
        let mixer = match (complexity, use_whole_words, length) {
            (None, None, None) => PenguinMixer::default(),
            (complexity, use_whole_words, length) => PenguinMixer::new(
                complexity.unwrap_or(ComplexityLevel::Medium),
                use_whole_words.unwrap_or(true),
                length.unwrap_or(12),
            ),
        };

        let mut collected = Vec::with_capacity(count);
        for _ in 0..count {
            collected.push(mixer.mix_password(&self.base_input));
        }

        collected
    }
}
