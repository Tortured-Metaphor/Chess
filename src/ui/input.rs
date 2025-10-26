use crossterm::event::{self, Event, KeyCode, KeyEvent};
use std::time::Duration;

use super::app::{App, GameMode};
use crate::game::PieceType;

pub fn handle_input(app: &mut App) -> std::io::Result<()> {
    if event::poll(Duration::from_millis(100))? {
        if let Event::Key(key) = event::read()? {
            match app.mode {
                GameMode::Menu => handle_menu_input(app, key),
                GameMode::TwoPlayer | GameMode::VsAI => handle_game_input(app, key),
                GameMode::GameOver => handle_game_over_input(app, key),
            }
        }
    }
    Ok(())
}

fn handle_menu_input(app: &mut App, key: KeyEvent) {
    match key.code {
        KeyCode::Up | KeyCode::Char('k') => {
            if app.menu_selection > 0 {
                app.menu_selection -= 1;
            }
        }
        KeyCode::Down | KeyCode::Char('j') => {
            if app.menu_selection < 2 {
                app.menu_selection += 1;
            }
        }
        KeyCode::Enter | KeyCode::Char(' ') => {
            match app.menu_selection {
                0 => app.start_two_player(),
                1 => app.start_vs_ai(),
                2 => app.quit(),
                _ => {}
            }
        }
        KeyCode::Char('q') | KeyCode::Esc => {
            app.quit();
        }
        _ => {}
    }
}

fn handle_game_input(app: &mut App, key: KeyEvent) {
    // Handle promotion menu if active
    if app.promotion_menu.is_some() {
        match key.code {
            KeyCode::Char('q') | KeyCode::Char('Q') => {
                app.execute_promotion(PieceType::Queen);
            }
            KeyCode::Char('r') | KeyCode::Char('R') => {
                app.execute_promotion(PieceType::Rook);
            }
            KeyCode::Char('b') | KeyCode::Char('B') => {
                app.execute_promotion(PieceType::Bishop);
            }
            KeyCode::Char('n') | KeyCode::Char('N') => {
                app.execute_promotion(PieceType::Knight);
            }
            KeyCode::Esc => {
                app.promotion_menu = None;
            }
            _ => {}
        }
        return;
    }

    match key.code {
        KeyCode::Up | KeyCode::Char('k') => {
            app.move_cursor(-1, 0);
        }
        KeyCode::Down | KeyCode::Char('j') => {
            app.move_cursor(1, 0);
        }
        KeyCode::Left | KeyCode::Char('h') => {
            app.move_cursor(0, -1);
        }
        KeyCode::Right | KeyCode::Char('l') => {
            app.move_cursor(0, 1);
        }
        KeyCode::Enter | KeyCode::Char(' ') => {
            app.select_square();
        }
        KeyCode::Esc => {
            app.deselect();
        }
        KeyCode::Char('m') => {
            app.return_to_menu();
        }
        KeyCode::Char('q') => {
            app.quit();
        }
        _ => {}
    }
}

fn handle_game_over_input(app: &mut App, key: KeyEvent) {
    match key.code {
        KeyCode::Enter | KeyCode::Char(' ') | KeyCode::Char('m') => {
            app.return_to_menu();
        }
        KeyCode::Char('q') | KeyCode::Esc => {
            app.quit();
        }
        _ => {}
    }
}
