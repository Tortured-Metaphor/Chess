use super::board::Board;
use super::moves::{Move, MoveType};
use super::pieces::{Color, Piece, PieceType, Position};

pub struct MoveGenerator;

impl MoveGenerator {
    pub fn generate_legal_moves(board: &Board, pos: Position) -> Vec<Move> {
        let piece = match board.get_piece(pos) {
            Some(p) if p.color == board.current_player => p,
            _ => return Vec::new(),
        };

        let mut moves = Self::generate_pseudo_legal_moves(board, pos, piece);

        // Filter out moves that would leave the king in check
        moves.retain(|mv| !Self::would_be_in_check(board, mv));

        moves
    }

    fn generate_pseudo_legal_moves(board: &Board, pos: Position, piece: Piece) -> Vec<Move> {
        match piece.piece_type {
            PieceType::Pawn => Self::generate_pawn_moves(board, pos, piece),
            PieceType::Knight => Self::generate_knight_moves(board, pos, piece),
            PieceType::Bishop => Self::generate_bishop_moves(board, pos, piece),
            PieceType::Rook => Self::generate_rook_moves(board, pos, piece),
            PieceType::Queen => Self::generate_queen_moves(board, pos, piece),
            PieceType::King => Self::generate_king_moves(board, pos, piece),
        }
    }

    fn generate_pawn_moves(board: &Board, pos: Position, piece: Piece) -> Vec<Move> {
        let mut moves = Vec::new();
        let direction: i32 = if piece.color == Color::White { -1 } else { 1 };
        let start_row = if piece.color == Color::White { 6 } else { 1 };
        let promotion_row = if piece.color == Color::White { 0 } else { 7 };

        // Forward move
        let new_row = (pos.row as i32 + direction) as usize;
        if new_row < 8 {
            let forward_pos = Position::new(new_row, pos.col);
            if board.get_piece(forward_pos).is_none() {
                if new_row == promotion_row {
                    // Promotion
                    for promo_type in [PieceType::Queen, PieceType::Rook, PieceType::Bishop, PieceType::Knight] {
                        moves.push(Move::new(pos, forward_pos, piece).with_type(MoveType::Promotion(promo_type)));
                    }
                } else {
                    moves.push(Move::new(pos, forward_pos, piece));
                }

                // Double move from start
                if pos.row == start_row {
                    let double_row = (pos.row as i32 + direction * 2) as usize;
                    let double_pos = Position::new(double_row, pos.col);
                    if board.get_piece(double_pos).is_none() {
                        moves.push(Move::new(pos, double_pos, piece));
                    }
                }
            }
        }

        // Captures
        for col_offset in [-1, 1] {
            let new_col = pos.col as i32 + col_offset;
            if new_col >= 0 && new_col < 8 {
                let new_row = (pos.row as i32 + direction) as usize;
                if new_row < 8 {
                    let capture_pos = Position::new(new_row, new_col as usize);

                    // Normal capture
                    if let Some(target) = board.get_piece(capture_pos) {
                        if target.color != piece.color {
                            if new_row == promotion_row {
                                for promo_type in [PieceType::Queen, PieceType::Rook, PieceType::Bishop, PieceType::Knight] {
                                    moves.push(Move::with_capture(pos, capture_pos, piece, target)
                                        .with_type(MoveType::Promotion(promo_type)));
                                }
                            } else {
                                moves.push(Move::with_capture(pos, capture_pos, piece, target));
                            }
                        }
                    }

                    // En passant
                    if Some(capture_pos) == board.en_passant_target {
                        let captured_pawn = Piece::new(PieceType::Pawn, piece.color.opposite());
                        moves.push(Move::with_capture(pos, capture_pos, piece, captured_pawn)
                            .with_type(MoveType::EnPassant));
                    }
                }
            }
        }

        moves
    }

    fn generate_knight_moves(board: &Board, pos: Position, piece: Piece) -> Vec<Move> {
        let mut moves = Vec::new();
        let offsets = [
            (-2, -1), (-2, 1), (-1, -2), (-1, 2),
            (1, -2), (1, 2), (2, -1), (2, 1),
        ];

        for (row_offset, col_offset) in offsets {
            let new_row = pos.row as i32 + row_offset;
            let new_col = pos.col as i32 + col_offset;

            if new_row >= 0 && new_row < 8 && new_col >= 0 && new_col < 8 {
                let new_pos = Position::new(new_row as usize, new_col as usize);
                match board.get_piece(new_pos) {
                    None => moves.push(Move::new(pos, new_pos, piece)),
                    Some(target) if target.color != piece.color => {
                        moves.push(Move::with_capture(pos, new_pos, piece, target));
                    }
                    _ => {}
                }
            }
        }

        moves
    }

