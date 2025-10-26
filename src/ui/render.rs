use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, List, ListItem, Paragraph, Widget},
    Frame,
};

use super::app::{App, GameMode, GameOverReason};
use crate::game::{Color as PieceColor, MoveGenerator, Position};

pub fn render(app: &App, frame: &mut Frame) {
    match app.mode {
        GameMode::Menu => render_menu(app, frame),
        GameMode::TwoPlayer | GameMode::VsAI => render_game(app, frame),
        GameMode::GameOver => render_game_over(app, frame),
    }
}

fn render_menu(app: &App, frame: &mut Frame) {
    let area = frame.area();

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(10),
            Constraint::Length(3),
        ])
        .split(area);

    // Title
    let title = Paragraph::new("♔ CHESS ♔")
        .style(Style::default().fg(Color::Green).add_modifier(Modifier::BOLD))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL).border_style(Style::default().fg(Color::White)));

    frame.render_widget(title, chunks[0]);

    // Menu options
    let menu_items = vec![
        "Two Player",
        "Play vs AI",
        "Quit",
    ];

    let items: Vec<ListItem> = menu_items
        .iter()
        .enumerate()
        .map(|(i, item)| {
            let style = if i == app.menu_selection {
                Style::default().fg(Color::Black).bg(Color::Green).add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::White)
            };
            ListItem::new(*item).style(style)
        })
        .collect();

    let menu = List::new(items)
        .block(Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::White))
            .title("Select Mode"));

    frame.render_widget(menu, chunks[1]);

    // Instructions
    let instructions = Paragraph::new("↑/↓: Navigate | Enter: Select | Q: Quit")
        .style(Style::default().fg(Color::Green))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL).border_style(Style::default().fg(Color::White)));

    frame.render_widget(instructions, chunks[2]);
}

fn render_game(app: &App, frame: &mut Frame) {
    let area = frame.area();

    let main_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(75), Constraint::Percentage(25)])
        .split(area);

    let left_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(85), Constraint::Percentage(15)])
        .split(main_chunks[0]);

    // Render board
    render_board(app, frame, left_chunks[0]);

    // Render status
    render_status(app, frame, left_chunks[1]);

    // Right panel
    let right_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(10),
            Constraint::Min(10),
        ])
        .split(main_chunks[1]);

    // Render captured pieces
    render_captured(app, frame, right_chunks[0]);

    // Render move history
    render_move_history(app, frame, right_chunks[1]);

    // Render promotion menu if active
    if app.promotion_menu.is_some() {
        render_promotion_menu(app, frame, area);
    }
}

fn render_board(app: &App, frame: &mut Frame, area: Rect) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::White))
        .title("Chess Board");

    let inner = block.inner(area);
    frame.render_widget(block, area);

    // Calculate board dimensions - make it fill most of the space
    // Use 90% of available space, ensuring square cells
    let max_cell_width = inner.width / 8;
    let max_cell_height = inner.height / 8;

    // Make cells square-ish (terminal chars are taller than wide, so use width * 2)
    let cell_height = max_cell_height.max(3);
    let cell_width = (cell_height * 2).min(max_cell_width);

    let board_width = cell_width * 8;
    let board_height = cell_height * 8;

    let board_area = Rect {
        x: inner.x + (inner.width.saturating_sub(board_width)) / 2,
        y: inner.y + (inner.height.saturating_sub(board_height)) / 2,
        width: board_width,
        height: board_height,
    };

    let mut board_widget = BoardWidget {
        app,
        cell_width,
        cell_height,
    };

    frame.render_widget(&mut board_widget, board_area);
}

struct BoardWidget<'a> {
    app: &'a App,
    cell_width: u16,
    cell_height: u16,
}

impl<'a> Widget for &mut BoardWidget<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        for row in 0..8 {
            for col in 0..8 {
                let pos = Position::new(row, col);
                let x = area.x + col as u16 * self.cell_width;
                let y = area.y + row as u16 * self.cell_height;
                let cell_area = Rect {
                    x,
                    y,
                    width: self.cell_width,
                    height: self.cell_height,
                };

                self.render_cell(pos, cell_area, buf);
            }
        }
    }
}

