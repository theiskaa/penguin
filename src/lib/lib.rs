//! TODO: add docs

use mixer::{ComplexityLevel, PenguinMixer};

pub mod mixer;

pub struct Penguin<'a> {
    base_input: Vec<&'a str>,
}

impl<'a> Penguin<'a> {
    pub fn new(base_input: Vec<&'a str>) -> Self {
        Self { base_input }
    }

    /// Generates passwords with optional customization parameters.
    /// If no parameters are provided, uses default mixer settings.
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
