use std::io::{self, Write};
use rand::seq::SliceRandom;
use crate::explore::{ChessMove, MoveSequence};
use crate::opening::Opening;

/// Handles the interactive play loop for practicing an opening
pub struct PlaySession<'a> {
    opening: &'a Opening,
    move_sequence: MoveSequence,
    current_sequence: String,
}

impl<'a> PlaySession<'a> {
    pub fn new(opening: &'a Opening) -> Self {
        Self {
            opening,
            move_sequence: MoveSequence::new(),
            current_sequence: String::new(),
        }
    }

    /// Starts an interactive play loop for practicing the opening
    pub fn run(&mut self) -> io::Result<()> {
        println!("\nStarting practice game. Available commands:");
        println!("  move <chess move> - Make a move (e.g., 'move e4')");
        println!("  explore          - Open current position in LiChess");
        println!("  stop            - End practice game");
        println!();

        loop {
            print!("(practice) > ");
            io::stdout().flush()?;

            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
            let input = input.trim();

            match input {
                "stop" => break,
                "explore" => self.handle_explore()?,
                cmd if cmd.starts_with("move ") => self.handle_move(cmd)?,
                _ => println!("\nUnknown command. Type 'move <chess move>', 'explore', or 'stop'.\n"),
            }
        }

        Ok(())
    }

    fn handle_explore(&self) -> io::Result<()> {
        let url = self.move_sequence.to_lichess_url();
        println!("\nOpen this URL in your browser to explore the position:");
        println!("{}\n", url);
        Ok(())
    }

    fn handle_move(&mut self, cmd: &str) -> io::Result<()> {
        let mv = cmd.strip_prefix("move ").unwrap().trim();
        let chess_move = ChessMove(mv.to_string());
        self.move_sequence.add_move(chess_move.clone());

        // Update current sequence
        if !self.current_sequence.is_empty() {
            self.current_sequence.push('-');
        }
        self.current_sequence.push_str(mv);

        // Check if this sequence exists in our opening
        if let Some(note) = self.opening.moves.get(&self.current_sequence) {
            println!("\nYour move: {}", mv);
            println!("Note: {}", note.note);

            // Find all possible next moves
            let mut possible_moves = Vec::new();
            for (seq, _) in &self.opening.moves {
                if seq.starts_with(&self.current_sequence) && seq != &self.current_sequence {
                    if let Some(next_move) = seq.strip_prefix(&self.current_sequence)
                        .and_then(|s| s.strip_prefix("-"))
                    {
                        possible_moves.push(next_move);
                    }
                }
            }

            // Choose a random response if available
            if let Some(&response) = possible_moves.choose(&mut rand::thread_rng()) {
                let response_move = ChessMove(response.to_string());
                self.move_sequence.add_move(response_move.clone());
                self.current_sequence.push('-');
                self.current_sequence.push_str(response);

                println!("\nComputer plays: {}", response);
                if let Some(note) = self.opening.moves.get(&self.current_sequence) {
                    println!("Note: {}", note.note);
                }
            }
        } else {
            println!("\nDeviation! This is not a recorded move in the opening.");
        }
        println!();

        Ok(())
    }
}