    fn generate_sliding_moves(
        board: &Board,
        pos: Position,
        piece: Piece,
        directions: &[(i32, i32)],
    ) -> Vec<Move> {
        let mut moves = Vec::new();

        for &(row_dir, col_dir) in directions {
            let mut new_row = pos.row as i32 + row_dir;
            let mut new_col = pos.col as i32 + col_dir;

            while new_row >= 0 && new_row < 8 && new_col >= 0 && new_col < 8 {
                let new_pos = Position::new(new_row as usize, new_col as usize);
                match board.get_piece(new_pos) {
                    None => {
                        moves.push(Move::new(pos, new_pos, piece));
                    }
                    Some(target) if target.color != piece.color => {
                        moves.push(Move::with_capture(pos, new_pos, piece, target));
                        break;
                    }
                    _ => break,
                }
                new_row += row_dir;
                new_col += col_dir;
            }
        }

        moves
    }

    fn generate_bishop_moves(board: &Board, pos: Position, piece: Piece) -> Vec<Move> {
        let directions = [(-1, -1), (-1, 1), (1, -1), (1, 1)];
        Self::generate_sliding_moves(board, pos, piece, &directions)
    }

    fn generate_rook_moves(board: &Board, pos: Position, piece: Piece) -> Vec<Move> {
        let directions = [(-1, 0), (1, 0), (0, -1), (0, 1)];
        Self::generate_sliding_moves(board, pos, piece, &directions)
    }

    fn generate_queen_moves(board: &Board, pos: Position, piece: Piece) -> Vec<Move> {
        let directions = [
            (-1, -1), (-1, 0), (-1, 1),
            (0, -1),           (0, 1),
            (1, -1),  (1, 0),  (1, 1),
        ];
        Self::generate_sliding_moves(board, pos, piece, &directions)
    }

    fn generate_king_moves(board: &Board, pos: Position, piece: Piece) -> Vec<Move> {
        let mut moves = Vec::new();
        let offsets = [
            (-1, -1), (-1, 0), (-1, 1),
            (0, -1),           (0, 1),
            (1, -1),  (1, 0),  (1, 1),
        ];

        // Normal king moves
        for (row_offset, col_offset) in offsets {
            let new_row = pos.row as i32 + row_offset;
            let new_col = pos.col as i32 + col_offset;

            if new_row >= 0 && new_row < 8 && new_col >= 0 && new_col < 8 {
                let new_pos = Position::new(new_row as usize, new_col as usize);
                match board.get_piece(new_pos) {
                    None => moves.push(Move::new(pos, new_pos, piece)),
                    Some(target) if target.color != piece.color => {
                        moves.push(Move::with_capture(pos, new_pos, piece, target));
                    }
                    _ => {}
                }
            }
        }

        // Castling
        let (kingside, queenside, row) = match piece.color {
            Color::White => (
                board.castling_rights.white_kingside,
                board.castling_rights.white_queenside,
                7,
            ),
            Color::Black => (
                board.castling_rights.black_kingside,
                board.castling_rights.black_queenside,
                0,
            ),
        };

        if !Self::is_square_attacked(board, pos, piece.color.opposite()) {
            // Kingside castling
            if kingside && pos.col == 4 {
                let f1 = Position::new(row, 5);
                let g1 = Position::new(row, 6);
                if board.get_piece(f1).is_none()
                    && board.get_piece(g1).is_none()
                    && !Self::is_square_attacked(board, f1, piece.color.opposite())
                {
                    moves.push(Move::new(pos, g1, piece).with_type(MoveType::Castle));
                }
            }

            // Queenside castling
            if queenside && pos.col == 4 {
                let d1 = Position::new(row, 3);
                let c1 = Position::new(row, 2);
                let b1 = Position::new(row, 1);
                if board.get_piece(d1).is_none()
                    && board.get_piece(c1).is_none()
                    && board.get_piece(b1).is_none()
                    && !Self::is_square_attacked(board, d1, piece.color.opposite())
                {
                    moves.push(Move::new(pos, c1, piece).with_type(MoveType::Castle));
                }
            }
        }

        moves
    }

    fn would_be_in_check(board: &Board, mv: &Move) -> bool {
        let mut test_board = board.clone();
        test_board.make_move(mv);

        // Switch back to check the original player's king
        let king_color = mv.piece.color;
        if let Some(king_pos) = test_board.find_king(king_color) {
            Self::is_square_attacked(&test_board, king_pos, king_color.opposite())
        } else {
            true // King not found, invalid position
        }
    }

