use crate::ai::ChessAI;
use crate::game::{Board, Color, Move, MoveGenerator, PieceType, Position};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameMode {
    Menu,
    TwoPlayer,
    VsAI,
    GameOver,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameOverReason {
    Checkmate(Color), // Winner
    Stalemate,
    InsufficientMaterial,
    FiftyMoveRule,
}

pub struct App {
    pub board: Board,
    pub cursor: Position,
    pub selected_piece: Option<Position>,
    pub legal_moves: Vec<Move>,
    pub move_history: Vec<Move>,
    pub captured_white: Vec<PieceType>,
    pub captured_black: Vec<PieceType>,
    pub mode: GameMode,
    pub menu_selection: usize,
    pub ai: Option<ChessAI>,
    pub ai_color: Option<Color>,
    pub game_over_reason: Option<GameOverReason>,
    pub promotion_menu: Option<Position>,
    pub should_quit: bool,
}

impl App {
    pub fn new() -> Self {
        App {
            board: Board::new(),
            cursor: Position::new(6, 4), // Start at white king's pawn
            selected_piece: None,
            legal_moves: Vec::new(),
            move_history: Vec::new(),
            captured_white: Vec::new(),
            captured_black: Vec::new(),
            mode: GameMode::Menu,
            menu_selection: 0,
            ai: None,
            ai_color: None,
            game_over_reason: None,
            promotion_menu: None,
            should_quit: false,
        }
    }

    pub fn start_two_player(&mut self) {
        self.board = Board::new();
        self.cursor = Position::new(6, 4);
        self.selected_piece = None;
        self.legal_moves = Vec::new();
        self.move_history = Vec::new();
        self.captured_white = Vec::new();
        self.captured_black = Vec::new();
        self.mode = GameMode::TwoPlayer;
        self.ai = None;
        self.ai_color = None;
        self.game_over_reason = None;
        self.promotion_menu = None;
    }

    pub fn start_vs_ai(&mut self) {
        self.board = Board::new();
        self.cursor = Position::new(6, 4);
        self.selected_piece = None;
        self.legal_moves = Vec::new();
        self.move_history = Vec::new();
        self.captured_white = Vec::new();
        self.captured_black = Vec::new();
        self.mode = GameMode::VsAI;
        self.ai = Some(ChessAI::new(3)); // Depth 3 for reasonable speed
        self.ai_color = Some(Color::Black);
        self.game_over_reason = None;
        self.promotion_menu = None;
    }

    pub fn move_cursor(&mut self, row_offset: i32, col_offset: i32) {
        let new_row = (self.cursor.row as i32 + row_offset).clamp(0, 7) as usize;
        let new_col = (self.cursor.col as i32 + col_offset).clamp(0, 7) as usize;
        self.cursor = Position::new(new_row, new_col);
    }

    pub fn select_square(&mut self) {
        if let Some(_promo_pos) = self.promotion_menu {
            // Already handled in promotion selection
            return;
        }

        if let Some(_selected_pos) = self.selected_piece {
            // Try to make a move
            if let Some(mv) = self.legal_moves.iter().find(|m| m.to == self.cursor) {
                // Check if this is a promotion move
                if mv.piece.piece_type == PieceType::Pawn {
                    let promotion_row = if mv.piece.color == Color::White { 0 } else { 7 };
                    if mv.to.row == promotion_row {
                        self.promotion_menu = Some(mv.to);
                        return;
                    }
                }

                self.execute_move(*mv);
            } else {
                // Deselect or select a different piece
                self.try_select_piece();
            }
        } else {
            // Try to select a piece
            self.try_select_piece();
        }
    }

    pub fn execute_promotion(&mut self, piece_type: PieceType) {
        if let (Some(_selected_pos), Some(promo_pos)) = (self.selected_piece, self.promotion_menu) {
            if let Some(mv) = self.legal_moves.iter().find(|m| m.to == promo_pos) {
                let promo_move = Move {
                    from: mv.from,
                    to: mv.to,
                    move_type: crate::game::MoveType::Promotion(piece_type),
                    piece: mv.piece,
                    captured: mv.captured,
                };
                self.execute_move(promo_move);
                self.promotion_menu = None;
            }
        }
    }

    fn try_select_piece(&mut self) {
        if let Some(piece) = self.board.get_piece(self.cursor) {
            if piece.color == self.board.current_player {
                self.selected_piece = Some(self.cursor);
                self.legal_moves = MoveGenerator::generate_legal_moves(&self.board, self.cursor);
            } else {
                self.selected_piece = None;
                self.legal_moves = Vec::new();
            }
        } else {
            self.selected_piece = None;
            self.legal_moves = Vec::new();
        }
    }

    fn execute_move(&mut self, mv: Move) {
        // Track captured pieces
        if let Some(captured) = mv.captured {
            match captured.color {
                Color::White => self.captured_white.push(captured.piece_type),
                Color::Black => self.captured_black.push(captured.piece_type),
            }
        }

        self.move_history.push(mv);
        self.board.make_move(&mv);
        self.selected_piece = None;
        self.legal_moves = Vec::new();

        self.check_game_over();

        // If playing against AI and it's AI's turn, make AI move
        if self.mode == GameMode::VsAI && self.game_over_reason.is_none() {
            if Some(self.board.current_player) == self.ai_color {
                self.make_ai_move();
            }
        }
    }

    pub fn make_ai_move(&mut self) {
        if let Some(ref ai) = self.ai {
            if let Some(mv) = ai.get_best_move(&self.board) {
                // Track captured pieces
                if let Some(captured) = mv.captured {
                    match captured.color {
                        Color::White => self.captured_white.push(captured.piece_type),
                        Color::Black => self.captured_black.push(captured.piece_type),
                    }
                }

                self.move_history.push(mv);
                self.board.make_move(&mv);
                self.check_game_over();
            }
        }
    }

    fn check_game_over(&mut self) {
        if MoveGenerator::is_checkmate(&self.board, self.board.current_player) {
            self.game_over_reason = Some(GameOverReason::Checkmate(self.board.current_player.opposite()));
            self.mode = GameMode::GameOver;
        } else if MoveGenerator::is_stalemate(&self.board, self.board.current_player) {
            self.game_over_reason = Some(GameOverReason::Stalemate);
            self.mode = GameMode::GameOver;
        } else if MoveGenerator::is_insufficient_material(&self.board) {
            self.game_over_reason = Some(GameOverReason::InsufficientMaterial);
            self.mode = GameMode::GameOver;
        } else if self.board.halfmove_clock >= 50 {
            self.game_over_reason = Some(GameOverReason::FiftyMoveRule);
            self.mode = GameMode::GameOver;
        }
    }

    pub fn deselect(&mut self) {
        self.selected_piece = None;
        self.legal_moves = Vec::new();
        self.promotion_menu = None;
    }

    pub fn return_to_menu(&mut self) {
        self.mode = GameMode::Menu;
        self.menu_selection = 0;
    }

    pub fn quit(&mut self) {
        self.should_quit = true;
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}
