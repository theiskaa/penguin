use clap::{Parser, Subcommand};
use penguin::{mixer::ComplexityLevel, Penguin};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Generate passwords
    #[command(alias = "g")]
    Generate {
        /// Words to use for password generation (comma-separated)
        #[arg(short = 'w', long, value_delimiter = ',')]
        words: Vec<String>,

        /// Number of passwords to generate
        #[arg(short = 'n', long, default_value_t = 1)]
        number: usize,

        /// Complexity level (basic, medium, hard, penguin)
        #[arg(short = 'c', long, default_value = "basic")]
        complexity: String,

        /// Use whole words
        #[arg(short = 'u', long = "whole-words")]
        whole_words: bool,

        /// Password length
        #[arg(short = 'l', long)]
        length: Option<usize>,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Generate {
            words,
            number,
            complexity,
            whole_words,
            length,
        } => {
            let complexity_level = match complexity.to_lowercase().as_str() {
                "basic" => ComplexityLevel::Basic,
                "medium" => ComplexityLevel::Medium,
                "hard" => ComplexityLevel::Hard,
                "penguin" => ComplexityLevel::Penguin,
                _ => ComplexityLevel::Basic,
            };

            let penguin = Penguin::new(words.iter().map(|s| s.as_str()).collect());
            let passwords = penguin.generate_password(
                number,
                Some(complexity_level),
                Some(whole_words),
                length,
            );

            println!("\n> Generated passwords:");
            for (i, password) in passwords.iter().enumerate() {
                println!("   {}. {}", i + 1, password);
            }
            println!();
        }
    }
}
