use super::pieces::{Piece, PieceType, Position};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MoveType {
    Normal,
    Capture,
    EnPassant,
    Castle,
    Promotion(PieceType),
}

#[derive(Debug, Clone, Copy)]
pub struct Move {
    pub from: Position,
    pub to: Position,
    pub move_type: MoveType,
    pub piece: Piece,
    pub captured: Option<Piece>,
}

impl Move {
    pub fn new(from: Position, to: Position, piece: Piece) -> Self {
        Move {
            from,
            to,
            move_type: MoveType::Normal,
            piece,
            captured: None,
        }
    }

    pub fn with_capture(from: Position, to: Position, piece: Piece, captured: Piece) -> Self {
        Move {
            from,
            to,
            move_type: MoveType::Capture,
            piece,
            captured: Some(captured),
        }
    }

    pub fn with_type(mut self, move_type: MoveType) -> Self {
        self.move_type = move_type;
        self
    }

    pub fn to_algebraic(&self) -> String {
        let piece_symbol = match self.piece.piece_type {
            PieceType::Pawn => "",
            PieceType::Knight => "N",
            PieceType::Bishop => "B",
            PieceType::Rook => "R",
            PieceType::Queen => "Q",
            PieceType::King => "K",
        };

        match self.move_type {
            MoveType::Castle => {
                if self.to.col > self.from.col {
                    "O-O".to_string()
                } else {
                    "O-O-O".to_string()
                }
            }
            MoveType::Promotion(promoted_to) => {
                let promo_symbol = match promoted_to {
                    PieceType::Knight => "N",
                    PieceType::Bishop => "B",
                    PieceType::Rook => "R",
                    PieceType::Queen => "Q",
                    _ => "",
                };
                let capture = if self.captured.is_some() { "x" } else { "" };
                format!("{}{}{}={}", piece_symbol, capture, self.to.to_algebraic(), promo_symbol)
            }
            _ => {
                let capture = if self.captured.is_some() {
                    if self.piece.piece_type == PieceType::Pawn {
                        format!("{}x", (b'a' + self.from.col as u8) as char)
                    } else {
                        "x".to_string()
                    }
                } else {
                    String::new()
                };
                format!("{}{}{}", piece_symbol, capture, self.to.to_algebraic())
            }
        }
    }
}