    pub fn is_square_attacked(board: &Board, pos: Position, by_color: Color) -> bool {
        // Check for pawn attacks
        let pawn_direction = if by_color == Color::White { -1 } else { 1 };
        for col_offset in [-1, 1] {
            let attack_row = (pos.row as i32 - pawn_direction) as usize;
            let attack_col = (pos.col as i32 + col_offset) as usize;
            if attack_row < 8 && attack_col < 8 {
                if let Some(piece) = board.squares[attack_row][attack_col] {
                    if piece.color == by_color && piece.piece_type == PieceType::Pawn {
                        return true;
                    }
                }
            }
        }

        // Check for knight attacks
        let knight_offsets = [
            (-2, -1), (-2, 1), (-1, -2), (-1, 2),
            (1, -2), (1, 2), (2, -1), (2, 1),
        ];
        for (row_offset, col_offset) in knight_offsets {
            let new_row = pos.row as i32 + row_offset;
            let new_col = pos.col as i32 + col_offset;
            if new_row >= 0 && new_row < 8 && new_col >= 0 && new_col < 8 {
                if let Some(piece) = board.squares[new_row as usize][new_col as usize] {
                    if piece.color == by_color && piece.piece_type == PieceType::Knight {
                        return true;
                    }
                }
            }
        }

        // Check for sliding piece attacks (bishop, rook, queen)
        let directions = [
            (-1, -1), (-1, 0), (-1, 1),
            (0, -1),           (0, 1),
            (1, -1),  (1, 0),  (1, 1),
        ];

        for (idx, &(row_dir, col_dir)) in directions.iter().enumerate() {
            let is_diagonal = idx % 2 == 0;
            let mut new_row = pos.row as i32 + row_dir;
            let mut new_col = pos.col as i32 + col_dir;

            while new_row >= 0 && new_row < 8 && new_col >= 0 && new_col < 8 {
                if let Some(piece) = board.squares[new_row as usize][new_col as usize] {
                    if piece.color == by_color {
                        match piece.piece_type {
                            PieceType::Queen => return true,
                            PieceType::Bishop if is_diagonal => return true,
                            PieceType::Rook if !is_diagonal => return true,
                            _ => break,
                        }
                    }
                    break;
                }
                new_row += row_dir;
                new_col += col_dir;
            }
        }

        // Check for king attacks
        let king_offsets = [
            (-1, -1), (-1, 0), (-1, 1),
            (0, -1),           (0, 1),
            (1, -1),  (1, 0),  (1, 1),
        ];
        for (row_offset, col_offset) in king_offsets {
            let new_row = pos.row as i32 + row_offset;
            let new_col = pos.col as i32 + col_offset;
            if new_row >= 0 && new_row < 8 && new_col >= 0 && new_col < 8 {
                if let Some(piece) = board.squares[new_row as usize][new_col as usize] {
                    if piece.color == by_color && piece.piece_type == PieceType::King {
                        return true;
                    }
                }
            }
        }

        false
    }

    pub fn is_in_check(board: &Board, color: Color) -> bool {
        if let Some(king_pos) = board.find_king(color) {
            Self::is_square_attacked(board, king_pos, color.opposite())
        } else {
            false
        }
    }

    pub fn is_checkmate(board: &Board, color: Color) -> bool {
        if !Self::is_in_check(board, color) {
            return false;
        }
        Self::has_no_legal_moves(board, color)
    }

    pub fn is_stalemate(board: &Board, color: Color) -> bool {
        if Self::is_in_check(board, color) {
            return false;
        }
        Self::has_no_legal_moves(board, color)
    }

    fn has_no_legal_moves(board: &Board, color: Color) -> bool {
        for row in 0..8 {
            for col in 0..8 {
                let pos = Position::new(row, col);
                if let Some(piece) = board.get_piece(pos) {
                    if piece.color == color {
                        let moves = Self::generate_legal_moves(board, pos);
                        if !moves.is_empty() {
                            return false;
                        }
                    }
                }
            }
        }
        true
    }

    pub fn is_insufficient_material(board: &Board) -> bool {
        let mut white_pieces = Vec::new();
        let mut black_pieces = Vec::new();

        for row in 0..8 {
            for col in 0..8 {
                if let Some(piece) = board.squares[row][col] {
                    match piece.color {
                        Color::White => white_pieces.push(piece.piece_type),
                        Color::Black => black_pieces.push(piece.piece_type),
                    }
                }
            }
        }

        // King vs King
        if white_pieces.len() == 1 && black_pieces.len() == 1 {
            return true;
        }

        // King and Bishop vs King
        // King and Knight vs King
        if (white_pieces.len() == 1 && black_pieces.len() == 2)
            || (white_pieces.len() == 2 && black_pieces.len() == 1)
        {
            let pieces = if white_pieces.len() == 2 { &white_pieces } else { &black_pieces };
            return pieces.contains(&PieceType::Bishop) || pieces.contains(&PieceType::Knight);
        }

        false
    }
}
