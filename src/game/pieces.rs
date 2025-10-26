use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Color {
    White,
    Black,
}

impl Color {
    pub fn opposite(&self) -> Color {
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

impl PieceType {
    pub fn value(&self) -> i32 {
        match self {
            PieceType::Pawn => 100,
            PieceType::Knight => 320,
            PieceType::Bishop => 330,
            PieceType::Rook => 500,
            PieceType::Queen => 900,
            PieceType::King => 20000,
        }
    }

    pub fn symbol(&self) -> char {
        match self {
            PieceType::Pawn => '♟',
            PieceType::Knight => '♞',
            PieceType::Bishop => '♝',
            PieceType::Rook => '♜',
            PieceType::Queen => '♛',
            PieceType::King => '♚',
        }
    }

    pub fn ascii_art(&self) -> Vec<&'static str> {
        match self {
            PieceType::Pawn => vec![
                " ● ",
                "▐█▌",
                "███",
            ],
            PieceType::Knight => vec![
                "▐▀▌",
                "▐█▌",
                "███",
            ],
            PieceType::Bishop => vec![
                " ◆ ",
                "▐█▌",
                "███",
            ],
            PieceType::Rook => vec![
                "█▀█",
                "▐█▌",
                "███",
            ],
            PieceType::Queen => vec![
                "◆◆◆",
                "▐█▌",
                "███",
            ],
            PieceType::King => vec![
                " + ",
                "▐█▌",
                "███",
            ],
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Piece {
    pub piece_type: PieceType,
    pub color: Color,
}

impl Piece {
    pub fn new(piece_type: PieceType, color: Color) -> Self {
        Piece { piece_type, color }
    }

    pub fn symbol(&self) -> char {
        self.piece_type.symbol()
    }

    pub fn value(&self) -> i32 {
        self.piece_type.value()
    }
}

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.symbol())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Position {
    pub row: usize,
    pub col: usize,
}

impl Position {
    pub fn new(row: usize, col: usize) -> Self {
        Position { row, col }
    }

    pub fn is_valid(&self) -> bool {
        self.row < 8 && self.col < 8
    }

    pub fn to_algebraic(&self) -> String {
        let file = (b'a' + self.col as u8) as char;
        let rank = (b'1' + (7 - self.row) as u8) as char;
        format!("{}{}", file, rank)
    }

    #[allow(dead_code)]
    pub fn from_algebraic(s: &str) -> Option<Self> {
        let bytes = s.as_bytes();
        if bytes.len() != 2 {
            return None;
        }
        let col = (bytes[0] as i32 - 'a' as i32) as usize;
        let row = 7 - (bytes[1] as i32 - '1' as i32) as usize;
        let pos = Position::new(row, col);
        if pos.is_valid() {
            Some(pos)
        } else {
            None
        }
    }
}
