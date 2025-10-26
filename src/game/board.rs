use super::pieces::{Color, Piece, PieceType, Position};
use super::moves::{Move, MoveType};

#[derive(Clone)]
pub struct Board {
    pub squares: [[Option<Piece>; 8]; 8],
    pub current_player: Color,
    pub en_passant_target: Option<Position>,
    pub castling_rights: CastlingRights,
    pub halfmove_clock: u32,
    pub fullmove_number: u32,
}

#[derive(Clone, Copy)]
pub struct CastlingRights {
    pub white_kingside: bool,
    pub white_queenside: bool,
    pub black_kingside: bool,
    pub black_queenside: bool,
}

impl CastlingRights {
    pub fn new() -> Self {
        CastlingRights {
            white_kingside: true,
            white_queenside: true,
            black_kingside: true,
            black_queenside: true,
        }
    }
}

impl Board {
    pub fn new() -> Self {
        let mut board = Board {
            squares: [[None; 8]; 8],
            current_player: Color::White,
            en_passant_target: None,
            castling_rights: CastlingRights::new(),
            halfmove_clock: 0,
            fullmove_number: 1,
        };
        board.setup_initial_position();
        board
    }

    fn setup_initial_position(&mut self) {
        // Place pawns
        for col in 0..8 {
            self.squares[1][col] = Some(Piece::new(PieceType::Pawn, Color::Black));
            self.squares[6][col] = Some(Piece::new(PieceType::Pawn, Color::White));
        }

        // Place black pieces
        self.squares[0][0] = Some(Piece::new(PieceType::Rook, Color::Black));
        self.squares[0][1] = Some(Piece::new(PieceType::Knight, Color::Black));
        self.squares[0][2] = Some(Piece::new(PieceType::Bishop, Color::Black));
        self.squares[0][3] = Some(Piece::new(PieceType::Queen, Color::Black));
        self.squares[0][4] = Some(Piece::new(PieceType::King, Color::Black));
        self.squares[0][5] = Some(Piece::new(PieceType::Bishop, Color::Black));
        self.squares[0][6] = Some(Piece::new(PieceType::Knight, Color::Black));
        self.squares[0][7] = Some(Piece::new(PieceType::Rook, Color::Black));

        // Place white pieces
        self.squares[7][0] = Some(Piece::new(PieceType::Rook, Color::White));
        self.squares[7][1] = Some(Piece::new(PieceType::Knight, Color::White));
        self.squares[7][2] = Some(Piece::new(PieceType::Bishop, Color::White));
        self.squares[7][3] = Some(Piece::new(PieceType::Queen, Color::White));
        self.squares[7][4] = Some(Piece::new(PieceType::King, Color::White));
        self.squares[7][5] = Some(Piece::new(PieceType::Bishop, Color::White));
        self.squares[7][6] = Some(Piece::new(PieceType::Knight, Color::White));
        self.squares[7][7] = Some(Piece::new(PieceType::Rook, Color::White));
    }

    pub fn get_piece(&self, pos: Position) -> Option<Piece> {
        if pos.is_valid() {
            self.squares[pos.row][pos.col]
        } else {
            None
        }
    }

    pub fn set_piece(&mut self, pos: Position, piece: Option<Piece>) {
        if pos.is_valid() {
            self.squares[pos.row][pos.col] = piece;
        }
    }

    pub fn make_move(&mut self, mv: &Move) -> bool {
        // Validate the piece is still there
        if self.get_piece(mv.from) != Some(mv.piece) {
            return false;
        }

        // Reset halfmove clock on capture or pawn move
        if mv.captured.is_some() || mv.piece.piece_type == PieceType::Pawn {
            self.halfmove_clock = 0;
        } else {
            self.halfmove_clock += 1;
        }

        // Handle different move types
        match mv.move_type {
            MoveType::Normal | MoveType::Capture => {
                self.set_piece(mv.from, None);
                self.set_piece(mv.to, Some(mv.piece));
            }
            MoveType::EnPassant => {
                self.set_piece(mv.from, None);
                self.set_piece(mv.to, Some(mv.piece));
                // Remove the captured pawn
                let captured_pawn_row = mv.from.row;
                let captured_pawn_pos = Position::new(captured_pawn_row, mv.to.col);
                self.set_piece(captured_pawn_pos, None);
            }
            MoveType::Castle => {
                // Move king
                self.set_piece(mv.from, None);
                self.set_piece(mv.to, Some(mv.piece));

                // Move rook
                if mv.to.col > mv.from.col {
                    // Kingside castle
                    let rook = self.get_piece(Position::new(mv.from.row, 7));
                    self.set_piece(Position::new(mv.from.row, 7), None);
                    self.set_piece(Position::new(mv.from.row, 5), rook);
                } else {
                    // Queenside castle
                    let rook = self.get_piece(Position::new(mv.from.row, 0));
                    self.set_piece(Position::new(mv.from.row, 0), None);
                    self.set_piece(Position::new(mv.from.row, 3), rook);
                }
            }
            MoveType::Promotion(promoted_to) => {
                self.set_piece(mv.from, None);
                self.set_piece(mv.to, Some(Piece::new(promoted_to, mv.piece.color)));
            }
        }

        // Update en passant target
        self.en_passant_target = None;
        if mv.piece.piece_type == PieceType::Pawn {
            let row_diff = (mv.to.row as i32 - mv.from.row as i32).abs();
            if row_diff == 2 {
                let ep_row = (mv.from.row + mv.to.row) / 2;
                self.en_passant_target = Some(Position::new(ep_row, mv.from.col));
            }
        }

        // Update castling rights
        if mv.piece.piece_type == PieceType::King {
            match mv.piece.color {
                Color::White => {
                    self.castling_rights.white_kingside = false;
                    self.castling_rights.white_queenside = false;
                }
                Color::Black => {
                    self.castling_rights.black_kingside = false;
                    self.castling_rights.black_queenside = false;
                }
            }
        } else if mv.piece.piece_type == PieceType::Rook {
            match (mv.piece.color, mv.from.row, mv.from.col) {
                (Color::White, 7, 0) => self.castling_rights.white_queenside = false,
                (Color::White, 7, 7) => self.castling_rights.white_kingside = false,
                (Color::Black, 0, 0) => self.castling_rights.black_queenside = false,
                (Color::Black, 0, 7) => self.castling_rights.black_kingside = false,
                _ => {}
            }
        }

        // Update move counters
        if self.current_player == Color::Black {
            self.fullmove_number += 1;
        }

        // Switch players
        self.current_player = self.current_player.opposite();

        true
    }

    pub fn find_king(&self, color: Color) -> Option<Position> {
        for row in 0..8 {
            for col in 0..8 {
                if let Some(piece) = self.squares[row][col] {
                    if piece.color == color && piece.piece_type == PieceType::King {
                        return Some(Position::new(row, col));
                    }
                }
            }
        }
        None
    }

    pub fn get_all_pieces(&self, color: Color) -> Vec<(Position, Piece)> {
        let mut pieces = Vec::new();
        for row in 0..8 {
            for col in 0..8 {
                if let Some(piece) = self.squares[row][col] {
                    if piece.color == color {
                        pieces.push((Position::new(row, col), piece));
                    }
                }
            }
        }
        pieces
    }
}

impl Default for Board {
    fn default() -> Self {
        Self::new()
    }
}
