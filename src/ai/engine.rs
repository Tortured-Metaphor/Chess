use crate::game::{Board, Color, Move, MoveGenerator, PieceType, Position};

pub struct ChessAI {
    pub depth: u32,
}

impl ChessAI {
    pub fn new(depth: u32) -> Self {
        ChessAI { depth }
    }

    pub fn get_best_move(&self, board: &Board) -> Option<Move> {
        let mut best_move = None;
        let mut best_score = -30000;
        let alpha = -30000;
        let beta = 30000;

        let pieces = board.get_all_pieces(board.current_player);
        let mut all_moves = Vec::new();

        for (pos, _) in pieces {
            let moves = MoveGenerator::generate_legal_moves(board, pos);
            all_moves.extend(moves);
        }

        // Order moves for better pruning (captures first)
        all_moves.sort_by_key(|mv| {
            if mv.captured.is_some() {
                0
            } else {
                1
            }
        });

        for mv in all_moves {
            let mut new_board = board.clone();
            new_board.make_move(&mv);

            let score = -self.minimax(&new_board, self.depth - 1, -beta, -alpha, false);

            if score > best_score {
                best_score = score;
                best_move = Some(mv);
            }
        }

        best_move
    }

    fn minimax(&self, board: &Board, depth: u32, mut alpha: i32, beta: i32, maximizing: bool) -> i32 {
        // Check for terminal conditions
        if depth == 0 {
            return self.evaluate(board);
        }

        if MoveGenerator::is_checkmate(board, board.current_player) {
            return -20000 - depth as i32; // Prefer quick checkmates
        }

        if MoveGenerator::is_stalemate(board, board.current_player)
            || MoveGenerator::is_insufficient_material(board)
            || board.halfmove_clock >= 50 {
            return 0;
        }

        let pieces = board.get_all_pieces(board.current_player);
        let mut all_moves = Vec::new();

        for (pos, _) in pieces {
            let moves = MoveGenerator::generate_legal_moves(board, pos);
            all_moves.extend(moves);
        }

        if all_moves.is_empty() {
            return 0; // Stalemate
        }

        // Order moves for better pruning
        all_moves.sort_by_key(|mv| {
            if mv.captured.is_some() {
                -mv.captured.unwrap().value()
            } else {
                0
            }
        });

        let mut best_score = -30000;

        for mv in all_moves {
            let mut new_board = board.clone();
            new_board.make_move(&mv);

            let score = -self.minimax(&new_board, depth - 1, -beta, -alpha, !maximizing);

            best_score = best_score.max(score);
            alpha = alpha.max(score);

            if alpha >= beta {
                break; // Beta cutoff
            }
        }

        best_score
    }

    fn evaluate(&self, board: &Board) -> i32 {
        let mut score = 0;

        // Material evaluation
        for row in 0..8 {
            for col in 0..8 {
                if let Some(piece) = board.squares[row][col] {
                    let piece_value = piece.value();
                    let position_value = self.get_position_value(piece.piece_type, Position::new(row, col), piece.color);

                    let total_value = piece_value + position_value;

                    if piece.color == board.current_player {
                        score += total_value;
                    } else {
                        score -= total_value;
                    }
                }
            }
        }

        // Mobility bonus
        let our_pieces = board.get_all_pieces(board.current_player);
        let mut our_mobility = 0;
        for (pos, _) in our_pieces {
            our_mobility += MoveGenerator::generate_legal_moves(board, pos).len();
        }
        score += our_mobility as i32 * 2;

        // Check bonus
        if MoveGenerator::is_in_check(board, board.current_player.opposite()) {
            score += 50;
        }

        score
    }