impl<'a> BoardWidget<'a> {
    fn render_cell(&self, pos: Position, area: Rect, buf: &mut Buffer) {
        let is_light_square = (pos.row + pos.col) % 2 == 0;
        let is_cursor = pos == self.app.cursor;
        let is_selected = Some(pos) == self.app.selected_piece;
        let is_legal_move = self.app.legal_moves.iter().any(|m| m.to == pos);

        // Determine background color
        let bg_color = if is_cursor {
            Color::Green
        } else if is_selected {
            Color::Rgb(0, 100, 0) // Dark green
        } else if is_legal_move {
            Color::Rgb(0, 150, 0) // Medium green for legal moves
        } else if is_light_square {
            Color::White
        } else {
            Color::Black
        };

        // Determine foreground color
        let fg_color = if is_light_square && !is_cursor && !is_selected && !is_legal_move {
            Color::Black
        } else {
            Color::White
        };

        // Fill cell background
        for dy in 0..area.height {
            for dx in 0..area.width {
                let x = area.x + dx;
                let y = area.y + dy;
                if x < buf.area.width && y < buf.area.height {
                    buf[(x, y)].set_bg(bg_color);
                }
            }
        }

        // Render piece
        if let Some(piece) = self.app.board.get_piece(pos) {
            let piece_fg = match piece.color {
                PieceColor::White => {
                    if is_cursor || is_selected {
                        Color::White // Bright white on green for selected
                    } else {
                        Color::Rgb(255, 215, 0) // Gold color for white pieces
                    }
                }
                PieceColor::Black => {
                    if is_cursor || is_selected {
                        Color::White // Bright white on green for selected
                    } else {
                        Color::Rgb(0, 180, 255) // Bright blue for black pieces
                    }
                }
            };

            // Use ASCII art if cell is large enough (height >= 3)
            if area.height >= 3 {
                let art = piece.piece_type.ascii_art();
                let art_height = art.len() as u16;

                // Calculate consistent starting position for the whole piece
                let max_line_width = art.iter().map(|s| s.len()).max().unwrap_or(0) as u16;
                let start_x = area.x + (area.width.saturating_sub(max_line_width)) / 2;
                let start_y = area.y + (area.height.saturating_sub(art_height)) / 2;

                for (i, line) in art.iter().enumerate() {
                    let y = start_y + i as u16;
                    if y >= buf.area.height || y >= area.y + area.height {
                        break;
                    }

                    // Render the line starting at consistent x position
                    for (j, ch) in line.chars().enumerate() {
                        let x = start_x + j as u16;
                        if x < buf.area.width && x < area.x + area.width && y < buf.area.height {
                            buf[(x, y)]
                                .set_char(ch)
                                .set_fg(piece_fg)
                                .set_bg(bg_color)
                                .set_style(Style::default().add_modifier(Modifier::BOLD));
                        }
                    }
                }
            } else {
                // Fall back to single character for small cells
                let symbol = piece.symbol();
                let center_x = area.x + area.width / 2;
                let center_y = area.y + area.height / 2;

                if center_x < buf.area.width && center_y < buf.area.height {
                    buf[(center_x, center_y)]
                        .set_char(symbol)
                        .set_fg(piece_fg)
                        .set_bg(bg_color)
                        .set_style(Style::default().add_modifier(Modifier::BOLD));
                }
            }
        } else if is_legal_move {
            // Show dot for legal move squares
            let center_x = area.x + area.width / 2;
            let center_y = area.y + area.height / 2;

            if center_x < buf.area.width && center_y < buf.area.height {
                buf[(center_x, center_y)]
                    .set_char('●')
                    .set_fg(Color::White)
                    .set_bg(bg_color);
            }
        }

        // Add file/rank labels on edges
        if pos.row == 7 && area.y + area.height < buf.area.height {
            let file_label = (b'a' + pos.col as u8) as char;
            buf[(area.x + area.width / 2, area.y + area.height - 1)]
                .set_char(file_label)
                .set_fg(fg_color)
                .set_bg(bg_color);
        }

        if pos.col == 0 && area.x > 0 {
            let rank_label = (b'8' - pos.row as u8) as char;
            buf[(area.x, area.y)]
                .set_char(rank_label)
                .set_fg(fg_color)
                .set_bg(bg_color);
        }
    }
}

