pub mod board;
use board::{Board, FieldState};

pub struct Game {
    board: Board,
    last: Option<usize>,
}

impl Game {
    pub fn new() -> Self {
        Self {
            last: None,
            board: Board::new(),
        }
    }

    pub fn set(&mut self, is_cross: bool, position: i32) -> Result<(), String> {
        if !self.board.is_valid(position) {
            return Err("ERROR: position outside of bounds".to_string());
        }

        let loc = position as usize;
        let (p, z) = (loc % 9, loc / 9);
        let (x, y) = (p   % 3, p   / 3);

        if !self.board.is_inner_board_playable(z) {
            return Err("ERROR: inner boards result is already determined".to_string());
        }

        let last = self.last.unwrap_or(z);

        if self.board.is_inner_board_playable(last) && last != z {
            return Err(format!("ERROR: next move has to be made in board {z}"));
        }

        if self.board[loc] != FieldState::Free {
            return Err("ERROR: field already occupied".to_string());
        }

        let state = if is_cross { 
            FieldState::Cross 
        } else {
            FieldState::Nought
        };

        self.last = Some(3 * y + x);
        self.board[position as usize] = state;

        if self.board.check_inner_board_wins(z, state) {
            self.board.set_inner_board_result(z, state);
        }

        if self.board.check_game_win(state) {
            println!("Someone won");
        }

        Ok(())
    }
}
