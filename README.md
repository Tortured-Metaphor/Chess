# Chess - Terminal User Interface

A complete, fully-functional chess game implemented in Rust with a beautiful TUI interface featuring ASCII art pieces and a black, white, and green color scheme.

![Version](https://img.shields.io/badge/version-1.0.0-blue.svg)

## Features

- **Full Chess Rules Implementation**
  - All standard piece movements (Pawn, Knight, Bishop, Rook, Queen, King)
  - Special moves: Castling, En Passant, Pawn Promotion
  - Check, Checkmate, and Stalemate detection
  - Draw conditions: Insufficient Material, 50-Move Rule

- **Game Modes**
  - Two-player local mode
  - Play against AI (Minimax algorithm with alpha-beta pruning)

- **User Interface**
  - **Large ASCII art pieces** for excellent visibility
  - Responsive layout that scales with terminal window size
  - Cursor-based piece selection with arrow keys
  - Legal move highlighting with green dots
  - Current selection highlighted in green
  - Move history panel with algebraic notation
  - Captured pieces display for both sides
  - Status panel showing current player and game state
  - Check/Checkmate/Stalemate notifications

- **Color Scheme**
  - White and black chessboard squares
  - Green highlighting for cursor, selected pieces, and legal moves
  - Gold pieces for white player, bright blue pieces for black player
  - High contrast design for easy visibility

## Installation

Make sure you have Rust installed. Then:

```bash
cargo build --release
```

## Running the Game

```bash
cargo run --release
```

## Controls

### Menu Navigation
- `↑/↓` or `k/j`: Navigate menu
- `Enter` or `Space`: Select option
- `q`: Quit

### In-Game Controls
- `↑/↓/←/→` or `k/j/h/l`: Move cursor
- `Enter` or `Space`: Select piece / Make move
- `Esc`: Deselect piece / Cancel promotion
- `m`: Return to main menu
- `q`: Quit game

### Pawn Promotion
When a pawn reaches the opposite end:
- `Q`: Promote to Queen
- `R`: Promote to Rook
- `B`: Promote to Bishop
- `N`: Promote to Knight

## How to Play

1. Launch the game and select a game mode from the menu
2. Use arrow keys to move the cursor to a piece of your color
3. Press Enter to select the piece - legal moves will be shown as green dots
4. Move the cursor to a highlighted square and press Enter to make the move
5. The game will automatically detect check, checkmate, and stalemate

## AI Difficulty

The AI uses a minimax algorithm with alpha-beta pruning at depth 3, providing a challenging opponent suitable for intermediate players. The AI evaluates positions based on:
- Material value
- Piece positioning
- Mobility
- King safety

## Technical Details

- **Language**: Rust
- **TUI Library**: ratatui 0.29
- **Terminal Backend**: crossterm 0.28
- **Architecture**: Modular design with separate game logic, AI, and UI layers

## License

See LICENSE file for details.
