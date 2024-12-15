use rand::prelude::SliceRandom;
use rand::Rng;

const NUMBERS: &str = "0123456789";
const SPECIAL_CHARS: &str = "!@#$%^&*";
const LOWERCASE: &str = "abcdefghijklmnopqrstuvwxyz";
const UPPERCASE: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";

#[derive(Debug, Clone, Copy)]
pub enum ComplexityLevel {
    Basic,   // Uses whole words + numbers
    Medium,  // Uses whole words + numbers + special chars
    Hard,    // Mixes characters from words + numbers + special chars
    Penguin, // Ultimate 64-char random password with all possible combinations
}

pub struct PenguinMixer {
    pub length: usize,
    pub complexity: ComplexityLevel,
    pub use_whole_words: bool,
}

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
    pub fn new(complexity: ComplexityLevel, use_whole_words: bool, length: usize) -> Self {
        Self {
            complexity,
            use_whole_words,
            length,
        }
    }

    pub fn mix_password(&self, base_input: &Vec<&str>) -> String {
        if base_input.is_empty() {
            return String::new();
        }

        match self.complexity {
            ComplexityLevel::Penguin => Self::generate_penguin_password(),
            _ => self.generate_regular_password(base_input),
        }
    }

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
