use std::collections::HashMap;
use std::fs;
use std::io::{self, Write};
use std::path::Path;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use rand::seq::SliceRandom;
use crate::explore::{ChessMove, MoveSequence};
use crate::play::PlaySession;
use crate::theme::Theme;

#[derive(Error, Debug)]
pub enum OpeningError {
    #[error("Invalid file extension. Expected .opening, got {0}")]
    InvalidExtension(String),
    
    #[error("Failed to read file: {0}")]
    FileReadError(#[from] io::Error),
    
    #[error("Failed to parse opening file: {0}")]
    ParseError(#[from] serde_json::Error),
}

/// Represents a single move sequence and its associated note
#[derive(Debug, Serialize, Deserialize)]
pub struct MoveNote {
    pub note: String,
}

/// Represents a complete opening study file
#[derive(Debug, Serialize, Deserialize)]
pub struct Opening {
    /// Author of the opening study
    pub author: String,
    
    /// Unix timestamp of when the file was last modified
    pub date_modified: u64,
    
    /// Name of the opening (e.g., "Caro Kann")
    pub name: String,
    
    /// Description of the opening study
    pub description: String,
    
    /// Map of move sequences to their notes
    /// Keys are move sequences in the format "e4-c6-d4-d5-e5"
    /// Values are the associated notes for that sequence
    pub moves: HashMap<String, MoveNote>,
}

impl Opening {
    /// Creates a new empty opening study
    pub fn new(name: String, author: String, description: String) -> Self {
        Self {
            author,
            date_modified: chrono::Utc::now().timestamp() as u64,
            name,
            description,
            moves: HashMap::new(),
        }
    }

    /// Loads an opening file from disk
    pub fn from_file(path: &Path) -> Result<Self, OpeningError> {
        // Validate file extension
        if path.extension().and_then(|ext| ext.to_str()) != Some("opening") {
            return Err(OpeningError::InvalidExtension(
                path.extension()
                    .and_then(|ext| ext.to_str())
                    .unwrap_or("none")
                    .to_string(),
            ));
        }

        // Read and parse the file
        let contents = fs::read_to_string(path)?;
        let opening = serde_json::from_str(&contents)?;
        Ok(opening)
    }

    /// Starts an interactive command loop for studying the opening
    pub fn study_loop(&self) -> io::Result<()> {
        let theme = Theme::new();
        
        println!("\n{}", theme.format_prompt(&format!("Studying: {}", self.name)));
        println!("{}", theme.format_prompt(&format!("Author: {}", self.author)));
        println!("{}", theme.format_prompt(&format!("Description: {}", self.description)));
        println!("\n{}", theme.format_prompt("Available commands:"));
        println!("  {} - List all move sequences", theme.format_prompt("list"));
        println!("  {} - Show notes for a specific sequence", theme.format_prompt("show <sequence>"));
        println!("  {} - Start a practice game", theme.format_prompt("play"));
        println!("  {} - Exit the study session", theme.format_prompt("quit"));
        println!();

        loop {
            print!("{}", theme.format_prompt("> "));
            io::stdout().flush()?;

            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
            let input = input.trim();

            match input {
                "list" => {
                    println!("\n{}", theme.format_prompt("Move sequences:"));
                    for sequence in self.moves.keys() {
                        println!("  {}", sequence);
                    }
                    println!();
                }
                "play" => {
                    let mut session = PlaySession::new(self);
                    if let Err(e) = session.run() {
                        eprintln!("{}", theme.format_prompt(&format!("Error during play session: {}", e)));
                    }
                }
                "quit" => break,
                cmd if cmd.starts_with("show ") => {
                    let sequence = cmd.strip_prefix("show ").unwrap().trim();
                    match self.moves.get(sequence) {
                        Some(note) => {
                            println!("\n{}", theme.format_prompt(&format!("Notes for {}:", sequence)));
                            println!("{}\n", theme.format_note(&note.note));
                        }
                        None => println!("\n{}", theme.format_prompt(&format!("No notes found for sequence: {}\n", sequence))),
                    }
                }
                _ => println!("\n{}", theme.format_prompt("Unknown command. Type 'list' to see available commands.\n")),
            }
        }

        Ok(())
    }
}