fn render_status(app: &App, frame: &mut Frame, area: Rect) {
    let current_player = match app.board.current_player {
        PieceColor::White => "White",
        PieceColor::Black => "Black",
    };

    let in_check = MoveGenerator::is_in_check(&app.board, app.board.current_player);
    let check_text = if in_check { " (CHECK!)" } else { "" };

    let status_text = format!("Current Player: {}{}", current_player, check_text);

    let mut lines = vec![
        Line::from(status_text),
        Line::from(format!("Move: {}", app.board.fullmove_number)),
    ];

    if app.mode == GameMode::VsAI {
        let ai_player = if app.ai_color == Some(PieceColor::Black) { "Black" } else { "White" };
        lines.push(Line::from(format!("AI: {}", ai_player)));
    }

    let status = Paragraph::new(lines)
        .style(Style::default().fg(Color::White))
        .block(Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Green))
            .title("Status"));

    frame.render_widget(status, area);
}

fn render_captured(app: &App, frame: &mut Frame, area: Rect) {
    let white_captured: String = app.captured_white.iter()
        .map(|p| p.symbol())
        .collect();
    let black_captured: String = app.captured_black.iter()
        .map(|p| p.symbol())
        .collect();

    let text = vec![
        Line::from(vec![
            Span::styled("White: ", Style::default().fg(Color::White)),
            Span::raw(&white_captured),
        ]),
        Line::from(vec![
            Span::styled("Black: ", Style::default().fg(Color::White)),
            Span::raw(&black_captured),
        ]),
    ];

    let captured = Paragraph::new(text)
        .block(Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::White))
            .title("Captured"));

    frame.render_widget(captured, area);
}

fn render_move_history(app: &App, frame: &mut Frame, area: Rect) {
    let items: Vec<ListItem> = app.move_history
        .iter()
        .enumerate()
        .map(|(i, mv)| {
            let move_num = (i / 2) + 1;
            let move_text = if i % 2 == 0 {
                format!("{}. {}", move_num, mv.to_algebraic())
            } else {
                format!("   {}..{}", move_num, mv.to_algebraic())
            };
            ListItem::new(move_text).style(Style::default().fg(Color::White))
        })
        .rev()
        .take(area.height as usize - 2)
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
        .collect();

    let history = List::new(items)
        .block(Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::White))
            .title("Move History"));

    frame.render_widget(history, area);
}

fn render_promotion_menu(_app: &App, frame: &mut Frame, area: Rect) {
    let popup_area = Rect {
        x: area.width / 2 - 15,
        y: area.height / 2 - 5,
        width: 30,
        height: 10,
    };

    let text = vec![
        Line::from("Promote pawn to:").alignment(Alignment::Center),
        Line::from(""),
        Line::from("Q - Queen").alignment(Alignment::Center),
        Line::from("R - Rook").alignment(Alignment::Center),
        Line::from("B - Bishop").alignment(Alignment::Center),
        Line::from("N - Knight").alignment(Alignment::Center),
    ];

    let popup = Paragraph::new(text)
        .style(Style::default().fg(Color::White).bg(Color::Black))
        .block(Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Green))
            .title("Promotion"));

    frame.render_widget(popup, popup_area);
}

fn render_game_over(app: &App, frame: &mut Frame) {
    // First render the game board in the background
    render_game(app, frame);

    // Then render game over popup on top
    let area = frame.area();
    let popup_area = Rect {
        x: area.width / 2 - 20,
        y: area.height / 2 - 6,
        width: 40,
        height: 12,
    };

    let message = match app.game_over_reason {
        Some(GameOverReason::Checkmate(winner)) => {
            let winner_str = match winner {
                PieceColor::White => "White",
                PieceColor::Black => "Black",
            };
            format!("Checkmate!\n\n{} wins!", winner_str)
        }
        Some(GameOverReason::Stalemate) => "Stalemate!\n\nGame is a draw.".to_string(),
        Some(GameOverReason::InsufficientMaterial) => "Insufficient Material!\n\nGame is a draw.".to_string(),
        Some(GameOverReason::FiftyMoveRule) => "Fifty Move Rule!\n\nGame is a draw.".to_string(),
        None => "Game Over".to_string(),
    };

    let text = Text::from(format!("{}\n\nPress Enter to return to menu\nPress Q to quit", message));

    let popup = Paragraph::new(text)
        .alignment(Alignment::Center)
        .style(Style::default().fg(Color::White).bg(Color::Black))
        .block(Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Green).add_modifier(Modifier::BOLD))
            .title("Game Over"));

    frame.render_widget(popup, popup_area);
}
