use std::fmt::Write;

/// Represents a chess move in algebraic notation
#[derive(Debug, Clone)]
pub struct ChessMove(pub String);

/// Represents a sequence of chess moves
#[derive(Debug, Default)]
pub struct MoveSequence {
    moves: Vec<ChessMove>,
}

impl MoveSequence {
    pub fn new() -> Self {
        Self { moves: Vec::new() }
    }

    pub fn add_move(&mut self, mv: ChessMove) {
        self.moves.push(mv);
    }

    /// Converts the move sequence to PGN format
    pub fn to_pgn(&self) -> String {
        let mut pgn = String::new();
        
        // Group moves into pairs (white and black moves)
        for (i, chunk) in self.moves.chunks(2).enumerate() {
            let move_number = i + 1;
            write!(&mut pgn, "{}. ", move_number).unwrap();
            
            // Add white move
            if let Some(white_move) = chunk.get(0) {
                write!(&mut pgn, "{}", white_move.0).unwrap();
            }
            
            // Add black move if it exists
            if let Some(black_move) = chunk.get(1) {
                write!(&mut pgn, " {}", black_move.0).unwrap();
            }
            
            // Add space between move pairs
            pgn.push(' ');
        }
        
        pgn.trim().to_string()
    }

    /// Generates a LiChess analysis URL for the current position
    pub fn to_lichess_url(&self) -> String {
        let pgn = self.to_pgn();
        let encoded_pgn = urlencoding::encode(&pgn);
        format!("https://lichess.org/analysis/pgn/{}", encoded_pgn)
    }
}
