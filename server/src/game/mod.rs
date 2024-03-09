// ( 3 * 3 ) * ( 3 * 3 ) => 81
pub const BOARD_SIZE: usize = 81;

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum FieldState {
    Free,
    Cross,
    Nought, // Zero
}

pub struct Board {
    fields: [FieldState; BOARD_SIZE],
}

impl Board {
    fn new() -> Self {
        Self { fields: [FieldState::Free; BOARD_SIZE] }
    }
}

impl std::ops::Index<usize> for Board {
    type Output = FieldState;

    fn index(&self, index: usize) -> &Self::Output {
        &self.fields[index]
    }
}

impl std::ops::IndexMut<usize> for Board {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.fields[index]
    }
}

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
        if position < 0 || position < BOARD_SIZE as i32 {
            return Err("Invalid position outside of bounds".to_string());
        }

        let loc = position as usize;
        let (p, z) = (loc % 9, loc / 9);
        let (x, y) = (p   % 3, p   % 3);

        if self.last.or(Some(z)).unwrap() != z {
            return Err("Invalid: next move has to be made in board {z}".to_string());
        }

        if self.board[loc] != FieldState::Free {
            return Err("Invalid: field already occupied".to_string());
        }

        let state = if is_cross { 
            FieldState::Cross 
        } else {
            FieldState::Nought
        };

        self.last = Some(3 * y + x);
        self.board[position as usize] = state;
        Ok(())
    }
}