    fn get_position_value(&self, piece_type: PieceType, pos: Position, color: Color) -> i32 {
        // Flip position for black pieces
        let row = if color == Color::White { 7 - pos.row } else { pos.row };

        match piece_type {
            PieceType::Pawn => {
                let pawn_table = [
                    0,  0,  0,  0,  0,  0,  0,  0,
                    50, 50, 50, 50, 50, 50, 50, 50,
                    10, 10, 20, 30, 30, 20, 10, 10,
                    5,  5, 10, 25, 25, 10,  5,  5,
                    0,  0,  0, 20, 20,  0,  0,  0,
                    5, -5,-10,  0,  0,-10, -5,  5,
                    5, 10, 10,-20,-20, 10, 10,  5,
                    0,  0,  0,  0,  0,  0,  0,  0,
                ];
                pawn_table[row * 8 + pos.col]
            }
            PieceType::Knight => {
                let knight_table = [
                    -50,-40,-30,-30,-30,-30,-40,-50,
                    -40,-20,  0,  0,  0,  0,-20,-40,
                    -30,  0, 10, 15, 15, 10,  0,-30,
                    -30,  5, 15, 20, 20, 15,  5,-30,
                    -30,  0, 15, 20, 20, 15,  0,-30,
                    -30,  5, 10, 15, 15, 10,  5,-30,
                    -40,-20,  0,  5,  5,  0,-20,-40,
                    -50,-40,-30,-30,-30,-30,-40,-50,
                ];
                knight_table[row * 8 + pos.col]
            }
            PieceType::Bishop => {
                let bishop_table = [
                    -20,-10,-10,-10,-10,-10,-10,-20,
                    -10,  0,  0,  0,  0,  0,  0,-10,
                    -10,  0,  5, 10, 10,  5,  0,-10,
                    -10,  5,  5, 10, 10,  5,  5,-10,
                    -10,  0, 10, 10, 10, 10,  0,-10,
                    -10, 10, 10, 10, 10, 10, 10,-10,
                    -10,  5,  0,  0,  0,  0,  5,-10,
                    -20,-10,-10,-10,-10,-10,-10,-20,
                ];
                bishop_table[row * 8 + pos.col]
            }
            PieceType::Rook => {
                let rook_table = [
                    0,  0,  0,  0,  0,  0,  0,  0,
                    5, 10, 10, 10, 10, 10, 10,  5,
                    -5,  0,  0,  0,  0,  0,  0, -5,
                    -5,  0,  0,  0,  0,  0,  0, -5,
                    -5,  0,  0,  0,  0,  0,  0, -5,
                    -5,  0,  0,  0,  0,  0,  0, -5,
                    -5,  0,  0,  0,  0,  0,  0, -5,
                    0,  0,  0,  5,  5,  0,  0,  0,
                ];
                rook_table[row * 8 + pos.col]
            }
            PieceType::Queen => {
                let queen_table = [
                    -20,-10,-10, -5, -5,-10,-10,-20,
                    -10,  0,  0,  0,  0,  0,  0,-10,
                    -10,  0,  5,  5,  5,  5,  0,-10,
                    -5,  0,  5,  5,  5,  5,  0, -5,
                    0,  0,  5,  5,  5,  5,  0, -5,
                    -10,  5,  5,  5,  5,  5,  0,-10,
                    -10,  0,  5,  0,  0,  0,  0,-10,
                    -20,-10,-10, -5, -5,-10,-10,-20,
                ];
                queen_table[row * 8 + pos.col]
            }
            PieceType::King => {
                let king_table = [
                    -30,-40,-40,-50,-50,-40,-40,-30,
                    -30,-40,-40,-50,-50,-40,-40,-30,
                    -30,-40,-40,-50,-50,-40,-40,-30,
                    -30,-40,-40,-50,-50,-40,-40,-30,
                    -20,-30,-30,-40,-40,-30,-30,-20,
                    -10,-20,-20,-20,-20,-20,-20,-10,
                    20, 20,  0,  0,  0,  0, 20, 20,
                    20, 30, 10,  0,  0, 10, 30, 20,
                ];
                king_table[row * 8 + pos.col]
            }
        }
    }
}
